# Model Selection

## What changed
TAB completion for `/model` now renders every registered model as `<provider-prefix>/<model-id>`, grouped by provider (Anthropic → xAI → OpenAi/DashScope → OpenCode GO). Short aliases (`opus`, `sonnet`, `haiku`, `grok`, `kimi`) still work and sit alongside the registry-driven entries in the same completion list.

## Example TAB output
```text
$ claw
claw-code  —  session 38f2...
Model: opencode-go/glm-5.1  (detected from env)

> /model <TAB>
/model anthropic/haiku
/model anthropic/opus
/model anthropic/sonnet
/model xai/grok
/model xai/grok-2
/model xai/grok-3
/model xai/grok-3-mini
/model xai/grok-code-fast-1
/model xai/grok-4-0709
/model xai/grok-4-fast-reasoning
/model xai/grok-4-fast-non-reasoning
/model xai/grok-4-1-fast-reasoning
/model xai/grok-4-1-fast-non-reasoning
/model xai/grok-4.20-0309-reasoning
/model xai/grok-4.20-0309-non-reasoning
/model opencode-go/glm-5
/model opencode-go/glm-5.1
/model opencode-go/kimi-k2.5
/model opencode-go/qwen3.5-plus
/model opencode-go/qwen3.6-plus
/model opencode-go/mimo-v2-pro
/model opencode-go/mimo-v2-omni
/model opencode-go/minimax-m2.5
/model opencode-go/minimax-m2.7
/model opus          (short alias)
/model sonnet        (short alias)
/model haiku         (short alias)
/model grok          (short alias)
/model kimi          (short alias)
```

Note: the OpenAi `kimi` alias and its `kimi-k2.5` canonical form render under the `openai/` prefix in the registry iteration. Bare `kimi-*` continues to route to Alibaba DashScope per upstream behavior from US-023; to target OpenCode GO use the explicit `opencode-go/kimi-k2.5` form.

## Selecting a model
- **REPL:** type `/model <provider>/<model-id>` and hit enter.
- **CLI:** pass `--model <provider>/<model-id>` at launch.
- **Short alias:** `/model kimi` or `--model kimi` resolves to the canonical ID defined in `MODEL_REGISTRY`.

## How it works under the hood
`slash_command_completion_candidates_with_sessions()` in `rusty-claude-cli/src/main.rs` now iterates over `api::registered_models()` and formats each entry via `api::provider_display_prefix()`. Both helpers are public in the `api` crate so plugins and other tools can reuse the same rendering.

## See also
- [`docs/providers/opencode-go.md`](../providers/opencode-go.md) — OpenCode GO provider details
- [`docs/providers/opencode-go.pt-br.md`](../providers/opencode-go.pt-br.md) — PT-BR
- [`docs/providers/opencode-go.es.md`](../providers/opencode-go.es.md) — ES
