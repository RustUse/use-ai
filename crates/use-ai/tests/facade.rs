use use_ai::{
    AgentName, AiCapabilityName, AiContextWindowSize, AiEvalRunId, AiMemoryId, AiMessageId,
    AiModelName, AiProviderName, AiRoleName, GuardrailName, PlanName, PromptName, RagCorpusName,
    ReasoningMode, ToolName,
};

#[test]
fn facade_reexports_every_child_crate() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = PromptName::new("support-triage")?;
    let message = AiMessageId::new("msg-001")?;
    let context_window = AiContextWindowSize::new(8_192)?;
    let role = AiRoleName::new("assistant")?;
    let model = AiModelName::new("reasoning-chat")?;
    let provider = AiProviderName::new("local-runtime")?;
    let capability = AiCapabilityName::new("tool-use")?;
    let tool = ToolName::new("ticket-search")?;
    let agent = AgentName::new("triage-agent")?;
    let plan = PlanName::new("resolve-ticket")?;
    let corpus = RagCorpusName::new("support-docs")?;
    let memory = AiMemoryId::new("memory-001")?;
    let guardrail = GuardrailName::new("pii-redaction")?;
    let eval = AiEvalRunId::new("eval-001")?;

    assert_eq!(prompt.as_str(), "support-triage");
    assert_eq!(message.as_str(), "msg-001");
    assert_eq!(context_window.value(), 8_192);
    assert_eq!(role.as_str(), "assistant");
    assert_eq!(model.as_str(), "reasoning-chat");
    assert_eq!(provider.as_str(), "local-runtime");
    assert_eq!(capability.as_str(), "tool-use");
    assert_eq!(tool.as_str(), "ticket-search");
    assert_eq!(agent.as_str(), "triage-agent");
    assert_eq!(ReasoningMode::Direct.as_str(), "direct");
    assert_eq!(plan.as_str(), "resolve-ticket");
    assert_eq!(corpus.as_str(), "support-docs");
    assert_eq!(memory.as_str(), "memory-001");
    assert_eq!(guardrail.as_str(), "pii-redaction");
    assert_eq!(eval.as_str(), "eval-001");
    Ok(())
}
