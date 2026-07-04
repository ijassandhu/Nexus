//! Sealed user secrets (provider API keys, tokens): encrypted under the KEK
//! with a per-secret purpose label as AAD, stored beside the keyring. Same
//! root of trust as the record itself — the passphrase. Never plaintext on
//! disk, never in the log, never hardcoded.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::crypto::{self, Envelope, Kek};

const FILE: &str = "secrets.json";

fn path(keyring_dir: &Path) -> PathBuf {
    keyring_dir.join(FILE)
}

fn load_map(keyring_dir: &Path) -> Result<BTreeMap<String, Envelope>> {
    let p = path(keyring_dir);
    if !p.exists() {
        return Ok(BTreeMap::new());
    }
    serde_json::from_str(&fs::read_to_string(&p)?).context("corrupt secrets file")
}

fn save_map(keyring_dir: &Path, map: &BTreeMap<String, Envelope>) -> Result<()> {
    fs::write(path(keyring_dir), serde_json::to_string_pretty(map)?)?;
    Ok(())
}

pub fn store(keyring_dir: &Path, kek: &Kek, name: &str, value: &str) -> Result<()> {
    let mut map = load_map(keyring_dir)?;
    let env = crypto::seal_with_kek(kek, &format!("secret/{name}"), value.as_bytes())?;
    map.insert(name.to_string(), env);
    save_map(keyring_dir, &map)
}

pub fn load(keyring_dir: &Path, kek: &Kek, name: &str) -> Result<Option<String>> {
    let map = load_map(keyring_dir)?;
    match map.get(name) {
        None => Ok(None),
        Some(env) => {
            let bytes = crypto::open_with_kek(kek, &format!("secret/{name}"), env)
                .with_context(|| format!("secret '{name}' cannot be unsealed"))?;
            Ok(Some(String::from_utf8(bytes)?))
        }
    }
}

pub fn names(keyring_dir: &Path) -> Result<Vec<String>> {
    Ok(load_map(keyring_dir)?.keys().cloned().collect())
}

pub fn delete(keyring_dir: &Path, name: &str) -> Result<bool> {
    let mut map = load_map(keyring_dir)?;
    let existed = map.remove(name).is_some();
    if existed {
        save_map(keyring_dir, &map)?;
    }
    Ok(existed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_wrong_kek_refused() {
        let dir = tempfile::tempdir().unwrap();
        let kek = Kek::derive("pass", b"0123456789abcdef").unwrap();
        store(dir.path(), &kek, "provider-key/openai", "sk-test-123").unwrap();
        assert_eq!(
            load(dir.path(), &kek, "provider-key/openai").unwrap().as_deref(),
            Some("sk-test-123")
        );
        assert_eq!(load(dir.path(), &kek, "missing").unwrap(), None);
        // Key material is not plaintext on disk.
        let raw = std::fs::read_to_string(dir.path().join("secrets.json")).unwrap();
        assert!(!raw.contains("sk-test-123"));
        // Wrong passphrase cannot unseal.
        let wrong = Kek::derive("wrong", b"0123456789abcdef").unwrap();
        assert!(load(dir.path(), &wrong, "provider-key/openai").is_err());
        // Delete works.
        assert!(delete(dir.path(), "provider-key/openai").unwrap());
        assert_eq!(load(dir.path(), &kek, "provider-key/openai").unwrap(), None);
    }
}
