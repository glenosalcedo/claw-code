# Spinner preserves streamed output

## Problem

During a turn, `run_turn` prints an animated spinner on the current
terminal line (`⠋ 🦀 Thinking...`) and later calls `spinner.finish(...)`
to replace it with `✔ ✨ Done`.

`finish()` executes:

```text
MoveToColumn(0)
Clear(ClearType::CurrentLine)
Print("✔ ✨ Done\n")
```

The clear-line is correct **when no content has been streamed yet** —
the spinner line is the current line and gets replaced cleanly.

It is **incorrect when content has already been streamed**, because the
stream writes through the same stdout the spinner used. When a model
emits a short answer that starts with newlines (observed in
`opencode-go/minimax-m2.7`, which begins every reply with `\n\n`), the
layout at the time `finish()` runs is:

```text
<prompt>                 <- cursor was saved here
⠋ 🦀 Thinking...         <- spinner line (left behind)
                         <- first '\n' from stream
<streamed answer here>   <- where the cursor now sits; Clear(CurrentLine) erases this
```

Result: the user sees the spinner disappear and then `✔ ✨ Done`, but
the final answer is gone.

## Fix

Add `Spinner::finish_after_stream(...)` that prints a newline before the
done marker instead of clearing the current line:

```text
Print("\n")
Print("✔ ✨ Done\n")
```

`run_turn` picks the variant after the turn finishes:

- `final_assistant_text(&summary).is_empty()` → `finish(...)` (content
  never reached the terminal; clear the spinner line as before).
- Otherwise → `finish_after_stream(...)` (content is on the terminal;
  don't erase it).

This preserves the existing appearance for tool-only turns (no
assistant text) and fixes the regression for short streamed answers.

## Why `final_assistant_text(&summary)` rather than a flag on the client

The rusty-claude-cli stream consumer (`AnthropicRuntimeClient::consume_stream`)
already builds an `AssistantEvent::TextDelta` list that the runtime
aggregates into `summary.assistant_messages`. Reading that after the
turn keeps the spinner logic synchronous with the rest of the turn
lifecycle — no shared atomic flag, no race with the stream's last
chunk.

## Tests

`render.rs`:

- `finish_clears_current_line_for_empty_stream` — confirms the original
  path still emits `\x1b[2K` (Clear(CurrentLine)).
- `finish_after_stream_does_not_clear_current_line` — confirms the new
  path never emits `\x1b[2K` and starts with `\n`.

## Affected models

Any model whose first content chunk begins with whitespace/newlines
**and** whose total streamed text fits on one terminal line. Observed:

- `opencode-go/minimax-m2.7` — always emits `\n\n<answer>`
- `opencode-go/kimi-k2.5` — when the turn is purely reasoning with a
  short final content block

Tool-only turns (no streamed text) are unaffected.
