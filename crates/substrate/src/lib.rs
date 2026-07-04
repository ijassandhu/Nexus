//! NEXUS substrate v0: the episodic log with per-item encryption and
//! crypto-shredding, implementing specs/record.md v0.1 §2–§7.
//!
//! Deliberately NOT here yet (Phase 0 scope, see ROADMAP.md): derived stores,
//! retrieval, consolidation, sync. The log is ground truth; everything else
//! is rebuildable and arrives behind `nx rebuild`.

pub mod bets;
pub mod crypto;
pub mod derived;
pub mod event;
pub mod identity;
pub mod keyring;
pub mod log;
pub mod secrets;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use rand::RngCore;

use crypto::Kek;
use event::{EventHeader, Kind, Origin, Privacy, Trust};
use identity::Identity;
use keyring::Keyring;
use log::{Record, SegmentLog};

pub const FORGET_SCHEMA: &str = "memory.forget/1";

pub struct Substrate {
    dir: PathBuf,
    kek: Kek,
    keyring: Keyring,
    log: SegmentLog,
    identity: Option<Identity>,
}

/// A read-out event: header plus body state.
pub struct ReadEvent {
    pub header: EventHeader,
    pub content_address: [u8; 32],
    pub body: BodyState,
}

pub enum BodyState {
    /// Decrypted plaintext.
    Plain(Vec<u8>),
    /// Keyring entry destroyed — provably unreadable (specs/record.md §7).
    Forgotten,
    /// Not decrypted (listing without --decrypt).
    Sealed,
}

impl Substrate {
    /// Create a new record at `dir`. Fails if one already exists.
    pub fn init(dir: &Path, passphrase: &str) -> Result<Self> {
        let salt_path = dir.join("keyring").join("salt");
        if salt_path.exists() {
            bail!("a record already exists at {}", dir.display());
        }
        fs::create_dir_all(dir.join("keyring"))?;
        fs::create_dir_all(dir.join("log"))?;
        let mut salt = [0u8; 16];
        rand::thread_rng().fill_bytes(&mut salt);
        fs::write(&salt_path, salt)?;
        Self::open(dir, passphrase)
    }

    pub fn open(dir: &Path, passphrase: &str) -> Result<Self> {
        let salt = fs::read(dir.join("keyring").join("salt"))
            .context("no record here — run `nx init` first")?;
        let kek = Kek::derive(passphrase, &salt)?;
        let keyring = Keyring::open(&dir.join("keyring"))?;
        let log = SegmentLog::open(&dir.join("log"))?;
        Ok(Self {
            dir: dir.to_path_buf(),
            kek,
            keyring,
            log,
            identity: None,
        })
    }

    /// Store a user secret (e.g. a provider API key), sealed under the KEK.
    pub fn store_secret(&mut self, name: &str, value: &str) -> Result<()> {
        secrets::store(&self.dir.join("keyring"), &self.kek, name, value)
    }

    pub fn load_secret(&self, name: &str) -> Result<Option<String>> {
        secrets::load(&self.dir.join("keyring"), &self.kek, name)
    }

    pub fn delete_secret(&mut self, name: &str) -> Result<bool> {
        secrets::delete(&self.dir.join("keyring"), name)
    }

    /// The daemon's signing identity, created lazily and sealed under the
    /// KEK (records initialized before signing existed get one on first use).
    pub fn identity(&mut self) -> Result<&Identity> {
        if self.identity.is_none() {
            self.identity = Some(Identity::load_or_create(
                &self.dir.join("keyring"),
                &self.kek,
            )?);
        }
        Ok(self.identity.as_ref().unwrap())
    }

    /// Append one event: fresh DEK, encrypt body (AAD = header bytes), wrap
    /// DEK into the keyring, frame into the active segment.
    pub fn append(
        &mut self,
        kind: Kind,
        schema: &str,
        origin: Origin,
        privacy: Privacy,
        refs: Vec<String>,
        body: &[u8],
    ) -> Result<EventHeader> {
        let header = EventHeader::new(kind, schema, origin, privacy, refs, body);
        let header_bytes = header.to_bytes()?;
        let dek = crypto::new_dek();
        let envelope = crypto::encrypt_body(&dek, &header_bytes, body)?;
        let wrapped = crypto::wrap_dek(&self.kek, &dek, &header.id)?;
        // Keyring first: an event whose key was never persisted is unreadable
        // (fails safe); a key without its event is harmless garbage.
        self.keyring.insert(&header.id, wrapped)?;
        self.log.append(&header_bytes, &envelope)?;
        Ok(header)
    }

    /// Read all events, optionally decrypting bodies.
    pub fn events(&self, decrypt: bool) -> Result<(Vec<ReadEvent>, usize)> {
        let (records, torn) = self.log.read_all()?;
        let mut out = Vec::with_capacity(records.len());
        for rec in records {
            out.push(self.to_read_event(rec, decrypt)?);
        }
        Ok((out, torn))
    }

    fn to_read_event(&self, rec: Record, decrypt: bool) -> Result<ReadEvent> {
        let body = match self.keyring.get(&rec.header.id) {
            None => BodyState::Forgotten,
            Some(_) if !decrypt => BodyState::Sealed,
            Some(wrapped) => {
                let dek = crypto::unwrap_dek(&self.kek, wrapped, &rec.header.id)?;
                BodyState::Plain(crypto::decrypt_body(&dek, &rec.header_bytes, &rec.envelope)?)
            }
        };
        Ok(ReadEvent {
            header: rec.header,
            content_address: rec.content_address,
            body,
        })
    }

    /// Forget per specs/record.md §7: shred the wrapped DEK, then append a
    /// tombstone referencing id and body_hash only — never content.
    pub fn forget(&mut self, id: &str) -> Result<bool> {
        let (records, _) = self.log.read_all()?;
        let Some(target) = records.iter().find(|r| r.header.id == id) else {
            return Ok(false);
        };
        if !self.keyring.forget(id)? {
            return Ok(false); // already forgotten
        }
        let tombstone = serde_json::json!({
            "forgotten": id,
            "body_hash": hex(&target.header.body_hash),
        });
        self.append(
            Kind::System,
            FORGET_SCHEMA,
            Origin {
                adapter: "substrate".into(),
                actor: "user".into(),
                trust: Trust::User,
            },
            Privacy::P1,
            vec![id.to_string()],
            tombstone.to_string().as_bytes(),
        )?;
        Ok(true)
    }

    /// Verify checksums and keyring coverage. Returns (records, torn frames,
    /// forgotten count).
    pub fn verify(&self) -> Result<(usize, usize, usize)> {
        let (records, torn) = self.log.read_all()?; // read_all fails hard on checksum mismatch
        let forgotten = records
            .iter()
            .filter(|r| self.keyring.get(&r.header.id).is_none())
            .count();
        Ok((records.len(), torn, forgotten))
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn origin() -> Origin {
        Origin {
            adapter: "test".into(),
            actor: "tester".into(),
            trust: Trust::Local,
        }
    }

    #[test]
    fn roundtrip_append_read() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = Substrate::init(dir.path(), "pass").unwrap();
        let h = s
            .append(Kind::Observation, "dev.note/1", origin(), Privacy::P1, vec![], b"hello nexus")
            .unwrap();
        let (events, torn) = s.events(true).unwrap();
        assert_eq!(torn, 0);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].header.id, h.id);
        match &events[0].body {
            BodyState::Plain(b) => assert_eq!(b, b"hello nexus"),
            _ => panic!("expected plaintext"),
        }
    }

    #[test]
    fn wrong_passphrase_cannot_read() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = Substrate::init(dir.path(), "right").unwrap();
        s.append(Kind::Observation, "dev.note/1", origin(), Privacy::P1, vec![], b"secret")
            .unwrap();
        let s2 = Substrate::open(dir.path(), "wrong").unwrap();
        assert!(s2.events(true).is_err());
    }

    #[test]
    fn forget_shreds_and_tombstones() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = Substrate::init(dir.path(), "pass").unwrap();
        let h = s
            .append(Kind::Observation, "dev.note/1", origin(), Privacy::P2, vec![], b"to be forgotten")
            .unwrap();
        assert!(s.forget(&h.id).unwrap());

        let (events, _) = s.events(true).unwrap();
        assert_eq!(events.len(), 2); // original + tombstone
        assert!(matches!(events[0].body, BodyState::Forgotten));
        assert_eq!(events[1].header.schema, FORGET_SCHEMA);
        assert_eq!(events[1].header.refs, vec![h.id.clone()]);
        // Reopen from disk: still forgotten (shred survived persistence).
        let s2 = Substrate::open(dir.path(), "pass").unwrap();
        let (events, _) = s2.events(true).unwrap();
        assert!(matches!(events[0].body, BodyState::Forgotten));
        // Forgetting twice is a no-op, not an error.
        let mut s2 = s2;
        assert!(!s2.forget(&h.id).unwrap());
    }

    #[test]
    fn tamper_is_detected() {
        let dir = tempfile::tempdir().unwrap();
        let mut s = Substrate::init(dir.path(), "pass").unwrap();
        s.append(Kind::Observation, "dev.note/1", origin(), Privacy::P1, vec![], b"immutable")
            .unwrap();
        // Flip a byte mid-file (inside the frame, past the 5-byte segment header).
        let seg = std::fs::read_dir(dir.path().join("log"))
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        let mut data = std::fs::read(&seg).unwrap();
        let mid = data.len() / 2;
        data[mid] ^= 0xFF;
        std::fs::write(&seg, data).unwrap();
        assert!(s.events(false).is_err());
    }
}
