# use-ai-capability

AI capability metadata primitives for `RustUse`.

## Experimental

`use-ai-capability` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_capability::{AiCapabilityKind, AiCapabilityName, AiCapabilityStatus};

let name = AiCapabilityName::new("tool-use")?;

assert_eq!(name.as_str(), "tool-use");
assert_eq!("structured output".parse::<AiCapabilityKind>()?, AiCapabilityKind::StructuredOutput);
assert_eq!(AiCapabilityStatus::Supported.as_str(), "supported");
# Ok::<(), use_ai_capability::AiCapabilityError>(())
```

## Scope

- Capability names and support labels for modality, tools, streaming, structured output, safety, reasoning, and memory.
- Metadata only.

## Non-goals

- Feature probing, provider calls, policy enforcement, safety checks, or runtime capability negotiation.

## License

Licensed under either Apache-2.0 or MIT.