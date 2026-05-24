# use-ai

AI interaction primitive facade for `RustUse`.

## Experimental

`use-ai` is experimental while the release line remains below `0.3.0`.

## Example

```rust
use use_ai::{AiMessageRole, AiModelName, PromptName, ToolName};

let prompt = PromptName::new("support-triage")?;
let model = AiModelName::new("reasoning-chat")?;
let tool = ToolName::new("ticket-search")?;

assert_eq!(prompt.as_str(), "support-triage");
assert_eq!(model.value(), "reasoning-chat");
assert_eq!(tool.as_str(), "ticket-search");
assert_eq!(AiMessageRole::Assistant.as_str(), "assistant");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Facade re-exports for focused AI interaction metadata crates.
- Prompt, message, context, role, model, provider, capability, tool-call, agent,
  reasoning, planning, RAG, memory, guardrail, and AI eval primitives.

## Relationship to use-ml

`use-ai` models AI interaction primitives: prompts, messages, roles, context
windows, tool calls, agents, RAG, reasoning, memory, guardrails, AI model
interfaces, and AI-specific evaluation.

`use-ml` models machine-learning primitives: datasets, features, labels,
tensors, model artifacts, training, inference, evaluation, metrics, pipelines,
embeddings, experiments, and model documentation.

These sets are siblings. They should interoperate conceptually but avoid
dependency cycles.

## Non-goals

- Model calls, network calls, tool execution, retrieval, memory storage,
  guardrail enforcement, eval execution, or orchestration.

## License

Licensed under either Apache-2.0 or MIT.
