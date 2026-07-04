//! Keyring: event id → wrapped DEK, stored separately from segments so that
//! destroying an entry kills every copy of the ciphertext (specs/record.md §7).
//!
//! v0 storage is a JSONL file rewritten on forget. Fine at personal scale;
//! migrates behind this interface later (D-013 escape hatch: derived data is
//! rebuildable, keyring is NOT derived — it is primary and must be backed up).

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::crypto::Envelope;

#[derive(Serialize, Deserialize)]
struct Entry {
    id: String,
    wrapped: Envelope,
}

pub struct Keyring {
    path: PathBuf,
    entries: BTreeMap<String, Envelope>,
}

impl Keyring {
    pub fn open(dir: &Path) -> Result<Self> {
        let path = dir.join("keys.jsonl");
        let mut entries = BTreeMap::new();
        if path.exists() {
            for line in fs::read_to_string(&path)?.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                let e: Entry = serde_json::from_str(line).context("corrupt keyring line")?;
                entries.insert(e.id, e.wrapped);
            }
        }
        Ok(Self { path, entries })
    }

    pub fn insert(&mut self, id: &str, wrapped: Envelope) -> Result<()> {
        let line = serde_json::to_string(&Entry {
            id: id.to_string(),
            wrapped: wrapped.clone(),
        })?;
        let mut f = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)?;
        writeln!(f, "{line}")?;
        f.sync_all()?;
        self.entries.insert(id.to_string(), wrapped);
        Ok(())
    }

    pub fn get(&self, id: &str) -> Option<&Envelope> {
        self.entries.get(id)
    }

    /// Crypto-shred: remove the wrapped DEK and rewrite the file so the wrapped
    /// key bytes are gone from the primary store. Returns false if absent.
    pub fn forget(&mut self, id: &str) -> Result<bool> {
        if self.entries.remove(id).is_none() {
            return Ok(false);
        }
        let tmp = self.path.with_extension("jsonl.tmp");
        {
            let mut f = fs::File::create(&tmp)?;
            for (id, wrapped) in &self.entries {
                let line = serde_json::to_string(&Entry {
                    id: id.clone(),
                    wrapped: wrapped.clone(),
                })?;
                writeln!(f, "{line}")?;
            }
            f.sync_all()?;
        }
        fs::rename(&tmp, &self.path)?;
        Ok(true)
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
