# use-ai-role

AI role and participant metadata primitives for `RustUse`.

## Experimental

`use-ai-role` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_role::{AiInstructionAuthority, AiParticipantId, AiRoleName};

let role = AiRoleName::new("assistant")?;
let participant = AiParticipantId::new("participant-001")?;

assert_eq!(role.as_str(), "assistant");
assert_eq!(participant.value(), "participant-001");
assert_eq!("system".parse::<AiInstructionAuthority>()?, AiInstructionAuthority::System);
# Ok::<(), use_ai_role::AiRoleError>(())
```

## Scope

- Role names, participant identifiers, participant kinds, authority labels, role scopes, personas, and status labels.
- Metadata only.

## Non-goals

- Authorization, access control, instruction merging, policy enforcement, or runtime identity management.

## License

Licensed under either Apache-2.0 or MIT.