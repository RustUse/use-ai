# use-ai-context

AI context metadata primitives for `RustUse`.

## Experimental

`use-ai-context` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_context::{AiContextItemKind, AiContextWindow, AiContextWindowSize};

let size = AiContextWindowSize::new(8192)?;
let window = AiContextWindow::new(size);

assert_eq!(window.size().value(), 8192);
assert_eq!("web page".parse::<AiContextItemKind>()?, AiContextItemKind::WebPage);
# Ok::<(), use_ai_context::AiContextError>(())
```

## Scope

- Context identifiers, item identifiers, citations, window sizes, item kinds, sources, priorities, fit, and grounding labels.
- Metadata only.

## Non-goals

- Retrieval, summarization, truncation, storage, ranking, citation verification, or network access.

## License

Licensed under either Apache-2.0 or MIT.