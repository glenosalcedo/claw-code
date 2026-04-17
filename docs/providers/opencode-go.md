# OpenCode GO Provider

## Overview
OpenCode GO proxies nine Chinese and open-weight models through a single OpenAI-compatible endpoint. `claw-code` integrates this provider to let developers access multiple models using one credential and a standard wire format.

## Getting an API key
1. Register at https://opencode.ai.
2. Subscribe to the Zen tier ($5 first month, $10/month after).
3. Copy the API key from your dashboard.
4. Note the usage limits: $12 per 5-hour rolling window, $30/week, $30/month.

## Environment variables
```toml
# Required
OPENCODE_GO_API_KEY="sk-..."

# Optional (defaults to https://opencode.ai/zen/go/v1)
OPENCODE_GO_BASE_URL="https://custom-proxy.example.com/v1"
```

## Available models
| Model ID | Family | Publisher | Notes |
|---|---|---|---|
| `glm-5` | GLM | Zhipu | General-purpose code and text |
| `glm-5.1` | GLM | Zhipu | Updated instruction following |
| `kimi-k2.5` | Kimi | Moonshot | Long-context processing |
| `qwen3.5-plus` | Qwen | Alibaba | Balanced code generation |
| `qwen3.6-plus` | Qwen | Alibaba | Multi-step workflow support |
| `mimo-v2-pro` | MiMo | Xiaomi | Standard code completion |
| `mimo-v2-omni` | MiMo | Xiaomi | Multimodal input handling |
| `minimax-m2.5` | MiniMax | MiniMax | Conversational tasks |
| `minimax-m2.7` | MiniMax | MiniMax | Agentic workflow support |

## Usage
```bash
# Using provider prefix (recommended for clarity; required for kimi-k2.5)
claw --model opencode-go/glm-5.1 "refactor this function for readability"
claw --model opencode-go/kimi-k2.5 "explain what this codebase does"

# Bare model id works for glm-*, mimo-*, minimax-*, qwen3.*-plus
claw --model glm-5.1 "summarize this file"

# Interactive session
claw --model opencode-go/qwen3.6-plus
```

**Note:** bare `kimi-*` names (without the `opencode-go/` prefix) route to Alibaba DashScope per upstream behavior from [US-023](https://github.com/ultraworkers/claw-code/commit/cec8d17). To reach OpenCode GO's Kimi, always use the explicit `opencode-go/kimi-k2.5` form.

## How routing works
`claw-code` routes `opencode-go/`-prefixed requests to `OPENCODE_GO_BASE_URL` using `OPENCODE_GO_API_KEY`. Bare names for GLM / MiMo / MiniMax / Qwen3.x-plus resolve via `MODEL_REGISTRY` and target the same endpoint. Bare `kimi-*` names continue to route to DashScope (upstream behavior).

## Known limitations
- Streaming cost frames are handled transparently.
- GLM models emit `reasoning_content` before `content`; the client processes this as a fallback.
- Tool-use support varies. See the OpenCode GO dashboard for current figures.

## Further reading
- [OpenCode GO official documentation](https://opencode.ai/docs/pt-br/go)
- [claw-code main README](https://github.com/ultraworkers/claw-code)
- [Supported providers list](../README.md#providers)
