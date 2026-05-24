# use-reasoning

AI reasoning metadata primitives for `RustUse`.

## Experimental

`use-reasoning` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_reasoning::{ReasoningMode, ReasoningVisibility};

assert_eq!("tool augmented".parse::<ReasoningMode>()?, ReasoningMode::ToolAugmented);
assert_eq!(ReasoningVisibility::SummaryOnly.as_str(), "summary-only");
# Ok::<(), use_reasoning::ReasoningError>(())
```

## Scope

- Reasoning modes, visibility, step kinds, trace status, effort, strategies, artifacts, and error labels.
- Metadata only.

## Non-goals

- Producing chain-of-thought, calculating answers, executing programs, verifying proofs, or exposing hidden traces.

## License

Licensed under either Apache-2.0 or MIT.