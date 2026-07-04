//! Onboarding: apply a provider choice — seal the key, write the route
//! table. The interactive prompts live in the CLI; this module is the
//! side-effect-bearing core so it stays testable.

use std::fs;
use std::path::Path;

use anyhow::{bail, Result};
use substrate::Substrate;

use crate::providers;
use crate::router::{secret_name, RouteRule, RouterConfig, TASK_CLASSES};

pub struct ProviderChoice {
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
    pub base_url: Option<String>,
}

/// Validate and apply: key into the sealed store, routes into router.json.
pub fn apply(data_dir: &Path, sub: &mut Substrate, choice: &ProviderChoice) -> Result<()> {
    let provider = providers::get(&choice.provider)?; // validates the id
    if provider.id() == "mock" {
        bail!("mock is a test provider — choose a real one (or run commands with --mock)");
    }
    if choice.model.trim().is_empty() {
        bail!("a model name is required");
    }
    if provider.needs_key() && choice.api_key.as_deref().map(str::trim).unwrap_or("").is_empty() {
        // The env var escape hatch still exists, but onboarding should leave
        // a working configuration behind, not a latent failure.
        bail!(
            "provider '{}' requires an API key (it will be sealed under your passphrase)",
            provider.id()
        );
    }
    if provider.id() == "openai-compat"
        && choice.base_url.as_deref().map(str::trim).unwrap_or("").is_empty()
    {
        bail!("openai-compat requires --base-url (the endpoint of your compatible server)");
    }

    if let Some(key) = choice.api_key.as_deref().map(str::trim).filter(|k| !k.is_empty()) {
        sub.store_secret(&secret_name(provider.id()), key)?;
    }

    let mut routes = std::collections::BTreeMap::new();
    for tc in TASK_CLASSES {
        routes.insert(
            tc.to_string(),
            RouteRule {
                provider: provider.id().to_string(),
                model: choice.model.trim().to_string(),
                max_tokens: if tc == "worker.execute" { 8192 } else { 4096 },
                base_url: choice.base_url.clone().filter(|b| !b.trim().is_empty()),
                fallbacks: vec![],
            },
        );
    }
    let config = RouterConfig { routes };
    fs::write(
        data_dir.join("router.json"),
        serde_json::to_string_pretty(&config)?,
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::Router;

    #[test]
    fn apply_seals_key_and_writes_routes() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        apply(
            dir.path(),
            &mut sub,
            &ProviderChoice {
                provider: "openai".into(),
                model: "gpt-5.2".into(),
                api_key: Some("sk-test".into()),
                base_url: None,
            },
        )
        .unwrap();
        // Key sealed, not plaintext anywhere in the data dir config.
        assert_eq!(sub.load_secret("provider-key/openai").unwrap().as_deref(), Some("sk-test"));
        let router_raw = std::fs::read_to_string(dir.path().join("router.json")).unwrap();
        assert!(!router_raw.contains("sk-test"));
        assert!(router_raw.contains("\"provider\": \"openai\""));
        // Router loads the new table.
        let _ = Router::load(dir.path(), false).unwrap();
    }

    #[test]
    fn apply_validates() {
        let dir = tempfile::tempdir().unwrap();
        let mut sub = Substrate::init(dir.path(), "pass").unwrap();
        let base = |p: &str, key: Option<&str>, url: Option<&str>| ProviderChoice {
            provider: p.into(),
            model: "m".into(),
            api_key: key.map(String::from),
            base_url: url.map(String::from),
        };
        assert!(apply(dir.path(), &mut sub, &base("mock", None, None)).is_err());
        assert!(apply(dir.path(), &mut sub, &base("anthropic", None, None)).is_err()); // key required
        assert!(apply(dir.path(), &mut sub, &base("openai-compat", None, None)).is_err()); // base_url required
        assert!(apply(dir.path(), &mut sub, &base("ollama", None, None)).is_ok()); // local: no key needed
    }
}
