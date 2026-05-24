# RustUse/use-ai

Composable AI interaction primitives for `RustUse`.

`use-ai` is a focused RustUse set for representing prompts, messages, context
windows, roles, model and provider metadata, capabilities, tool calls, agents,
reasoning, planning, RAG metadata, memory metadata, guardrails, and AI-specific
evaluation as small Rust primitives. It is not an AI runtime, model client,
agent orchestrator, vector database, memory store, guardrail engine, or eval
runner.

## Workspace crates

| Crate               | Path                         | Purpose                                                        |
| ------------------- | ---------------------------- | -------------------------------------------------------------- |
| `use-ai`            | `crates/use-ai/`             | Facade over the focused AI interaction primitive crates        |
| `use-ai-prompt`     | `crates/use-ai-prompt/`      | Prompt names, text, variables, formats, parts, and status      |
| `use-ai-message`    | `crates/use-ai-message/`     | Message identity, roles, parts, formats, token counts, state   |
| `use-ai-context`    | `crates/use-ai-context/`     | Context items, citations, priorities, fit, and windows         |
| `use-ai-role`       | `crates/use-ai-role/`        | Participants, authority, role scopes, personas, and status     |
| `use-ai-model`      | `crates/use-ai-model/`       | AI model identity, modalities, interfaces, limits, lifecycle   |
| `use-ai-provider`   | `crates/use-ai-provider/`    | Provider, endpoint, quota, billing, API mode, and region labels|
| `use-ai-capability` | `crates/use-ai-capability/`  | Capability support labels for AI model and provider surfaces   |
| `use-tool-call`     | `crates/use-tool-call/`      | Tool-call names, arguments, result kinds, schemas, and status   |
| `use-agent`         | `crates/use-agent/`          | Agent identity, modes, loops, actions, observations, handoffs  |
| `use-reasoning`     | `crates/use-reasoning/`      | Reasoning modes, visibility, step kinds, effort, and artifacts |
| `use-planning`      | `crates/use-planning/`       | Plan identity, steps, status, dependencies, review, and risks  |
| `use-rag`           | `crates/use-rag/`            | RAG corpus, document, chunk, retrieval, ranking, and grounding |
| `use-ai-memory`     | `crates/use-ai-memory/`      | Memory identity, confidence, scope, retention, sensitivity     |
| `use-guardrail`     | `crates/use-guardrail/`      | Guardrail identity, checks, actions, results, and violations   |
| `use-ai-eval`       | `crates/use-ai-eval/`        | AI eval runs, rubrics, scores, datasets, metrics, and outcomes |

## Experimental

Every crate in this workspace is experimental while the release line remains
below `0.3.0`. Expect small API adjustments as the AI primitive surface settles.

## Relationship to use-ml

`use-ai` models AI interaction primitives: prompts, messages, roles, context
windows, tool calls, agents, RAG, reasoning, memory, guardrails, AI model
interfaces, and AI-specific evaluation.

`use-ml` models machine-learning primitives: datasets, features, labels,
tensors, model artifacts, training, inference, evaluation, metrics, pipelines,
embeddings, experiments, and model documentation.

These sets are siblings. They should interoperate conceptually but avoid
dependency cycles.

## Example

```rust
use use_ai::{AiMessageRole, AiModelName, PromptFormat, PromptName, ToolName};

let prompt = PromptName::new("support-triage")?;
let model = AiModelName::new("reasoning-chat")?;
let tool = ToolName::new("ticket-search")?;

assert_eq!(prompt.as_str(), "support-triage");
assert_eq!(model.value(), "reasoning-chat");
assert_eq!(tool.as_str(), "ticket-search");
assert_eq!(AiMessageRole::Assistant.as_str(), "assistant");
assert_eq!("plain text".parse::<PromptFormat>()?, PromptFormat::PlainText);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- Validated identifiers, names, text references, finite scores, bounded counts,
  and positive limits.
- Small enums for prompts, messages, context, roles, model metadata, providers,
  capabilities, tools, agents, reasoning, planning, RAG, memory, guardrails, and
  AI evaluation.
- Metadata-only primitives suitable for application glue code, docs tooling,
  examples, CLIs, and test fixtures.

## v0.2 follow-ups

Keep future `use-ai` work focused on AI interaction metadata. Good follow-up
candidates include optional `serde` support, richer schema descriptors,
provider-neutral request metadata, optional `no_std` compatibility where
practical, and conversion helpers between sibling crates where they do not
create dependency cycles.

## Non-goals

- Calling models, providers, local runtimes, model hubs, cloud services, or APIs.
- Running tools, shell commands, retrieval, embeddings, guardrails, memory
  storage, evals, planners, or agent loops.
- Training models, serving models, implementing tensor math, allocating vectors,
  or computing embedding values.
- Shelling out to Python, Node.js, model CLIs, cloud CLIs, vector databases,
  notebooks, package registries, or external services.

## Development

```sh
cargo fmt
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
```

For release-readiness checks, also run the repository tasks in the `Makefile`.

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
