//! Provider-agnostic model interface (Principle 5: models are peripherals).
//!
//! Every provider implements [`Provider`]; nothing outside this module knows
//! request or response shapes. Switching vendors is a `router.json` edit —
//! zero application-logic changes. Three wire protocols cover the world:
//!
//! - **Anthropic** Messages API.
//! - **Google Gemini** generateContent API.
//! - **OpenAI chat completions** — spoken natively by OpenAI and, as the de
//!   facto standard, by OpenRouter, Ollama, LM Studio, and most future
//!   providers. One implementation, many providers; `openai-compat` with a
//!   custom `base_url` covers anything not listed here yet.
//!
//! Keys come from the environment (operator override) or the sealed secrets
//! store — never from code, never from config files in plaintext.

use anyhow::{anyhow, bail, Result};

pub struct CompletionRequest<'a> {
    pub task_class: &'a str,
    pub model: &'a str,
    pub system: &'a str,
    pub user: &'a str,
    pub max_tokens: u32,
    /// Override the provider's default endpoint (self-hosted, proxies).
    pub base_url: Option<&'a str>,
}

pub trait Provider {
    fn id(&self) -> &'static str;
    /// Environment variable that may carry the key (operator override).
    fn env_key(&self) -> Option<&'static str>;
    fn needs_key(&self) -> bool;
    fn complete(&self, key: Option<&str>, req: &CompletionRequest) -> Result<String>;
}

/// All known provider ids, in onboarding display order.
pub const PROVIDER_IDS: [&str; 8] = [
    "anthropic", "openai", "gemini", "openrouter", "ollama", "lmstudio", "openai-compat", "mock",
];

pub fn get(id: &str) -> Result<Box<dyn Provider>> {
    Ok(match id {
        "anthropic" => Box::new(Anthropic),
        "gemini" => Box::new(Gemini),
        "openai" => Box::new(OpenAiCompat {
            id: "openai",
            default_base: "https://api.openai.com/v1",
            env: Some("OPENAI_API_KEY"),
            needs_key: true,
        }),
        "openrouter" => Box::new(OpenAiCompat {
            id: "openrouter",
            default_base: "https://openrouter.ai/api/v1",
            env: Some("OPENROUTER_API_KEY"),
            needs_key: true,
        }),
        "ollama" => Box::new(OpenAiCompat {
            id: "ollama",
            default_base: "http://localhost:11434/v1",
            env: None,
            needs_key: false,
        }),
        "lmstudio" => Box::new(OpenAiCompat {
            id: "lmstudio",
            default_base: "http://localhost:1234/v1",
            env: None,
            needs_key: false,
        }),
        "openai-compat" => Box::new(OpenAiCompat {
            id: "openai-compat",
            default_base: "",
            env: Some("OPENAI_COMPAT_API_KEY"),
            needs_key: false,
        }),
        "mock" => Box::new(Mock),
        other => bail!("unknown provider '{other}' (known: {})", PROVIDER_IDS.join(", ")),
    })
}

/// A sensible default model per provider, offered (editable) at onboarding.
pub fn suggested_model(id: &str) -> &'static str {
    match id {
        "anthropic" => "claude-sonnet-5",
        "openai" => "gpt-5.2",
        "gemini" => "gemini-2.5-pro",
        "openrouter" => "anthropic/claude-sonnet-5",
        "ollama" => "llama3.3",
        "lmstudio" => "local-model",
        _ => "",
    }
}

// ---------------------------------------------------------------- Anthropic

struct Anthropic;

impl Provider for Anthropic {
    fn id(&self) -> &'static str {
        "anthropic"
    }
    fn env_key(&self) -> Option<&'static str> {
        Some("ANTHROPIC_API_KEY")
    }
    fn needs_key(&self) -> bool {
        true
    }
    fn complete(&self, key: Option<&str>, req: &CompletionRequest) -> Result<String> {
        let key = key.ok_or_else(|| anyhow!("anthropic requires an API key"))?;
        let base = req.base_url.unwrap_or("https://api.anthropic.com");
        let resp: serde_json::Value = ureq::post(&format!("{base}/v1/messages"))
            .set("x-api-key", key)
            .set("anthropic-version", "2023-06-01")
            .set("content-type", "application/json")
            .send_json(serde_json::json!({
                "model": req.model,
                "max_tokens": req.max_tokens,
                "system": req.system,
                "messages": [{"role": "user", "content": req.user}],
            }))
            .map_err(|e| anyhow!("anthropic api: {e}"))?
            .into_json()?;
        resp["content"][0]["text"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| anyhow!("unexpected anthropic response shape: {resp}"))
    }
}

// ------------------------------------------------------------------- Gemini

struct Gemini;

impl Provider for Gemini {
    fn id(&self) -> &'static str {
        "gemini"
    }
    fn env_key(&self) -> Option<&'static str> {
        Some("GEMINI_API_KEY")
    }
    fn needs_key(&self) -> bool {
        true
    }
    fn complete(&self, key: Option<&str>, req: &CompletionRequest) -> Result<String> {
        let key = key.ok_or_else(|| anyhow!("gemini requires an API key"))?;
        let base = req.base_url.unwrap_or("https://generativelanguage.googleapis.com");
        let url = format!("{base}/v1beta/models/{}:generateContent", req.model);
        let resp: serde_json::Value = ureq::post(&url)
            .set("x-goog-api-key", key)
            .set("content-type", "application/json")
            .send_json(serde_json::json!({
                "system_instruction": {"parts": [{"text": req.system}]},
                "contents": [{"role": "user", "parts": [{"text": req.user}]}],
                "generationConfig": {"maxOutputTokens": req.max_tokens},
            }))
            .map_err(|e| anyhow!("gemini api: {e}"))?
            .into_json()?;
        resp["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| anyhow!("unexpected gemini response shape: {resp}"))
    }
}

// -------------------------------------------- OpenAI-compatible (the family)

struct OpenAiCompat {
    id: &'static str,
    default_base: &'static str,
    env: Option<&'static str>,
    needs_key: bool,
}

impl Provider for OpenAiCompat {
    fn id(&self) -> &'static str {
        self.id
    }
    fn env_key(&self) -> Option<&'static str> {
        self.env
    }
    fn needs_key(&self) -> bool {
        self.needs_key
    }
    fn complete(&self, key: Option<&str>, req: &CompletionRequest) -> Result<String> {
        let base = match (req.base_url, self.default_base) {
            (Some(b), _) => b,
            (None, "") => bail!("provider '{}' requires base_url in router.json", self.id),
            (None, d) => d,
        };
        let mut http = ureq::post(&format!("{base}/chat/completions"))
            .set("content-type", "application/json");
        if let Some(k) = key {
            http = http.set("authorization", &format!("Bearer {k}"));
        } else if self.needs_key {
            bail!("provider '{}' requires an API key", self.id);
        }
        let resp: serde_json::Value = http
            .send_json(serde_json::json!({
                "model": req.model,
                // max_tokens (not max_completion_tokens): the compat ecosystem
                // (OpenRouter/Ollama/LM Studio) understands the former.
                "max_tokens": req.max_tokens,
                "messages": [
                    {"role": "system", "content": req.system},
                    {"role": "user", "content": req.user},
                ],
            }))
            .map_err(|e| anyhow!("{} api: {e}", self.id))?
            .into_json()?;
        resp["choices"][0]["message"]["content"]
            .as_str()
            .map(String::from)
            .ok_or_else(|| anyhow!("unexpected {} response shape: {resp}", self.id))
    }
}

// --------------------------------------------------------------------- Mock

/// Deterministic offline provider for tests and demos. Scripted per task
/// class; exercises every loop mechanism without a network.
struct Mock;

impl Provider for Mock {
    fn id(&self) -> &'static str {
        "mock"
    }
    fn env_key(&self) -> Option<&'static str> {
        None
    }
    fn needs_key(&self) -> bool {
        false
    }
    fn complete(&self, _key: Option<&str>, req: &CompletionRequest) -> Result<String> {
        let input: serde_json::Value = serde_json::from_str(req.user).unwrap_or_default();
        match req.task_class {
            "worker.execute" => {
                let intent = input["intent"].as_str().unwrap_or("(no intent)");
                // Memory demonstrably influences output: cautions from the
                // working set surface in the plan and the artifact.
                let cautions: Vec<String> = input["memory"]["cautions"]
                    .as_array()
                    .map(|a| {
                        a.iter()
                            .filter_map(|c| c["statement"].as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let mut plan = vec!["record the intent in NOTES.md (mock worker)".to_string()];
                let mut caution_block = String::new();
                if !cautions.is_empty() {
                    plan.push(format!("heeding {} caution(s) from memory", cautions.len()));
                    caution_block = format!(
                        "\n## Cautions heeded\n{}\n",
                        cautions.iter().map(|c| format!("- {c}")).collect::<Vec<_>>().join("\n")
                    );
                }
                Ok(serde_json::json!({
                    "plan": plan,
                    "operations": [{
                        "op": "write",
                        "path": "NOTES.md",
                        "content": format!("# Task\n\n{intent}\n{caution_block}\n_(written by the mock worker)_\n"),
                    }],
                })
                .to_string())
            }
            "critic.review" => Ok(serde_json::json!({"verdict": "approve", "issues": []}).to_string()),
            "archivist.appraise" => {
                let eps = input["episodes"].as_array().cloned().unwrap_or_default();
                let mut candidates: Vec<serde_json::Value> = eps
                    .iter()
                    .filter(|e| {
                        let t = e["text"].as_str().unwrap_or("");
                        t.contains("prefer") || t.contains("require")
                    })
                    .map(|e| {
                        serde_json::json!({
                            "statement": e["text"].as_str().unwrap_or(""),
                            "scope": "general",
                            "kind": "belief",
                            "stakes": "R1",
                            "falsifiers": ["a counterexample is observed"],
                            "provenance": [e["id"]],
                            "horizon_days": null,
                        })
                    })
                    .collect();
                // One deliberately unfalsifiable candidate: proves the
                // admission rule filters model output, not just user input.
                if let Some(first) = eps.first() {
                    candidates.push(serde_json::json!({
                        "statement": "everything is always fine",
                        "scope": "general",
                        "kind": "belief",
                        "stakes": "R1",
                        "falsifiers": [],
                        "provenance": [first["id"]],
                    }));
                }
                Ok(serde_json::json!({ "candidates": candidates }).to_string())
            }
            other => bail!("mock provider has no script for task class '{other}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_knows_every_advertised_provider() {
        for id in PROVIDER_IDS {
            let p = get(id).unwrap();
            assert_eq!(p.id(), id);
        }
        assert!(get("skynet").is_err());
    }

    #[test]
    fn key_requirements_are_sane() {
        assert!(get("anthropic").unwrap().needs_key());
        assert!(get("openai").unwrap().needs_key());
        assert!(get("gemini").unwrap().needs_key());
        assert!(get("openrouter").unwrap().needs_key());
        assert!(!get("ollama").unwrap().needs_key());
        assert!(!get("lmstudio").unwrap().needs_key());
        assert!(!get("mock").unwrap().needs_key());
    }

    #[test]
    fn compat_without_base_url_is_refused() {
        let p = get("openai-compat").unwrap();
        let req = CompletionRequest {
            task_class: "worker.execute",
            model: "m",
            system: "s",
            user: "{}",
            max_tokens: 16,
            base_url: None,
        };
        assert!(p.complete(None, &req).unwrap_err().to_string().contains("base_url"));
    }
}
