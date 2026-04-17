# Seleção de modelo

## O que mudou
A conclusão por TAB para `/model` agora exibe cada modelo registrado no formato `<provider-prefix>/<model-id>`, agrupados por provedor (Anthropic → xAI → OpenAi/DashScope → OpenCode GO). Os aliases curtos (`opus`, `sonnet`, `haiku`, `grok`, `kimi`) continuam funcionando e permanecem na mesma lista de conclusão, ao lado das entradas gerenciadas pelo registro.

## Exemplo do output do TAB
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

Nota: o alias `kimi` da OpenAi e sua forma canônica `kimi-k2.5` são exibidos sob o prefixo `openai/` durante a iteração do registro. O padrão `kimi-*` isolado continua sendo roteado para o Alibaba DashScope, conforme o comportamento upstream do US-023; para direcionar ao OpenCode GO, utilize a forma explícita `opencode-go/kimi-k2.5`.

## Selecionando um modelo
- **REPL:** digite `/model <provider>/<model-id>` e pressione Enter.
- **CLI:** informe `--model <provider>/<model-id>` ao iniciar.
- **Short alias:** `/model kimi` ou `--model kimi` é resolvido para o ID canônico definido em `MODEL_REGISTRY`.

## Como funciona por baixo dos panos
A função `slash_command_completion_candidates_with_sessions()` em `rusty-claude-cli/src/main.rs` agora itera sobre `api::registered_models()` e formata cada entrada por meio de `api::provider_display_prefix()`. Ambos os auxiliares são públicos no crate `api`, permitindo que plug-ins e outras ferramentas reutilizem a mesma renderização.

## Veja também
- [`docs/providers/opencode-go.md`](../providers/opencode-go.md) — Detalhes do provedor OpenCode GO
- [`docs/providers/opencode-go.pt-br.md`](../providers/opencode-go.pt-br.md) — PT-BR
- [`docs/providers/opencode-go.es.md`](../providers/opencode-go.es.md) — ES
