# use-guardrail

AI guardrail metadata primitives for `RustUse`.

## Experimental

`use-guardrail` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_guardrail::{GuardrailAction, GuardrailKind, GuardrailName};

let name = GuardrailName::new("pii-redaction")?;

assert_eq!(name.as_str(), "pii-redaction");
assert_eq!("require review".parse::<GuardrailAction>()?, GuardrailAction::RequireReview);
assert_eq!(GuardrailKind::ToolUse.as_str(), "tool-use");
# Ok::<(), use_guardrail::GuardrailError>(())
```

## Scope

- Guardrail names and identifiers plus kind, action, severity, status, policy area, check, result, and violation labels.
- Metadata only.

## Non-goals

- Moderation, redaction, policy checks, prompt-injection detection, enforcement, or human-review workflows.

## License

Licensed under either Apache-2.0 or MIT.