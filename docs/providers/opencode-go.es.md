# Proveedor OpenCode GO

## Visión general
OpenCode GO actúa como proxy para nueve modelos chinos y de pesos abiertos a través de un único endpoint compatible con OpenAI. `claw-code` integra este proveedor para permitir a los desarrolladores acceder a múltiples modelos utilizando una sola credencial y un formato de transmisión estándar.

## Obteniendo una clave API
1. Regístrate en https://opencode.ai.
2. Suscríbete al nivel Zen ($5 el primer mes, $10/mes después).
3. Copia la clave API desde tu panel de control.
4. Ten en cuenta los límites de uso: $12 por ventana móvil de 5 horas, $30/semana, $30/mes.

## Variables de entorno
```toml
# Obligatorio
OPENCODE_GO_API_KEY="sk-..."

# Opcional (por defecto: https://opencode.ai/zen/go/v1)
OPENCODE_GO_BASE_URL="https://custom-proxy.example.com/v1"
```

## Modelos disponibles
| ID del modelo | Familia | Publicador | Notas |
|---|---|---|---|
| `glm-5` | GLM | Zhipu | Código y texto de propósito general |
| `glm-5.1` | GLM | Zhipu | Seguimiento de instrucciones mejorado |
| `kimi-k2.5` | Kimi | Moonshot | Procesamiento de contexto largo |
| `qwen3.5-plus` | Qwen | Alibaba | Generación de código equilibrada |
| `qwen3.6-plus` | Qwen | Alibaba | Compatibilidad con flujos de trabajo de varios pasos |
| `mimo-v2-pro` | MiMo | Xiaomi | Finalización de código estándar |
| `mimo-v2-omni` | MiMo | Xiaomi | Manejo de entrada multimodal |
| `minimax-m2.5` | MiniMax | MiniMax | Tareas conversacionales |
| `minimax-m2.7` | MiniMax | MiniMax | Compatibilidad con flujos de trabajo de agentes |

## Uso
```bash
# Uso del prefijo del proveedor (recomendado para mayor claridad; obligatorio para kimi-k2.5)
claw --model opencode-go/glm-5.1 "refactor this function for readability"
claw --model opencode-go/kimi-k2.5 "explain what this codebase does"

# El ID del modelo sin prefijo funciona para glm-*, mimo-*, minimax-*, qwen3.*-plus
claw --model glm-5.1 "summarize this file"

# Sesión interactiva
claw --model opencode-go/qwen3.6-plus
```

**Nota:** los nombres `kimi-*` sin el prefijo `opencode-go/` se enrutan a Alibaba DashScope según el comportamiento del upstream desde [US-023](https://github.com/ultraworkers/claw-code/commit/cec8d17). Para acceder a Kimi mediante OpenCode GO, utiliza siempre la forma explícita `opencode-go/kimi-k2.5`.

## Cómo funciona el enrutamiento
`claw-code` enruta las solicitudes con el prefijo `opencode-go/` a `OPENCODE_GO_BASE_URL` utilizando `OPENCODE_GO_API_KEY`. Los nombres directos de GLM / MiMo / MiniMax / Qwen3.x-plus se resuelven mediante `MODEL_REGISTRY` y apuntan al mismo endpoint. Los nombres `kimi-*` sin prefijo siguen enrutándose a DashScope (comportamiento del upstream).

## Limitaciones conocidas
- Los marcos de costos de transmisión se gestionan de forma transparente.
- Los modelos GLM emiten `reasoning_content` antes que `content`; el cliente procesa esto como alternativa.
- El soporte para uso de herramientas varía. Consulta el panel de OpenCode GO para obtener las cifras actuales.

## Lectura adicional
- [Documentación oficial de OpenCode GO](https://opencode.ai/docs/pt-br/go)
- [README principal de claw-code](https://github.com/ultraworkers/claw-code)
- [Lista de proveedores compatibles](../README.md#providers)
