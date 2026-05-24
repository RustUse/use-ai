# use-tool-call

Tool-call metadata primitives for `RustUse` AI workflows.

## Experimental

`use-tool-call` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_tool_call::{ToolCallKind, ToolCallStatus, ToolName};

let name = ToolName::new("ticket-search")?;

assert_eq!(name.as_str(), "ticket-search");
assert_eq!("tool-result".parse::<ToolCallKind>().is_err(), true);
assert_eq!(ToolCallStatus::Succeeded.as_str(), "succeeded");
# Ok::<(), use_tool_call::ToolCallError>(())
```

## Scope

- Tool names, call identifiers, argument names, call status, tool kinds, argument kinds, result kinds, schema kinds, choices, and error labels.
- Metadata only.

## Non-goals

- Executing tools, shell commands, browser automation, API calls, validation, authorization, or retry logic.

## License

Licensed under either Apache-2.0 or MIT.