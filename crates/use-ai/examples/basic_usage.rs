use use_ai::{AiMessageRole, AiModelName, PromptName, ToolName};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = PromptName::new("support-triage")?;
    let model = AiModelName::new("reasoning-chat")?;
    let tool = ToolName::new("ticket-search")?;

    assert_eq!(prompt.as_str(), "support-triage");
    assert_eq!(model.as_str(), "reasoning-chat");
    assert_eq!(tool.as_str(), "ticket-search");
    assert_eq!(AiMessageRole::Assistant.as_str(), "assistant");
    Ok(())
}
