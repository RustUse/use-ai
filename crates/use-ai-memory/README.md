# use-ai-memory

AI memory metadata primitives for `RustUse`.

## Experimental

`use-ai-memory` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_memory::{AiMemoryConfidence, AiMemoryId, AiMemoryKind};

let id = AiMemoryId::new("memory-001")?;
let confidence = AiMemoryConfidence::new(0.75)?;

assert_eq!(id.as_str(), "memory-001");
assert_eq!(confidence.value(), 0.75);
assert_eq!("policy controlled".parse::<use_ai_memory::AiMemoryRetentionKind>()?, use_ai_memory::AiMemoryRetentionKind::PolicyControlled);
assert_eq!(AiMemoryKind::Preference.as_str(), "preference");
# Ok::<(), use_ai_memory::AiMemoryError>(())
```

## Scope

- Memory identifiers, confidence scores, kinds, scopes, status, sources, retention, sensitivity, operation, and conflict labels.
- Metadata only.

## Non-goals

- Storing, retrieving, merging, inferring, deleting, suppressing, or enforcing memory records.

## License

Licensed under either Apache-2.0 or MIT.