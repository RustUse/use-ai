# use-ai-message

AI message metadata primitives for `RustUse`.

## Experimental

`use-ai-message` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_message::{AiMessageId, AiMessageRole, AiMessageTokenCount};

let id = AiMessageId::new("msg-001")?;
let tokens = AiMessageTokenCount::new(0);

assert_eq!(id.as_str(), "msg-001");
assert_eq!(tokens.value(), 0);
assert_eq!("assistant".parse::<AiMessageRole>()?, AiMessageRole::Assistant);
# Ok::<(), use_ai_message::AiMessageError>(())
```

## Scope

- Message identifiers, conversation identifiers, content references, token counts, roles, parts, formats, visibility, source, and status labels.
- Metadata only.

## Non-goals

- Sending messages, streaming tokens, redacting content, storing conversations, or calling models.

## License

Licensed under either Apache-2.0 or MIT.