# Custom Providers

## Overview
Define external AI endpoints directly in your configuration. Use this feature when you need to route requests to third-party APIs or self-hosted models not included in the default registry.

## Schema
```json
{
  "providers": {
    "<prefix>": {
      "base_url": "https://example.com/v1",
      "api_key_env": "MY_PROVIDER_API_KEY",
      "models": ["model-a", "model-b"]
    }
  }
}
```

## Field reference
| Field | Type | Required? | Description |
|---|---|---|---|
| `prefix` (map key) | string | Yes | Unique identifier used to route requests. |
| `base_url` | string | Yes | The OpenAI-compatible API endpoint. |
| `api_key_env` | string | Yes | Environment variable name containing the authentication token. |
| `models` | array | Yes | List of supported model identifiers available at this endpoint. |

## Routing behavior
- The map key becomes the display prefix used in `/model <prefix>/<model-id>`.
- Custom providers are consulted AFTER the builtin `MODEL_REGISTRY` (builtin wins on conflict).
- Models from all providers appear in `/model` + TAB and `/providers` output.

## A minimal working example
```json
{
  "providers": {
    "openrouter": {
      "base_url": "https://openrouter.ai/api/v1",
      "api_key_env": "OPENROUTER_API_KEY",
      "models": ["anthropic/claude-sonnet-4.6"]
    }
  }
}
```
```bash
export OPENROUTER_API_KEY="your-key-here"
claw --model openrouter/anthropic/claude-sonnet-4.6 "Summarize this text."
```

## Precedence
Project-level `./.claw.json` entries override user-level `~/.claw.json` when both define the same prefix. Always verify which configuration is active before troubleshooting routing issues.

## Restart required
`claw` caches custom providers per-process; restart the application after editing `.claw.json`.

## See also
- `docs/config/custom-provider-examples.md`
- `docs/providers/opencode-go.md`
- `schema/claw.schema.json`

