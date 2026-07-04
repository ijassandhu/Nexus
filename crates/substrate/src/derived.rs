//! Derived store v0: a SQLite index over the episodic log, rebuilt from the
//! log at any time (`nx rebuild`) — the escape hatch that makes every
//! derived-store schema decision reversible (D-013, ARCHITECTURE §4).
//!
//! v0 indexes event *headers only* (already plaintext-at-rest as AEAD
//! associated data in the frames). Bodies are deliberately NOT indexed:
//! full-text search over decrypted content would put plaintext at rest
//! outside the envelope model, so FTS waits for a derived-store encryption
//! story (Phase 2, tracked in PROGRESS.md).

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use rusqlite::{params, Connection};

use crate::{hex, BodyState, Substrate};

// Rebuild happens inside SQLite (drop + recreate) rather than by deleting the
// db file: Windows refuses to delete a file any other handle has open, and a
// rebuild must work while a reader (future inbox UI) holds the index.
const SCHEMA: &str = "
DROP TABLE IF EXISTS meta;
DROP TABLE IF EXISTS events;
DROP TABLE IF EXISTS refs;
DROP TABLE IF EXISTS bets;
CREATE TABLE bets (
    id           TEXT PRIMARY KEY,
    created      TEXT NOT NULL,
    kind         TEXT NOT NULL,
    stakes       TEXT NOT NULL,
    status       TEXT NOT NULL,
    held         INTEGER NOT NULL,
    falsified    INTEGER NOT NULL,
    unjustified  INTEGER NOT NULL,
    statement    TEXT NOT NULL
);
CREATE TABLE meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE events (
    seq             INTEGER PRIMARY KEY,
    id              TEXT NOT NULL UNIQUE,
    ts              TEXT NOT NULL,
    kind            TEXT NOT NULL,
    schema          TEXT NOT NULL,
    adapter         TEXT NOT NULL,
    actor           TEXT NOT NULL,
    trust           TEXT NOT NULL,
    privacy         TEXT NOT NULL,
    body_hash       TEXT NOT NULL,
    content_address TEXT NOT NULL,
    forgotten       INTEGER NOT NULL
);
CREATE INDEX idx_events_schema ON events(schema);
CREATE INDEX idx_events_trust  ON events(trust);
CREATE INDEX idx_events_ts     ON events(ts);
CREATE TABLE refs (
    event_id TEXT NOT NULL,
    ref_id   TEXT NOT NULL
);
CREATE INDEX idx_refs_ref ON refs(ref_id);
";

#[derive(Debug)]
pub struct IndexedEvent {
    pub seq: i64,
    pub id: String,
    pub ts: String,
    pub kind: String,
    pub schema: String,
    pub trust: String,
    pub privacy: String,
    pub forgotten: bool,
}

#[derive(Default, Debug)]
pub struct QueryFilter {
    /// SQL LIKE pattern, e.g. "txn.%"
    pub schema: Option<String>,
    pub trust: Option<String>,
    pub kind: Option<String>,
    /// Only events referencing this event id.
    pub references: Option<String>,
    pub limit: Option<u32>,
}

pub struct DerivedStore {
    conn: Connection,
    pub path: PathBuf,
}

impl DerivedStore {
    fn db_path(data_dir: &Path) -> PathBuf {
        data_dir.join("derived").join("index.db")
    }

    /// Drop and re-derive the entire index from the log. Idempotent: same
    /// log → same index. Returns (event rows, ref rows).
    pub fn rebuild(data_dir: &Path, sub: &Substrate) -> Result<(Self, usize, usize)> {
        let path = Self::db_path(data_dir);
        fs::create_dir_all(path.parent().unwrap())?;
        let mut conn = Connection::open(&path)?;
        conn.execute_batch(SCHEMA)?;

        let (events, _torn) = sub.events(false)?;
        let mut n_refs = 0usize;
        let tx = conn.transaction()?;
        {
            let mut ins_event = tx.prepare(
                "INSERT INTO events (seq,id,ts,kind,schema,adapter,actor,trust,privacy,body_hash,content_address,forgotten)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
            )?;
            let mut ins_ref = tx.prepare("INSERT INTO refs (event_id, ref_id) VALUES (?1, ?2)")?;
            for (seq, e) in events.iter().enumerate() {
                let forgotten = matches!(e.body, BodyState::Forgotten);
                ins_event.execute(params![
                    seq as i64,
                    e.header.id,
                    e.header.ts.to_rfc3339(),
                    e.header.kind.as_str(),
                    e.header.schema,
                    e.header.origin.adapter,
                    e.header.origin.actor,
                    e.header.origin.trust.as_str(),
                    e.header.privacy.as_str(),
                    hex(&e.header.body_hash),
                    hex(&e.content_address),
                    forgotten as i64,
                ])?;
                for r in &e.header.refs {
                    ins_ref.execute(params![e.header.id, r])?;
                    n_refs += 1;
                }
            }
            let mut ins_bet = tx.prepare(
                "INSERT INTO bets (id,created,kind,stakes,status,held,falsified,unjustified,statement)
                 VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            )?;
            for v in crate::bets::views(sub)? {
                ins_bet.execute(params![
                    v.id,
                    v.created.to_rfc3339(),
                    format!("{:?}", v.bet.kind).to_lowercase(),
                    v.bet.stakes,
                    v.status,
                    v.held as i64,
                    v.falsified_count as i64,
                    v.unjustified as i64,
                    v.bet.statement,
                ])?;
            }
            let mut ins_meta = tx.prepare("INSERT INTO meta (key, value) VALUES (?1, ?2)")?;
            ins_meta.execute(params!["record_spec", "0.2"])?;
            ins_meta.execute(params!["event_count", events.len().to_string()])?;
        }
        tx.commit()?;
        let n = events.len();
        Ok((Self { conn, path }, n, n_refs))
    }

    pub fn open(data_dir: &Path) -> Result<Self> {
        let path = Self::db_path(data_dir);
        anyhow::ensure!(path.exists(), "no derived index — run `nx rebuild` first");
        Ok(Self {
            conn: Connection::open(&path)?,
            path,
        })
    }

    pub fn query(&self, f: &QueryFilter) -> Result<Vec<IndexedEvent>> {
        let mut sql = String::from(
            "SELECT e.seq, e.id, e.ts, e.kind, e.schema, e.trust, e.privacy, e.forgotten
             FROM events e WHERE 1=1",
        );
        let mut args: Vec<String> = Vec::new();
        if let Some(s) = &f.schema {
            sql.push_str(" AND e.schema LIKE ?");
            args.push(s.clone());
        }
        if let Some(t) = &f.trust {
            sql.push_str(" AND e.trust = ?");
            args.push(t.clone());
        }
        if let Some(k) = &f.kind {
            sql.push_str(" AND e.kind = ?");
            args.push(k.clone());
        }
        if let Some(r) = &f.references {
            sql.push_str(" AND e.id IN (SELECT event_id FROM refs WHERE ref_id = ?)");
            args.push(r.clone());
        }
        sql.push_str(" ORDER BY e.seq");
        if let Some(l) = f.limit {
            sql.push_str(&format!(" LIMIT {l}"));
        }
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(args.iter()), |row| {
            Ok(IndexedEvent {
                seq: row.get(0)?,
                id: row.get(1)?,
                ts: row.get(2)?,
                kind: row.get(3)?,
                schema: row.get(4)?,
                trust: row.get(5)?,
                privacy: row.get(6)?,
                forgotten: row.get::<_, i64>(7)? != 0,
            })
        })?;
        Ok(rows.collect::<std::result::Result<Vec<_>, _>>()?)
    }

    /// (total events, forgotten, distinct schemas)
    pub fn stats(&self) -> Result<(i64, i64, i64)> {
        let total: i64 = self.conn.query_row("SELECT COUNT(*) FROM events", [], |r| r.get(0))?;
        let forgotten: i64 =
            self.conn
                .query_row("SELECT COUNT(*) FROM events WHERE forgotten=1", [], |r| r.get(0))?;
        let schemas: i64 =
            self.conn
                .query_row("SELECT COUNT(DISTINCT schema) FROM events", [], |r| r.get(0))?;
        Ok((total, forgotten, schemas))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Kind, Origin, Privacy, Trust};

    fn origin(trust: Trust) -> Origin {
        Origin {
            adapter: "test".into(),
            actor: "tester".into(),
            trust,
        }
    }

    #[test]
    fn rebuild_indexes_log_and_reflects_forgetting() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let a = sub
            .append(Kind::Observation, "dev.note/1", origin(Trust::User), Privacy::P1, vec![], b"one")
            .unwrap();
        sub.append(Kind::Observation, "web.page/1", origin(Trust::External), Privacy::P2, vec![], b"two")
            .unwrap();
        sub.append(Kind::Action, "txn.begin/1", origin(Trust::Local), Privacy::P1, vec![a.id.clone()], b"three")
            .unwrap();

        let (store, n, n_refs) = DerivedStore::rebuild(dir.path(), &sub).unwrap();
        assert_eq!(n, 3);
        assert_eq!(n_refs, 1);

        // Filters work.
        let ext = store
            .query(&QueryFilter { trust: Some("external".into()), ..Default::default() })
            .unwrap();
        assert_eq!(ext.len(), 1);
        assert_eq!(ext[0].schema, "web.page/1");
        let referring = store
            .query(&QueryFilter { references: Some(a.id.clone()), ..Default::default() })
            .unwrap();
        assert_eq!(referring.len(), 1);
        assert_eq!(referring[0].schema, "txn.begin/1");

        // Forget, rebuild: index reflects it (forgotten flag + tombstone row).
        sub.forget(&a.id).unwrap();
        let (store, n, _) = DerivedStore::rebuild(dir.path(), &sub).unwrap();
        assert_eq!(n, 4); // + tombstone event
        let rows = store
            .query(&QueryFilter { schema: Some("dev.note/1".into()), ..Default::default() })
            .unwrap();
        assert!(rows[0].forgotten);
        let (total, forgotten, _) = store.stats().unwrap();
        assert_eq!((total, forgotten), (4, 1));
    }

    #[test]
    fn rebuild_is_idempotent() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        sub.append(Kind::Observation, "dev.note/1", origin(Trust::User), Privacy::P1, vec![], b"x")
            .unwrap();
        let (s1, n1, _) = DerivedStore::rebuild(dir.path(), &sub).unwrap();
        let stats1 = s1.stats().unwrap();
        drop(s1);
        let (s2, n2, _) = DerivedStore::rebuild(dir.path(), &sub).unwrap();
        assert_eq!(n1, n2);
        assert_eq!(stats1.0, s2.stats().unwrap().0);
    }
}
