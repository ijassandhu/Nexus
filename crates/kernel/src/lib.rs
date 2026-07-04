//! NEXUS trust kernel v0: capability table with mint / attenuate / revoke /
//! check, the R0–R3 effect-class taxonomy, and the context-trust ceiling
//! (specs/capability.md v0.1 §1–§5, D-004, D-014).
//!
//! Phase 0 scope: in-memory table + rules, exercised by tests and the CLI.
//! Transactions, the effect boundary, and ledger signing land next (the
//! ledger's entry format is specified; signing needs the daemon identity key,
//! which belongs to daemon bring-up, not the walking skeleton).

pub mod capability;
pub mod ledger;
pub mod txn;

pub use capability::*;
