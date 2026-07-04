//! Model router v0 (D-016, D-017): a static, user-editable policy table
//! mapping task class → provider/model, with outcome logging on every call —
//! the flywheel's intake, so learned routing has data when it arrives.
//!
//! The router knows providers only through the [`crate::providers`] registry
//! (Principle 5): switching vendors is a `router.json` edit or `nx configure`,
//! never an application-logic change. API keys resolve from the environment
//! (operator override) or the sealed secrets store — never from code.
//!
//! Outcome events log task class, provider, model, latency, and an output
//! hash — never prompt or completion content.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::Instant;

use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use substrate::event::{Kind, Origin, Privacy, Trust};
use substrate::Substrate;

use crate::providers::{self, CompletionRequest};

pub const ROUTER_CALL_SCHEMA: &str = "router.call/1";

/// Name under which a provider's key lives in the sealed secrets store.
pub fn secret_name(provider: &str) -> String {
    format!("provider-key/{provider}")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteRule {
    /// Provider id from the registry. (`backend` accepted for old configs.)
    #[serde(alias = "backend")]
    pub provider: String,
    pub model: String,
    pub max_tokens: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouterConfig {
    pub routes: BTreeMap<String, RouteRule>,
}

#[derive(Debug)]
pub struct Completion {
    pub text: String,
    pub model_id: String,
    pub ms: u128,
}

/// Task classes every route table must cover.
pub const TASK_CLASSES: [&str; 3] = ["worker.execute", "critic.review", "archivist.appraise"];

fn default_config() -> RouterConfig {
    // Deliberately unusable: a fresh record must be configured, not silently
    // routed to a vendor the user never chose.
    let mut routes = BTreeMap::new();
    for tc in TASK_CLASSES {
        routes.insert(
            tc.to_string(),
            RouteRule {
                provider: "unconfigured".into(),
                model: String::new(),
                max_tokens: if tc == "worker.execute" { 8192 } else { 4096 },
                base_url: None,
            },
        );
    }
    RouterConfig { routes }
}

pub struct Router {
    config: RouterConfig,
    /// Force every call to the mock provider (offline runs, tests).
    pub force_mock: bool,
}

impl Router {
    /// Load `<data>/router.json`, writing the unconfigured skeleton if absent.
    pub fn load(data_dir: &Path, force_mock: bool) -> Result<Self> {
        let path = data_dir.join("router.json");
        let config = if path.exists() {
            serde_json::from_str(&fs::read_to_string(&path)?).context("corrupt router.json")?
        } else {
            let c = default_config();
            fs::write(&path, serde_json::to_string_pretty(&c)?)?;
            c
        };
        Ok(Self { config, force_mock })
    }

    /// Resolve a provider's API key: environment variable (operator
    /// override) first, then the sealed secrets store.
    fn resolve_key(
        sub: &Substrate,
        provider: &dyn providers::Provider,
    ) -> Result<Option<String>> {
        if let Some(var) = provider.env_key() {
            if let Ok(v) = std::env::var(var) {
                if !v.is_empty() {
                    return Ok(Some(v));
                }
            }
        }
        if let Some(v) = sub.load_secret(&secret_name(provider.id()))? {
            return Ok(Some(v));
        }
        if provider.needs_key() {
            bail!(
                "no API key for provider '{}' — run `nx configure`{}",
                provider.id(),
                provider
                    .env_key()
                    .map(|v| format!(" or set {v}"))
                    .unwrap_or_default()
            );
        }
        Ok(None)
    }

    pub fn complete(
        &self,
        sub: &mut Substrate,
        task_class: &str,
        system: &str,
        user: &str,
    ) -> Result<Completion> {
        let rule = self
            .config
            .routes
            .get(task_class)
            .ok_or_else(|| anyhow!("no route for task class '{task_class}' in router.json"))?;
        let provider_id = if self.force_mock { "mock" } else { rule.provider.as_str() };
        if provider_id == "unconfigured" {
            bail!("no model provider configured — run `nx configure` to choose one");
        }
        let provider = providers::get(provider_id)?;
        let key = Self::resolve_key(sub, provider.as_ref())?;

        let req = CompletionRequest {
            task_class,
            model: &rule.model,
            system,
            user,
            max_tokens: rule.max_tokens,
            base_url: rule.base_url.as_deref(),
        };
        let start = Instant::now();
        let result = provider.complete(key.as_deref(), &req);
        let ms = start.elapsed().as_millis();

        let model_id = if provider_id == "mock" { "mock".to_string() } else { rule.model.clone() };
        let (ok, out_hash) = match &result {
            Ok(text) => (true, substrate::hex(blake3::hash(text.as_bytes()).as_bytes())),
            Err(_) => (false, String::new()),
        };
        sub.append(
            Kind::Action,
            ROUTER_CALL_SCHEMA,
            Origin { adapter: "runtime.router".into(), actor: task_class.into(), trust: Trust::Local },
            Privacy::P1,
            vec![],
            serde_json::json!({
                "task_class": task_class,
                // Operator dimension for per-reasoner scorecards
                // (COGNITIVE_ARCHITECTURE Part VIII).
                "operator": match task_class {
                    "worker.execute" => "deliberate",
                    "critic.review" => "review",
                    "archivist.appraise" => "appraise",
                    _ => "unknown",
                },
                "provider": provider_id,
                "model": model_id,
                "ms": ms as u64,
                "ok": ok,
                "out_hash": out_hash,
            })
            .to_string()
            .as_bytes(),
        )?;

        result.map(|text| Completion { text, model_id, ms })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_routes_and_logs_outcome() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let router = Router::load(dir.path(), true).unwrap();
        let c = router
            .complete(&mut sub, "worker.execute", "sys", r#"{"intent":"hello"}"#)
            .unwrap();
        assert_eq!(c.model_id, "mock");
        assert!(c.text.contains("NOTES.md"));
        let (events, _) = sub.events(true).unwrap();
        let call = events.iter().find(|e| e.header.schema == ROUTER_CALL_SCHEMA).unwrap();
        let substrate::BodyState::Plain(b) = &call.body else { panic!() };
        let v: serde_json::Value = serde_json::from_slice(b).unwrap();
        assert_eq!(v["ok"], true);
        assert_eq!(v["provider"], "mock");
        assert!(!String::from_utf8_lossy(b).contains("hello"));
    }

    #[test]
    fn unconfigured_route_demands_onboarding() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let router = Router::load(dir.path(), false).unwrap();
        let err = router.complete(&mut sub, "worker.execute", "s", "u").unwrap_err();
        assert!(err.to_string().contains("nx configure"), "{err}");
    }

    #[test]
    fn old_backend_field_still_loads() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("router.json"),
            r#"{"routes":{"worker.execute":{"backend":"mock","model":"m","max_tokens":16},
                         "critic.review":{"backend":"mock","model":"m","max_tokens":16}}}"#,
        )
        .unwrap();
        let router = Router::load(dir.path(), false).unwrap();
        assert_eq!(router.config.routes["worker.execute"].provider, "mock");
    }

    #[test]
    fn sealed_secret_resolves_when_env_absent() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        // openrouter uses OPENROUTER_API_KEY, unset in tests.
        let p = providers::get("openrouter").unwrap();
        assert!(Router::resolve_key(&sub, p.as_ref()).is_err());
        sub.store_secret(&secret_name("openrouter"), "sk-or-sealed").unwrap();
        assert_eq!(
            Router::resolve_key(&sub, p.as_ref()).unwrap().as_deref(),
            Some("sk-or-sealed")
        );
    }

    #[test]
    fn unknown_task_class_is_refused() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let router = Router::load(dir.path(), true).unwrap();
        assert!(router.complete(&mut sub, "nope.nothing", "s", "u").is_err());
    }
}
