//! Custom provider configuration loader for `.claw.json` files.
//!
//! This module reads and merges custom LLM provider definitions from
//! user-level (`~/.claw.json`) and project-level (`./.claw.json`)
//! configuration files, letting users register arbitrary OpenAI-compatible
//! endpoints (OpenRouter, Ollama, LM Studio, Groq, etc.) without
//! recompiling `claw-code`.

use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

/// Runtime-registered provider pulled from `.claw.json`.
#[derive(Debug, Clone, Deserialize)]
pub struct CustomProvider {
    /// Display prefix (e.g., `"openrouter"`). Matches the map key in
    /// `.claw.json` so users run `claw --model openrouter/<model-id>`.
    pub name: String,
    pub base_url: String,
    pub api_key_env: String,
    #[serde(default)]
    pub models: Vec<String>,
}

/// Intermediate struct used only for deserialization — the map key in
/// `.claw.json` becomes the provider's `name` field.
#[derive(Debug, Deserialize)]
struct ProviderConfigFields {
    base_url: String,
    api_key_env: String,
    #[serde(default)]
    models: Vec<String>,
}

/// Read custom providers from `.claw.json`, merging user-level
/// (`~/.claw.json`) with project-level (`./.claw.json`). Project entries
/// override user entries by `name`. Invalid or missing files are treated
/// as empty — never a hard error — with a single `eprintln!` warning per
/// malformed file so users see why their customization didn't load.
#[must_use]
pub fn load_custom_providers() -> Vec<CustomProvider> {
    let mut providers_by_name: HashMap<String, CustomProvider> = HashMap::new();

    if let Ok(home) = env::var("HOME") {
        let user_config_path = Path::new(&home).join(".claw.json");
        load_providers_from_file(&user_config_path, &mut providers_by_name);
    }

    let project_config_path = Path::new(".claw.json");
    load_providers_from_file(project_config_path, &mut providers_by_name);

    providers_by_name.into_values().collect()
}

fn load_providers_from_file(path: &Path, providers_by_name: &mut HashMap<String, CustomProvider>) {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return,
        Err(err) => {
            eprintln!("claw: {}: {}", path.display(), err);
            return;
        }
    };

    let config: serde_json::Value = match serde_json::from_str(&content) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("claw: {}: {}", path.display(), err);
            return;
        }
    };

    let Some(providers_obj) = config.get("providers").and_then(|v| v.as_object()) else {
        return;
    };

    for (name, provider_value) in providers_obj {
        match serde_json::from_value::<ProviderConfigFields>(provider_value.clone()) {
            Ok(fields) => {
                let provider = CustomProvider {
                    name: name.clone(),
                    base_url: fields.base_url,
                    api_key_env: fields.api_key_env,
                    models: fields.models,
                };
                providers_by_name.insert(name.clone(), provider);
            }
            Err(err) => {
                eprintln!("claw: {}: provider '{}': {}", path.display(), name, err);
            }
        }
    }
}

/// Given a model string like `"openrouter/claude-sonnet-4.6"`, find the
/// matching custom provider by comparing the part before the first `/` to
/// each provider's `name`. Returns `None` when the model has no `/` or no
/// registered prefix matches.
#[must_use]
pub fn find_custom_provider_for_model<'a>(
    providers: &'a [CustomProvider],
    model: &str,
) -> Option<&'a CustomProvider> {
    let (prefix, _rest) = model.split_once('/')?;
    providers.iter().find(|p| p.name == prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_providers_object() {
        let json = r#"{
            "openrouter": {
                "base_url": "https://openrouter.ai/api/v1",
                "api_key_env": "OPENROUTER_API_KEY",
                "models": ["anthropic/claude-sonnet-4.6", "google/gemini-2.5-pro"]
            },
            "ollama-local": {
                "base_url": "http://localhost:11434/v1",
                "api_key_env": "OLLAMA_API_KEY",
                "models": ["llama3.3:70b", "qwen2.5-coder:32b"]
            }
        }"#;

        let map: HashMap<String, ProviderConfigFields> = serde_json::from_str(json).unwrap();
        let providers: Vec<CustomProvider> = map
            .into_iter()
            .map(|(name, fields)| CustomProvider {
                name,
                base_url: fields.base_url,
                api_key_env: fields.api_key_env,
                models: fields.models,
            })
            .collect();

        assert_eq!(providers.len(), 2);

        let openrouter = providers.iter().find(|p| p.name == "openrouter").unwrap();
        assert_eq!(openrouter.base_url, "https://openrouter.ai/api/v1");
        assert_eq!(openrouter.api_key_env, "OPENROUTER_API_KEY");
        assert_eq!(openrouter.models.len(), 2);
        assert_eq!(openrouter.models[0], "anthropic/claude-sonnet-4.6");

        let ollama = providers.iter().find(|p| p.name == "ollama-local").unwrap();
        assert_eq!(ollama.base_url, "http://localhost:11434/v1");
        assert_eq!(ollama.api_key_env, "OLLAMA_API_KEY");
        assert_eq!(ollama.models[1], "qwen2.5-coder:32b");
    }

    #[test]
    fn find_custom_provider_matches_prefix() {
        let providers = vec![
            CustomProvider {
                name: "openrouter".to_string(),
                base_url: "https://openrouter.ai/api/v1".to_string(),
                api_key_env: "OPENROUTER_API_KEY".to_string(),
                models: vec!["anthropic/claude-sonnet-4.6".to_string()],
            },
            CustomProvider {
                name: "ollama-local".to_string(),
                base_url: "http://localhost:11434/v1".to_string(),
                api_key_env: "OLLAMA_API_KEY".to_string(),
                models: vec!["llama3.3:70b".to_string()],
            },
        ];

        let result = find_custom_provider_for_model(&providers, "openrouter/foo");
        assert_eq!(result.map(|p| p.name.as_str()), Some("openrouter"));

        let result = find_custom_provider_for_model(&providers, "ollama-local/bar");
        assert_eq!(result.map(|p| p.name.as_str()), Some("ollama-local"));
    }

    #[test]
    fn find_custom_provider_returns_none_without_slash() {
        let providers = vec![CustomProvider {
            name: "openrouter".to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            api_key_env: "OPENROUTER_API_KEY".to_string(),
            models: vec![],
        }];
        assert!(find_custom_provider_for_model(&providers, "just-a-name").is_none());
    }

    #[test]
    fn find_custom_provider_returns_none_for_unknown_prefix() {
        let providers = vec![CustomProvider {
            name: "openrouter".to_string(),
            base_url: "https://openrouter.ai/api/v1".to_string(),
            api_key_env: "OPENROUTER_API_KEY".to_string(),
            models: vec![],
        }];
        assert!(find_custom_provider_for_model(&providers, "xxx/whatever").is_none());
    }
}
