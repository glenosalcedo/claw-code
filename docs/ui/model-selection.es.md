# Selección de modelo

## Qué cambió
La autocompletación con TAB para `/model` ahora muestra cada modelo registrado como `<provider-prefix>/<model-id>`, agrupados por proveedor (Anthropic → xAI → OpenAi/DashScope → OpenCode GO). Los alias cortos (`opus`, `sonnet`, `haiku`, `grok`, `kimi`) siguen funcionando y aparecen junto a las entradas basadas en el registro en la misma lista de autocompletado.

## Ejemplo de salida del TAB
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

Nota: el alias `kimi` de OpenAi y su forma canónica `kimi-k2.5` se muestran bajo el prefijo `openai/` en la iteración del registro. El uso directo de `kimi-*` sigue redirigiéndose a Alibaba DashScope según el comportamiento original de US-023; para apuntar a OpenCode GO, utilice la forma explícita `opencode-go/kimi-k2.5`.

## Seleccionando un modelo
- **REPL:** escriba `/model <provider>/<model-id>` y presione Enter.
- **CLI:** especifique `--model <provider>/<model-id>` al iniciar.
- **Alias corto:** `/model kimi` o `--model kimi` se resuelve al ID canónico definido en `MODEL_REGISTRY`.

## Cómo funciona internamente
`slash_command_completion_candidates_with_sessions()` en `rusty-claude-cli/src/main.rs` ahora itera sobre `api::registered_models()` y formatea cada entrada mediante `api::provider_display_prefix()`. Ambas funciones auxiliares son públicas en el crate `api`, por lo que los complementos y otras herramientas pueden reutilizar la misma lógica de representación.

## Ver también
- [`docs/providers/opencode-go.md`](../providers/opencode-go.md) — Detalles del proveedor OpenCode GO
- [`docs/providers/opencode-go.pt-br.md`](../providers/opencode-go.pt-br.md) — PT-BR
- [`docs/providers/opencode-go.es.md`](../providers/opencode-go.es.md) — ES
