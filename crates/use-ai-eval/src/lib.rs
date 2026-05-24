#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiEvalDatasetKind, AiEvalError, AiEvalFailureMode, AiEvalJudgeKind, AiEvalKind,
        AiEvalMetricKind, AiEvalOutcome, AiEvalRubricName, AiEvalRunId, AiEvalScore,
        AiEvalTargetKind,
    };
}

macro_rules! eval_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AiEvalError> {
                non_empty_text(value).map(Self)
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }

            pub fn value(&self) -> &str {
                self.as_str()
            }

            pub fn into_string(self) -> String {
                self.0
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                self.as_str()
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = AiEvalError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AiEvalError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! eval_enum {
    ($name:ident { $($variant:ident => $label:literal),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub const ALL: &'static [Self] = &[$(Self::$variant),+];

            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $label),+
                }
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(self.as_str())
            }
        }

        impl FromStr for $name {
            type Err = AiEvalError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiEvalError::UnknownLabel),
                }
            }
        }
    };
}

eval_text_newtype!(AiEvalRunId);
eval_text_newtype!(AiEvalRubricName);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AiEvalScore(f64);

impl AiEvalScore {
    pub fn new(value: f64) -> Result<Self, AiEvalError> {
        if !value.is_finite() {
            return Err(AiEvalError::NonFinite);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(AiEvalError::OutOfRange);
        }
        Ok(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

eval_enum!(AiEvalKind {
    PromptEval => "prompt-eval",
    ResponseEval => "response-eval",
    ConversationEval => "conversation-eval",
    ToolUseEval => "tool-use-eval",
    AgentEval => "agent-eval",
    RagEval => "rag-eval",
    SafetyEval => "safety-eval",
    RegressionEval => "regression-eval",
    HumanEval => "human-eval",
    Custom => "custom",
});

eval_enum!(AiEvalTargetKind {
    Prompt => "prompt",
    ModelResponse => "model-response",
    Conversation => "conversation",
    Agent => "agent",
    ToolCall => "tool-call",
    RagPipeline => "rag-pipeline",
    Guardrail => "guardrail",
    Memory => "memory",
    Custom => "custom",
});

eval_enum!(AiEvalJudgeKind {
    Human => "human",
    Model => "model",
    Rule => "rule",
    Heuristic => "heuristic",
    GoldenAnswer => "golden-answer",
    Pairwise => "pairwise",
    Consensus => "consensus",
    Custom => "custom",
});

eval_enum!(AiEvalMetricKind {
    Helpfulness => "helpfulness",
    Correctness => "correctness",
    Faithfulness => "faithfulness",
    Groundedness => "groundedness",
    Relevance => "relevance",
    InstructionFollowing => "instruction-following",
    Safety => "safety",
    RefusalQuality => "refusal-quality",
    Toxicity => "toxicity",
    Bias => "bias",
    CitationQuality => "citation-quality",
    ToolUseCorrectness => "tool-use-correctness",
    Latency => "latency",
    Cost => "cost",
    Custom => "custom",
});

eval_enum!(AiEvalDatasetKind {
    GoldenSet => "golden-set",
    RedTeamSet => "red-team-set",
    RegressionSet => "regression-set",
    ConversationSet => "conversation-set",
    RetrievalSet => "retrieval-set",
    Synthetic => "synthetic",
    ProductionSample => "production-sample",
    Custom => "custom",
});

eval_enum!(AiEvalOutcome {
    Passed => "passed",
    Failed => "failed",
    Warning => "warning",
    Inconclusive => "inconclusive",
    Error => "error",
});

eval_enum!(AiEvalFailureMode {
    Hallucination => "hallucination",
    UngroundedAnswer => "ungrounded-answer",
    BadCitation => "bad-citation",
    ToolError => "tool-error",
    UnsafeOutput => "unsafe-output",
    PolicyViolation => "policy-violation",
    RefusalFailure => "refusal-failure",
    OverRefusal => "over-refusal",
    FormatFailure => "format-failure",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiEvalError {
    Empty,
    NonFinite,
    OutOfRange,
    UnknownLabel,
}

impl fmt::Display for AiEvalError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI eval metadata text cannot be empty"),
            Self::NonFinite => formatter.write_str("AI eval score must be finite"),
            Self::OutOfRange => formatter.write_str("AI eval score must be in 0.0..=1.0"),
            Self::UnknownLabel => formatter.write_str("unknown AI eval metadata label"),
        }
    }
}

impl Error for AiEvalError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiEvalError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiEvalError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiEvalError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiEvalError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiEvalDatasetKind, AiEvalError, AiEvalFailureMode, AiEvalJudgeKind, AiEvalKind,
        AiEvalMetricKind, AiEvalOutcome, AiEvalRubricName, AiEvalRunId, AiEvalScore,
        AiEvalTargetKind,
    };
    use core::{fmt, str::FromStr};

    macro_rules! assert_text_newtype {
        ($type:ty, $value:literal) => {{
            let value = <$type>::new(concat!(" ", $value, " "))?;
            assert_eq!(value.as_str(), $value);
            assert_eq!(value.value(), $value);
            assert_eq!(value.as_ref(), $value);
            assert_eq!(value.to_string(), $value);
            assert_eq!(<$type as TryFrom<&str>>::try_from($value)?, value);
            assert_eq!(value.into_string(), $value.to_string());
        }};
    }

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiEvalError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiEvalError>,
    {
        for variant in variants {
            let label = variant.to_string();
            assert_eq!(label.parse::<T>()?, *variant);
            assert_eq!(label.replace('-', "_").parse::<T>()?, *variant);
            assert_eq!(label.replace('-', " ").parse::<T>()?, *variant);
        }
        Ok(())
    }

    #[test]
    fn validates_eval_text_newtypes() -> Result<(), AiEvalError> {
        assert_text_newtype!(AiEvalRunId, "eval-001");
        assert_text_newtype!(AiEvalRubricName, "helpfulness");
        assert_eq!(AiEvalRunId::new("  "), Err(AiEvalError::Empty));
        Ok(())
    }

    #[test]
    fn validates_eval_scores() -> Result<(), AiEvalError> {
        assert_eq!(AiEvalScore::new(0.0)?.value(), 0.0);
        assert_eq!(AiEvalScore::new(1.0)?.value(), 1.0);
        assert_eq!(AiEvalScore::new(-0.1), Err(AiEvalError::OutOfRange));
        assert_eq!(AiEvalScore::new(1.1), Err(AiEvalError::OutOfRange));
        assert_eq!(AiEvalScore::new(f64::INFINITY), Err(AiEvalError::NonFinite));
        Ok(())
    }

    #[test]
    fn displays_and_parses_eval_enums() -> Result<(), AiEvalError> {
        assert_enum_family(AiEvalKind::ALL)?;
        assert_enum_family(AiEvalTargetKind::ALL)?;
        assert_enum_family(AiEvalJudgeKind::ALL)?;
        assert_enum_family(AiEvalMetricKind::ALL)?;
        assert_enum_family(AiEvalDatasetKind::ALL)?;
        assert_enum_family(AiEvalOutcome::ALL)?;
        assert_enum_family(AiEvalFailureMode::ALL)?;
        assert_eq!(
            "tool use eval".parse::<AiEvalKind>()?,
            AiEvalKind::ToolUseEval
        );
        Ok(())
    }
}
