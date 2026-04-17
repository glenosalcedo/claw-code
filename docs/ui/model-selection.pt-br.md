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

## O comando /providers
O comando `/providers` verifica o ambiente de execução e lista todos os provedores de IA registrados junto com o status de suas credenciais. Ele detecta automaticamente as variáveis de ambiente necessárias e agrupa os modelos disponíveis sob seus respectivos namespaces. Essa saída oferece uma visão consolidada da configuração ativa, indicando quais serviços estão autenticados e prontos para roteamento.

```text
Providers:
  [OK ] anthropic      env ANTHROPIC_API_KEY detected
         aliases: haiku, opus, sonnet
  [OK ] xai            env XAI_API_KEY detected
         models: grok, grok-2, grok-3, grok-3-mini, grok-4-0709,
                 grok-4-fast-reasoning, grok-4-fast-non-reasoning,
                 grok-4-1-fast-reasoning, grok-4-1-fast-non-reasoning,
                 grok-4.20-0309-reasoning, grok-4.20-0309-non-reasoning,
                 grok-code-fast-1
  [MISS] dashscope     env DASHSCOPE_API_KEY not set
         aliases: kimi
         (bare kimi-* names route here; use opencode-go/kimi-k2.5 for OpenCode GO)
  [OK ] opencode-go    env OPENCODE_GO_API_KEY detected
         models: glm-5, glm-5.1, kimi-k2.5, minimax-m2.5, minimax-m2.7,
                 mimo-v2-omni, mimo-v2-pro, qwen3.5-plus, qwen3.6-plus
```

**Por que ajuda**
- Realiza uma verificação rápida para confirmar quais provedores estão prontos para uso imediato.
- Garante visibilidade explícita sobre o registro de cada modelo, incluindo a `auth_env` exigida e o `prefix` de roteamento.
- Complementa o fluxo `/model` + `TAB`, que agora é totalmente orientado por registro, conforme a PR anterior.

## Veja também
- [`docs/providers/opencode-go.md`](../providers/opencode-go.md) — Detalhes do provedor OpenCode GO
- [`docs/providers/opencode-go.pt-br.md`](../providers/opencode-go.pt-br.md) — PT-BR
- [`docs/providers/opencode-go.es.md`](../providers/opencode-go.es.md) — ES
