//! Encryption envelope and key hierarchy per specs/record.md §5.
//!
//! Per-item DEK (XChaCha20-Poly1305), AAD = serialized event header,
//! DEK wrapped by the user's KEK (Argon2id-derived). Destroying a wrapped
//! DEK is forgetting (crypto-shredding, D-011).

use anyhow::{anyhow, Context, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    Key, XChaCha20Poly1305, XNonce,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};

pub const ALG: &str = "xchacha20poly1305/1";

/// Root key-encryption key. Held in memory only; derived per session.
pub struct Kek([u8; 32]);

impl Kek {
    pub fn derive(passphrase: &str, salt: &[u8]) -> Result<Self> {
        let mut out = [0u8; 32];
        argon2::Argon2::default()
            .hash_password_into(passphrase.as_bytes(), salt, &mut out)
            .map_err(|e| anyhow!("argon2 key derivation failed: {e}"))?;
        Ok(Self(out))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope {
    pub alg: String,
    #[serde(with = "serde_bytes")]
    pub nonce: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub ct: Vec<u8>,
}

fn random_bytes<const N: usize>() -> [u8; N] {
    let mut b = [0u8; N];
    rand::thread_rng().fill_bytes(&mut b);
    b
}

pub fn new_dek() -> [u8; 32] {
    random_bytes::<32>()
}

fn seal(key: &[u8; 32], aad: &[u8], plaintext: &[u8]) -> Result<Envelope> {
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let nonce_bytes = random_bytes::<24>();
    let nonce = XNonce::from_slice(&nonce_bytes);
    let ct = cipher
        .encrypt(nonce, Payload { msg: plaintext, aad })
        .map_err(|_| anyhow!("encryption failed"))?;
    Ok(Envelope {
        alg: ALG.to_string(),
        nonce: nonce_bytes.to_vec(),
        ct,
    })
}

fn open(key: &[u8; 32], aad: &[u8], env: &Envelope) -> Result<Vec<u8>> {
    if env.alg != ALG {
        return Err(anyhow!("unknown envelope algorithm: {}", env.alg));
    }
    let cipher = XChaCha20Poly1305::new(Key::from_slice(key));
    let nonce = XNonce::from_slice(&env.nonce);
    cipher
        .decrypt(nonce, Payload { msg: &env.ct, aad })
        .map_err(|_| anyhow!("decryption failed (wrong key, tamper, or forgotten item)"))
}

/// Encrypt an event body under a fresh DEK. AAD is the header bytes.
pub fn encrypt_body(dek: &[u8; 32], header_bytes: &[u8], body: &[u8]) -> Result<Envelope> {
    seal(dek, header_bytes, body)
}

pub fn decrypt_body(dek: &[u8; 32], header_bytes: &[u8], env: &Envelope) -> Result<Vec<u8>> {
    open(dek, header_bytes, env).context("body decrypt")
}

/// Wrap a DEK under the KEK; AAD is the event id so a wrapped key cannot be
/// replayed onto a different event.
pub fn wrap_dek(kek: &Kek, dek: &[u8; 32], event_id: &str) -> Result<Envelope> {
    seal(&kek.0, event_id.as_bytes(), dek)
}

/// Seal arbitrary key material under the KEK with a purpose label as AAD
/// (e.g. the daemon identity key). Not for event bodies — those go through
/// per-item DEKs.
pub fn seal_with_kek(kek: &Kek, purpose: &str, plaintext: &[u8]) -> Result<Envelope> {
    seal(&kek.0, purpose.as_bytes(), plaintext)
}

pub fn open_with_kek(kek: &Kek, purpose: &str, env: &Envelope) -> Result<Vec<u8>> {
    open(&kek.0, purpose.as_bytes(), env).context("KEK-sealed material")
}

pub fn unwrap_dek(kek: &Kek, wrapped: &Envelope, event_id: &str) -> Result<[u8; 32]> {
    let bytes = open(&kek.0, event_id.as_bytes(), wrapped).context("DEK unwrap")?;
    let arr: [u8; 32] = bytes
        .try_into()
        .map_err(|_| anyhow!("unwrapped DEK has wrong length"))?;
    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_and_aad_binding() {
        let dek = new_dek();
        let env = encrypt_body(&dek, b"header", b"secret body").unwrap();
        assert_eq!(decrypt_body(&dek, b"header", &env).unwrap(), b"secret body");
        // Swapped header (AAD) must fail: header/body binding.
        assert!(decrypt_body(&dek, b"other-header", &env).is_err());
    }

    #[test]
    fn wrap_binds_to_event_id() {
        let kek = Kek::derive("pass", b"0123456789abcdef").unwrap();
        let dek = new_dek();
        let wrapped = wrap_dek(&kek, &dek, "01ABC").unwrap();
        assert_eq!(unwrap_dek(&kek, &wrapped, "01ABC").unwrap(), dek);
        assert!(unwrap_dek(&kek, &wrapped, "01XYZ").is_err());
    }
}
