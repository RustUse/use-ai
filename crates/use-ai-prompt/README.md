# use-ai-prompt

AI prompt metadata primitives for `RustUse`.

## Experimental

`use-ai-prompt` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_prompt::{PromptFormat, PromptName, PromptTemplate, PromptText};

let name = PromptName::new("support-triage")?;
let text = PromptText::new("Classify the support request")?;
let template = PromptTemplate::new(name, text, PromptFormat::Markdown);

assert_eq!(template.name().as_str(), "support-triage");
assert_eq!(template.text().value(), "Classify the support request");
assert_eq!(template.format(), PromptFormat::Markdown);
# Ok::<(), use_ai_prompt::AiPromptError>(())
```

## Scope

- Prompt names, identifiers, text, variables, parts, instructions, formats, and status labels.
- Prompt template metadata only.

## Non-goals

- Rendering prompts, calling models, loading files, executing tools, or enforcing policies.

## License

Licensed under either Apache-2.0 or MIT.