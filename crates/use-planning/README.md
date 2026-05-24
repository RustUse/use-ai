# use-planning

AI planning metadata primitives for `RustUse`.

## Experimental

`use-planning` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_planning::{PlanName, PlanPriority, PlanStepKind};

let name = PlanName::new("resolve-ticket")?;

assert_eq!(name.as_str(), "resolve-ticket");
assert_eq!("call tool".parse::<PlanStepKind>()?, PlanStepKind::CallTool);
assert_eq!(PlanPriority::Urgent.as_str(), "urgent");
# Ok::<(), use_planning::PlanningError>(())
```

## Scope

- Plan names, identifiers, step identifiers, step kinds, status, priority, dependencies, execution modes, review status, and risk labels.
- Metadata only.

## Non-goals

- Building plans, running steps, calling tools, reviewing outcomes, or enforcing execution constraints.

## License

Licensed under either Apache-2.0 or MIT.