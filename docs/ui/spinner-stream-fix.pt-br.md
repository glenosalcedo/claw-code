# Spinner preserva a saída do stream

## Problema

Durante um turno, `run_turn` imprime um spinner animado na linha atual
do terminal (`⠋ 🦀 Thinking...`) e depois chama `spinner.finish(...)`
para substituí-lo por `✔ ✨ Done`.

`finish()` executa:

```text
MoveToColumn(0)
Clear(ClearType::CurrentLine)
Print("✔ ✨ Done\n")
```

Limpar a linha está correto **quando nenhum conteúdo foi streamado**:
a linha do spinner é a linha atual e é substituída direitinho.

É **incorreto quando o stream já escreveu conteúdo**, porque o stream
usa o mesmo stdout que o spinner. Quando um modelo emite uma resposta
curta começando com newlines (caso observado com
`opencode-go/minimax-m2.7`, que começa toda resposta com `\n\n`), o
terminal no momento em que `finish()` roda está assim:

```text
<prompt>                 <- cursor foi salvo aqui
⠋ 🦀 Thinking...         <- linha do spinner (ficou pra trás)
                         <- primeiro '\n' do stream
<resposta aqui>          <- cursor está aqui; Clear(CurrentLine) apaga isto
```

Resultado: o usuário vê o spinner sumir e aparecer `✔ ✨ Done`, mas a
resposta final desapareceu.

## Correção

Adicionar `Spinner::finish_after_stream(...)` que imprime um newline
antes do marcador de conclusão em vez de limpar a linha atual:

```text
Print("\n")
Print("✔ ✨ Done\n")
```

`run_turn` escolhe a variante depois que o turno termina:

- `final_assistant_text(&summary).is_empty()` → `finish(...)` (conteúdo
  nunca chegou no terminal; limpa a linha do spinner como antes).
- Caso contrário → `finish_after_stream(...)` (conteúdo está no
  terminal; não apaga).

Isso preserva a aparência original para turnos só-ferramenta (sem
texto do assistant) e corrige a regressão para respostas curtas que
foram streamadas.

## Por que `final_assistant_text(&summary)` em vez de uma flag no cliente

O consumidor de stream do rusty-claude-cli
(`AnthropicRuntimeClient::consume_stream`) já constrói uma lista de
`AssistantEvent::TextDelta` que o runtime agrega em
`summary.assistant_messages`. Ler isso depois do turno mantém a lógica
do spinner síncrona com o resto do ciclo de vida do turno — sem flag
atômica compartilhada e sem race com o último chunk do stream.

## Testes

`render.rs`:

- `finish_clears_current_line_for_empty_stream` — confirma que o
  caminho original ainda emite `\x1b[2K` (Clear(CurrentLine)).
- `finish_after_stream_does_not_clear_current_line` — confirma que o
  novo caminho nunca emite `\x1b[2K` e começa com `\n`.

## Modelos afetados

Qualquer modelo cujo primeiro chunk de conteúdo comece com
espaço/newline **E** cujo texto streamado total caiba em uma linha do
terminal. Observado:

- `opencode-go/minimax-m2.7` — sempre emite `\n\n<resposta>`
- `opencode-go/kimi-k2.5` — quando o turno é puramente raciocínio com
  um bloco de conteúdo final curto

Turnos só-ferramenta (sem texto streamado) não são afetados.
