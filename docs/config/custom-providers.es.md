# Proveedores personalizados

## Descripción general
Define endpoints de IA externos directamente en tu configuración. Usa esta función cuando necesites enrutar solicitudes a APIs de terceros o modelos autoalojados que no estén en el registro predeterminado.

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

## Referencia de campos
| Campo | Tipo | ¿Obligatorio? | Descripción |
|---|---|---|---|
| `prefix` (map key) | string | Sí | Identificador único usado para enrutar solicitudes. |
| `base_url` | string | Sí | El endpoint de API compatible con OpenAI. |
| `api_key_env` | string | Sí | Nombre de la variable de entorno que contiene el token de autenticación. |
| `models` | array | Sí | Lista de identificadores de modelo compatibles en este endpoint. |

## Comportamiento de enrutamiento
- La clave del mapa se convierte en el prefijo visible usado en `/model <prefix>/<model-id>`.
- Los proveedores personalizados se consultan DESPUÉS del builtin `MODEL_REGISTRY` (el builtin gana en caso de conflicto).
- Los modelos de todos los proveedores aparecen en la salida de `/model` + TAB y `/providers`.

## Ejemplo mínimo funcional
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

## Precedencia
Las entradas en `./.claw.json` del proyecto anulan las de `~/.claw.json` del usuario cuando ambos definen el mismo prefijo. Verifica siempre qué configuración está activa antes de solucionar problemas de enrutamiento.

## Reinicio necesario
`claw` almacena en caché los proveedores personalizados por proceso; reinicia la aplicación después de editar `.claw.json`.

## Véase también
- `docs/config/custom-provider-examples.md`
- `docs/providers/opencode-go.md`
- `schema/claw.schema.json`
