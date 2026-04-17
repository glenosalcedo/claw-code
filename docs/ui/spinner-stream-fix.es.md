# El spinner conserva la salida del stream

## Problema

Durante un turno, `run_turn` imprime un spinner animado en la línea
actual del terminal (`⠋ 🦀 Thinking...`) y después llama
`spinner.finish(...)` para reemplazarlo por `✔ ✨ Done`.

`finish()` ejecuta:

```text
MoveToColumn(0)
Clear(ClearType::CurrentLine)
Print("✔ ✨ Done\n")
```

Limpiar la línea es correcto **cuando todavía no se ha transmitido
contenido**: la línea del spinner es la línea actual y se reemplaza
limpiamente.

Es **incorrecto cuando el stream ya escribió contenido**, porque el
stream usa el mismo stdout que el spinner. Cuando un modelo emite una
respuesta corta que empieza con saltos de línea (caso observado con
`opencode-go/minimax-m2.7`, que empieza cada respuesta con `\n\n`), el
diseño del terminal en el momento en que `finish()` se ejecuta es:

```text
<prompt>                 <- el cursor fue guardado aquí
⠋ 🦀 Thinking...         <- línea del spinner (quedó atrás)
                         <- primer '\n' del stream
<respuesta aquí>         <- el cursor está aquí; Clear(CurrentLine) lo borra
```

Resultado: el usuario ve desaparecer el spinner y aparecer
`✔ ✨ Done`, pero la respuesta final no está.

## Corrección

Añadir `Spinner::finish_after_stream(...)` que imprime un salto de
línea antes del marcador de finalización en lugar de limpiar la línea
actual:

```text
Print("\n")
Print("✔ ✨ Done\n")
```

`run_turn` elige la variante después de que termine el turno:

- `final_assistant_text(&summary).is_empty()` → `finish(...)` (el
  contenido nunca llegó al terminal; limpia la línea del spinner como
  antes).
- En otro caso → `finish_after_stream(...)` (el contenido está en el
  terminal; no lo borres).

Esto preserva la apariencia original para turnos que solo usan
herramientas (sin texto del asistente) y corrige la regresión para
respuestas cortas transmitidas.

## Por qué `final_assistant_text(&summary)` en lugar de una bandera en el cliente

El consumidor de stream de rusty-claude-cli
(`AnthropicRuntimeClient::consume_stream`) ya construye una lista de
`AssistantEvent::TextDelta` que el runtime agrega en
`summary.assistant_messages`. Leer esto después del turno mantiene la
lógica del spinner sincronizada con el resto del ciclo de vida del
turno: sin bandera atómica compartida ni carrera con el último chunk
del stream.

## Tests

`render.rs`:

- `finish_clears_current_line_for_empty_stream` — confirma que el
  camino original sigue emitiendo `\x1b[2K` (Clear(CurrentLine)).
- `finish_after_stream_does_not_clear_current_line` — confirma que el
  nuevo camino nunca emite `\x1b[2K` y empieza con `\n`.

## Modelos afectados

Cualquier modelo cuyo primer chunk de contenido empiece con
espacios/saltos de línea **Y** cuyo texto transmitido total quepa en
una sola línea del terminal. Observado:

- `opencode-go/minimax-m2.7` — siempre emite `\n\n<respuesta>`
- `opencode-go/kimi-k2.5` — cuando el turno es pura reflexión con un
  bloque de contenido final corto

Los turnos que solo usan herramientas (sin texto transmitido) no se
ven afectados.
