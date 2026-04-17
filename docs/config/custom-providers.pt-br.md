# Provedores customizados

## Visão geral
Defina endpoints de IA externos diretamente na sua configuração. Use este recurso quando precisar rotear requisições para APIs de terceiros ou modelos auto-hospedados que não estão no registro padrão.

## Esquema
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

## Referência de campos
| Campo | Tipo | Obrigatório? | Descrição |
|---|---|---|---|
| `prefix` (map key) | string | Sim | Identificador exclusivo usado para rotear requisições. |
| `base_url` | string | Sim | O endpoint de API compatível com OpenAI. |
| `api_key_env` | string | Sim | Nome da variável de ambiente que contém o token de autenticação. |
| `models` | array | Sim | Lista de identificadores de modelo suportados neste endpoint. |

## Comportamento de roteamento
- A chave do mapa se torna o prefixo exibido usado em `/model <prefix>/<model-id>`.
- Provedores customizados são consultados DEPOIS do builtin `MODEL_REGISTRY` (o builtin vence em caso de conflito).
- Modelos de todos os provedores aparecem na saída de `/model` + TAB e `/providers`.

## Exemplo mínimo funcional
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

## Precedência
Entradas no `./.claw.json` do projeto substituem as do `~/.claw.json` do usuário quando ambos definem o mesmo prefixo. Sempre verifique qual configuração está ativa antes de solucionar problemas de roteamento.

## Reinicialização necessária
O `claw` armazena provedores customizados em cache por processo; reinicie o aplicativo após editar o `.claw.json`.

## Consulte também
- `docs/config/custom-provider-examples.md`
- `docs/providers/opencode-go.md`
- `schema/claw.schema.json`

