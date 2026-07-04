//! Daemon identity: an Ed25519 keypair that signs ledger entries, making the
//! flight recorder tamper-evident (specs/capability.md §6).
//!
//! The signing key is generated lazily on first use, sealed under the user's
//! KEK (AAD = purpose label), and stored beside the keyring. Signing key and
//! record therefore share one root of trust: the passphrase.

use std::fs;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::RngCore;

use crate::crypto::{self, Envelope, Kek};

const PURPOSE: &str = "daemon-identity/ed25519";
const FILE: &str = "identity.env";

pub struct Identity {
    signing: SigningKey,
}

impl Identity {
    /// Load the identity, creating and sealing a fresh one if absent.
    pub fn load_or_create(keyring_dir: &Path, kek: &Kek) -> Result<Self> {
        let path = keyring_dir.join(FILE);
        let secret: [u8; 32] = if path.exists() {
            let env: Envelope = serde_json::from_str(&fs::read_to_string(&path)?)
                .context("corrupt identity envelope")?;
            crypto::open_with_kek(kek, PURPOSE, &env)?
                .try_into()
                .map_err(|_| anyhow!("identity key has wrong length"))?
        } else {
            let mut bytes = [0u8; 32];
            rand::thread_rng().fill_bytes(&mut bytes);
            let env = crypto::seal_with_kek(kek, PURPOSE, &bytes)?;
            fs::write(&path, serde_json::to_string(&env)?)?;
            bytes
        };
        Ok(Self {
            signing: SigningKey::from_bytes(&secret),
        })
    }

    pub fn sign(&self, msg: &[u8]) -> [u8; 64] {
        self.signing.sign(msg).to_bytes()
    }

    pub fn public_key_hex(&self) -> String {
        crate::hex(self.signing.verifying_key().as_bytes())
    }

    /// Verify a signature against this daemon's own public key.
    pub fn verify(&self, msg: &[u8], sig: &[u8]) -> bool {
        let Ok(sig64): std::result::Result<[u8; 64], _> = sig.try_into() else {
            return false;
        };
        self.signing
            .verifying_key()
            .verify(msg, &Signature::from_bytes(&sig64))
            .is_ok()
    }
}

/// Verify against an arbitrary public key (hex) — used when auditing a
/// record whose signer key is recorded in the entries themselves.
pub fn verify_with(public_key_hex: &str, msg: &[u8], sig: &[u8]) -> bool {
    let Ok(pk_bytes) = from_hex(public_key_hex) else {
        return false;
    };
    let Ok(pk_arr): std::result::Result<[u8; 32], _> = pk_bytes.try_into() else {
        return false;
    };
    let Ok(pk) = VerifyingKey::from_bytes(&pk_arr) else {
        return false;
    };
    let Ok(sig64): std::result::Result<[u8; 64], _> = sig.try_into() else {
        return false;
    };
    pk.verify(msg, &Signature::from_bytes(&sig64)).is_ok()
}

fn from_hex(s: &str) -> Result<Vec<u8>> {
    if s.len() % 2 != 0 {
        return Err(anyhow!("odd hex length"));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| anyhow!(e)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_verify_roundtrip_and_persistence() {
        let dir = tempfile::tempdir().unwrap();
        let kek = Kek::derive("pass", b"0123456789abcdef").unwrap();
        let id1 = Identity::load_or_create(dir.path(), &kek).unwrap();
        let sig = id1.sign(b"ledger entry");
        assert!(id1.verify(b"ledger entry", &sig));
        assert!(!id1.verify(b"tampered entry", &sig));
        // Reload from disk: same key.
        let id2 = Identity::load_or_create(dir.path(), &kek).unwrap();
        assert_eq!(id1.public_key_hex(), id2.public_key_hex());
        assert!(id2.verify(b"ledger entry", &sig));
        // External verification path.
        assert!(verify_with(&id1.public_key_hex(), b"ledger entry", &sig));
        assert!(!verify_with(&id1.public_key_hex(), b"other", &sig));
    }

    #[test]
    fn wrong_kek_cannot_load_identity() {
        let dir = tempfile::tempdir().unwrap();
        let kek = Kek::derive("right", b"0123456789abcdef").unwrap();
        Identity::load_or_create(dir.path(), &kek).unwrap();
        let wrong = Kek::derive("wrong", b"0123456789abcdef").unwrap();
        assert!(Identity::load_or_create(dir.path(), &wrong).is_err());
    }
}
