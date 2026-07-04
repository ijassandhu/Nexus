//! Event schema per specs/record.md §2–§4.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Observation,
    Action,
    Feedback,
    System,
}

/// Trust levels, declared lowest-to-highest so that derived `Ord` gives
/// `External < Derived < Local < User`. The capability ceiling keys on the
/// *minimum* trust present in a context (specs/capability.md §5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Trust {
    External,
    Derived,
    Local,
    User,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Privacy {
    P0,
    P1,
    P2,
    P3,
}

impl Kind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Kind::Observation => "observation",
            Kind::Action => "action",
            Kind::Feedback => "feedback",
            Kind::System => "system",
        }
    }
}

impl Trust {
    pub fn as_str(&self) -> &'static str {
        match self {
            Trust::External => "external",
            Trust::Derived => "derived",
            Trust::Local => "local",
            Trust::User => "user",
        }
    }
}

impl Privacy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Privacy::P0 => "P0",
            Privacy::P1 => "P1",
            Privacy::P2 => "P2",
            Privacy::P3 => "P3",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Origin {
    pub adapter: String,
    pub actor: String,
    pub trust: Trust,
}

/// Everything except the body. Serialized bytes of this struct are the AEAD
/// associated data, binding header to payload (specs/record.md §5).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventHeader {
    pub id: String,
    pub ts: DateTime<Utc>,
    pub kind: Kind,
    pub schema: String,
    pub origin: Origin,
    pub privacy: Privacy,
    pub refs: Vec<String>,
    /// BLAKE3 of the plaintext body — stable plaintext identity (§2 rationale).
    #[serde(with = "serde_bytes")]
    pub body_hash: Vec<u8>,
}

impl EventHeader {
    pub fn new(
        kind: Kind,
        schema: impl Into<String>,
        origin: Origin,
        privacy: Privacy,
        refs: Vec<String>,
        body: &[u8],
    ) -> Self {
        Self {
            id: ulid::Ulid::new().to_string(),
            ts: Utc::now(),
            kind,
            schema: schema.into(),
            origin,
            privacy,
            refs,
            body_hash: blake3::hash(body).as_bytes().to_vec(),
        }
    }

    /// Canonical header bytes used as AAD. CBOR via ciborium; the exact bytes
    /// written into the frame are reused verbatim on read, so encoder
    /// determinism across versions is not load-bearing (spec open question:
    /// deterministic-CBOR profile pin lands in 0.2).
    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        ciborium::ser::into_writer(self, &mut buf)?;
        Ok(buf)
    }

    pub fn from_bytes(bytes: &[u8]) -> anyhow::Result<Self> {
        Ok(ciborium::de::from_reader(bytes)?)
    }
}
