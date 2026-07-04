//! Ledger entry signing and audit (specs/capability.md §6).
//!
//! Kernel-emitted ledger bodies are JSON with alphabetically-ordered keys
//! (serde_json's default map). The signature covers the canonical body
//! *without* the `sig`/`signer` fields; the signer's public key rides in the
//! entry so a record is auditable from its own contents.

use anyhow::Result;
use base64::engine::general_purpose::STANDARD as B64;
use base64::Engine;
use serde_json::Value;
use substrate::Substrate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Valid,
    Invalid,
    Unsigned,
}

/// Sign a ledger body: canonicalize, sign, attach `sig` + `signer`.
pub fn sign_body(sub: &mut Substrate, mut body: Value) -> Result<Value> {
    let canonical = body.to_string();
    let identity = sub.identity()?;
    let sig = identity.sign(canonical.as_bytes());
    body["sig"] = Value::String(B64.encode(sig));
    body["signer"] = Value::String(identity.public_key_hex());
    Ok(body)
}

/// Verify a ledger body string against the signer key recorded inside it.
pub fn verify_body(body_str: &str) -> Verdict {
    let Ok(mut v) = serde_json::from_str::<Value>(body_str) else {
        return Verdict::Invalid;
    };
    let sig_b64 = v.get("sig").and_then(|s| s.as_str()).map(String::from);
    let signer = v.get("signer").and_then(|s| s.as_str()).map(String::from);
    let (Some(sig_b64), Some(signer)) = (sig_b64, signer) else {
        return Verdict::Unsigned;
    };
    let Ok(sig) = B64.decode(sig_b64) else {
        return Verdict::Invalid;
    };
    let Some(obj) = v.as_object_mut() else {
        return Verdict::Invalid;
    };
    obj.remove("sig");
    obj.remove("signer");
    let canonical = v.to_string();
    if substrate::identity::verify_with(&signer, canonical.as_bytes(), &sig) {
        Verdict::Valid
    } else {
        Verdict::Invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sign_then_verify_and_detect_tamper() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let body = serde_json::json!({"op": "write", "path": "a.txt", "txn": "T1"});
        let signed = sign_body(&mut sub, body).unwrap();
        let s = signed.to_string();
        assert_eq!(verify_body(&s), Verdict::Valid);

        // Any field tampered → invalid.
        let tampered = s.replace("a.txt", "b.txt");
        assert_eq!(verify_body(&tampered), Verdict::Invalid);

        // No sig → unsigned.
        assert_eq!(verify_body(r#"{"op":"write"}"#), Verdict::Unsigned);
    }
}
