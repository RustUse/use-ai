# use-ai-eval

AI evaluation metadata primitives for `RustUse`.

## Experimental

`use-ai-eval` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_eval::{AiEvalKind, AiEvalRunId, AiEvalScore};

let run = AiEvalRunId::new("eval-001")?;
let score = AiEvalScore::new(0.92)?;

assert_eq!(run.as_str(), "eval-001");
assert_eq!(score.value(), 0.92);
assert_eq!("tool use eval".parse::<AiEvalKind>()?, AiEvalKind::ToolUseEval);
# Ok::<(), use_ai_eval::AiEvalError>(())
```

## Scope

- Eval run identifiers, rubric names, bounded scores, eval kinds, targets, judges, metrics, datasets, outcomes, and failure modes.
- Metadata only.

## Non-goals

- Running evals, judging outputs, calling models, scoring datasets, collecting latency, or calculating costs.

## License

Licensed under either Apache-2.0 or MIT.