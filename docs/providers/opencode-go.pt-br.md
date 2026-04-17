# Provedor OpenCode GO

## Visão geral
O OpenCode GO atua como proxy para nove modelos chineses e de pesos abertos por meio de um único endpoint compatível com OpenAI. O `claw-code` integra esse provedor para permitir que os desenvolvedores acessem vários modelos usando uma única credencial e um formato de transmissão padrão.

## Obtendo uma chave de API
1. Registre-se em https://opencode.ai.
2. Assine o plano Zen ($5 no primeiro mês, $10/mês após).
3. Copie a chave de API no seu painel.
4. Observe os limites de uso: $12 por janela móvel de 5 horas, $30/semana, $30/mês.

## Variáveis de ambiente
```toml
# Obrigatório
OPENCODE_GO_API_KEY="sk-..."

# Opcional (padrão: https://opencode.ai/zen/go/v1)
OPENCODE_GO_BASE_URL="https://custom-proxy.example.com/v1"
```

## Modelos disponíveis
| ID do modelo | Família | Publicador | Observações |
|---|---|---|---|
| `glm-5` | GLM | Zhipu | Código e texto de uso geral |
| `glm-5.1` | GLM | Zhipu | Seguimento de instruções atualizado |
| `kimi-k2.5` | Kimi | Moonshot | Processamento de contexto longo |
| `qwen3.5-plus` | Qwen | Alibaba | Geração de código equilibrada |
| `qwen3.6-plus` | Qwen | Alibaba | Suporte a fluxos de trabalho em múltiplas etapas |
| `mimo-v2-pro` | MiMo | Xiaomi | Conclusão de código padrão |
| `mimo-v2-omni` | MiMo | Xiaomi | Manipulação de entrada multimodal |
| `minimax-m2.5` | MiniMax | MiniMax | Tarefas conversacionais |
| `minimax-m2.7` | MiniMax | MiniMax | Suporte a fluxos de trabalho com agentes |

## Uso
```bash
# Usando o prefixo do provedor (recomendado para clareza; obrigatório para kimi-k2.5)
claw --model opencode-go/glm-5.1 "refactor this function for readability"
claw --model opencode-go/kimi-k2.5 "explain what this codebase does"

# Nome direto do modelo funciona para glm-*, mimo-*, minimax-*, qwen3.*-plus
claw --model glm-5.1 "summarize this file"

# Sessão interativa
claw --model opencode-go/qwen3.6-plus
```

**Observação:** nomes `kimi-*` sem o prefixo `opencode-go/` são roteados para o Alibaba DashScope, mantendo o comportamento do upstream a partir do [US-023](https://github.com/ultraworkers/claw-code/commit/cec8d17). Para acessar o Kimi via OpenCode GO, use sempre a forma explícita `opencode-go/kimi-k2.5`.

## Como funciona o roteamento
O `claw-code` roteia requisições com o prefixo `opencode-go/` para `OPENCODE_GO_BASE_URL` usando `OPENCODE_GO_API_KEY`. Nomes diretos de GLM / MiMo / MiniMax / Qwen3.x-plus são resolvidos via `MODEL_REGISTRY` e direcionados ao mesmo endpoint. Nomes `kimi-*` sem prefixo continuam indo para o DashScope (comportamento do upstream).

## Limitações conhecidas
- Os frames de custo de streaming são tratados de forma transparente.
- Os modelos GLM emitem `reasoning_content` antes de `content`; o cliente processa isso como fallback.
- O suporte ao uso de ferramentas varia. Consulte o painel do OpenCode GO para os dados atuais.

## Leitura adicional
- [Documentação oficial do OpenCode GO](https://opencode.ai/docs/pt-br/go)
- [README principal do claw-code](https://github.com/ultraworkers/claw-code)
- [Lista de provedores suportados](../README.md#providers)
