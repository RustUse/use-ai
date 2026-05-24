# use-ai-model

AI model metadata primitives for `RustUse`.

## Experimental

`use-ai-model` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_model::{AiModelContextWindow, AiModelKind, AiModelName};

let name = AiModelName::new("reasoning-chat")?;
let context = AiModelContextWindow::new(128_000)?;

assert_eq!(name.as_str(), "reasoning-chat");
assert_eq!(context.value(), 128_000);
assert_eq!("image generation".parse::<AiModelKind>()?, AiModelKind::ImageGeneration);
# Ok::<(), use_ai_model::AiModelError>(())
```

## Scope

- Model names, identifiers, families, context windows, output limits, kind, modality, interface, reasoning, deployment, and lifecycle labels.
- Metadata only.

## Non-goals

- Loading models, routing requests, tokenization, pricing, inference, embeddings, or provider calls.

## License

Licensed under either Apache-2.0 or MIT.