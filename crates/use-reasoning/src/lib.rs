#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        ReasoningArtifactKind, ReasoningEffort, ReasoningError, ReasoningErrorKind, ReasoningMode,
        ReasoningStepKind, ReasoningStrategy, ReasoningTraceStatus, ReasoningVisibility,
    };
}

macro_rules! reasoning_enum {
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
            type Err = ReasoningError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(ReasoningError::UnknownLabel),
                }
            }
        }
    };
}

reasoning_enum!(ReasoningMode {
    None => "none",
    Direct => "direct",
    ChainOfThoughtLike => "chain-of-thought-like",
    TreeOfThoughtLike => "tree-of-thought-like",
    GraphOfThoughtLike => "graph-of-thought-like",
    ProgramAided => "program-aided",
    ToolAugmented => "tool-augmented",
    Reflective => "reflective",
    Custom => "custom",
});

reasoning_enum!(ReasoningVisibility {
    Hidden => "hidden",
    SummaryOnly => "summary-only",
    UserVisible => "user-visible",
    Redacted => "redacted",
});

reasoning_enum!(ReasoningStepKind {
    Observe => "observe",
    Decompose => "decompose",
    Infer => "infer",
    Retrieve => "retrieve",
    Calculate => "calculate",
    Compare => "compare",
    Verify => "verify",
    Reflect => "reflect",
    Decide => "decide",
    Explain => "explain",
    Custom => "custom",
});

reasoning_enum!(ReasoningTraceStatus {
    Unavailable => "unavailable",
    Available => "available",
    Redacted => "redacted",
    Summarized => "summarized",
    Disallowed => "disallowed",
});

reasoning_enum!(ReasoningEffort {
    Minimal => "minimal",
    Low => "low",
    Medium => "medium",
    High => "high",
    Max => "max",
    Unknown => "unknown",
});

reasoning_enum!(ReasoningStrategy {
    Deductive => "deductive",
    Inductive => "inductive",
    Abductive => "abductive",
    Analogical => "analogical",
    Causal => "causal",
    Probabilistic => "probabilistic",
    Geometric => "geometric",
    Symbolic => "symbolic",
    Custom => "custom",
});

reasoning_enum!(ReasoningArtifactKind {
    Summary => "summary",
    Scratchpad => "scratchpad",
    ProofSketch => "proof-sketch",
    Calculation => "calculation",
    Plan => "plan",
    Diagram => "diagram",
    Code => "code",
    CitationMap => "citation-map",
    Custom => "custom",
});

reasoning_enum!(ReasoningErrorKind {
    UnsupportedPremise => "unsupported-premise",
    MissingEvidence => "missing-evidence",
    Contradiction => "contradiction",
    Hallucination => "hallucination",
    ToolError => "tool-error",
    CalculationError => "calculation-error",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReasoningError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for ReasoningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("reasoning metadata label cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown reasoning metadata label"),
        }
    }
}

impl Error for ReasoningError {}

fn normalized_label(value: &str) -> Result<String, ReasoningError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(ReasoningError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ReasoningArtifactKind, ReasoningEffort, ReasoningError, ReasoningErrorKind, ReasoningMode,
        ReasoningStepKind, ReasoningStrategy, ReasoningTraceStatus, ReasoningVisibility,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), ReasoningError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = ReasoningError>,
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
    fn displays_and_parses_reasoning_enums() -> Result<(), ReasoningError> {
        assert_enum_family(ReasoningMode::ALL)?;
        assert_enum_family(ReasoningVisibility::ALL)?;
        assert_enum_family(ReasoningStepKind::ALL)?;
        assert_enum_family(ReasoningTraceStatus::ALL)?;
        assert_enum_family(ReasoningEffort::ALL)?;
        assert_enum_family(ReasoningStrategy::ALL)?;
        assert_enum_family(ReasoningArtifactKind::ALL)?;
        assert_enum_family(ReasoningErrorKind::ALL)?;
        assert_eq!(
            "tool augmented".parse::<ReasoningMode>()?,
            ReasoningMode::ToolAugmented
        );
        assert_eq!("".parse::<ReasoningMode>(), Err(ReasoningError::Empty));
        Ok(())
    }
}
