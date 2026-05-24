# use-agent

AI agent metadata primitives for `RustUse`.

## Experimental

`use-agent` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_agent::{AgentKind, AgentName, AgentStatus};

let name = AgentName::new("triage-agent")?;

assert_eq!(name.as_str(), "triage-agent");
assert_eq!("waiting for tool".parse::<AgentStatus>()?, AgentStatus::WaitingForTool);
assert_eq!(AgentKind::Planner.as_str(), "planner");
# Ok::<(), use_agent::AgentError>(())
```

## Scope

- Agent names and identifiers, kinds, status, modes, autonomy levels, loop kinds, handoffs, observations, and actions.
- Metadata only.

## Non-goals

- Running agent loops, making plans, calling tools, memory access, retrieval, delegation, or user prompting.

## License

Licensed under either Apache-2.0 or MIT.