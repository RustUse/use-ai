#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiCapabilityError, AiCapabilityKind, AiCapabilityName, AiCapabilityStatus,
        AiMemoryCapability, AiModalitySupport, AiReasoningCapability, AiSafetyCapability,
        AiStreamingSupport, AiStructuredOutputSupport, AiToolUseSupport,
    };
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AiCapabilityName(String);

impl AiCapabilityName {
    pub fn new(value: impl AsRef<str>) -> Result<Self, AiCapabilityError> {
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

impl AsRef<str> for AiCapabilityName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AiCapabilityName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AiCapabilityName {
    type Err = AiCapabilityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for AiCapabilityName {
    type Error = AiCapabilityError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

macro_rules! capability_enum {
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
            type Err = AiCapabilityError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiCapabilityError::UnknownLabel),
                }
            }
        }
    };
}

capability_enum!(AiCapabilityKind {
    TextGeneration => "text-generation",
    Chat => "chat",
    ToolUse => "tool-use",
    StructuredOutput => "structured-output",
    JsonMode => "json-mode",
    Vision => "vision",
    AudioInput => "audio-input",
    AudioOutput => "audio-output",
    ImageGeneration => "image-generation",
    ImageEditing => "image-editing",
    Embedding => "embedding",
    Reranking => "reranking",
    Reasoning => "reasoning",
    Realtime => "realtime",
    Batch => "batch",
    Moderation => "moderation",
    Custom => "custom",
});

capability_enum!(AiCapabilityStatus {
    Supported => "supported",
    Unsupported => "unsupported",
    Preview => "preview",
    Deprecated => "deprecated",
    Unknown => "unknown",
});

capability_enum!(AiModalitySupport {
    None => "none",
    Input => "input",
    Output => "output",
    InputOutput => "input-output",
    Unknown => "unknown",
});

capability_enum!(AiToolUseSupport {
    None => "none",
    SingleToolCall => "single-tool-call",
    ParallelToolCalls => "parallel-tool-calls",
    RequiredToolChoice => "required-tool-choice",
    ToolChoice => "tool-choice",
    Unknown => "unknown",
});

capability_enum!(AiStreamingSupport {
    None => "none",
    Text => "text",
    Events => "events",
    Audio => "audio",
    Multimodal => "multimodal",
    Unknown => "unknown",
});

capability_enum!(AiStructuredOutputSupport {
    None => "none",
    Json => "json",
    JsonSchema => "json-schema",
    Grammar => "grammar",
    Custom => "custom",
});

capability_enum!(AiSafetyCapability {
    Moderation => "moderation",
    Refusal => "refusal",
    PolicyFilter => "policy-filter",
    Citation => "citation",
    Grounding => "grounding",
    PiiRedaction => "pii-redaction",
    Unknown => "unknown",
});

capability_enum!(AiReasoningCapability {
    None => "none",
    Hidden => "hidden",
    Summary => "summary",
    Stepwise => "stepwise",
    ToolAugmented => "tool-augmented",
    Unknown => "unknown",
});

capability_enum!(AiMemoryCapability {
    None => "none",
    Session => "session",
    LongTerm => "long-term",
    External => "external",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiCapabilityError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for AiCapabilityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI capability metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown AI capability metadata label"),
        }
    }
}

impl Error for AiCapabilityError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiCapabilityError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiCapabilityError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiCapabilityError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiCapabilityError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiCapabilityError, AiCapabilityKind, AiCapabilityName, AiCapabilityStatus,
        AiMemoryCapability, AiModalitySupport, AiReasoningCapability, AiSafetyCapability,
        AiStreamingSupport, AiStructuredOutputSupport, AiToolUseSupport,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiCapabilityError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiCapabilityError>,
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
    fn validates_capability_names() -> Result<(), AiCapabilityError> {
        let name = AiCapabilityName::new(" tool-use ")?;

        assert_eq!(name.as_str(), "tool-use");
        assert_eq!(name.value(), "tool-use");
        assert_eq!(name.as_ref(), "tool-use");
        assert_eq!(name.to_string(), "tool-use");
        assert_eq!(AiCapabilityName::try_from("tool-use")?, name);
        assert_eq!(name.into_string(), "tool-use".to_string());
        assert_eq!(AiCapabilityName::new("  "), Err(AiCapabilityError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_capability_enums() -> Result<(), AiCapabilityError> {
        assert_enum_family(AiCapabilityKind::ALL)?;
        assert_enum_family(AiCapabilityStatus::ALL)?;
        assert_enum_family(AiModalitySupport::ALL)?;
        assert_enum_family(AiToolUseSupport::ALL)?;
        assert_enum_family(AiStreamingSupport::ALL)?;
        assert_enum_family(AiStructuredOutputSupport::ALL)?;
        assert_enum_family(AiSafetyCapability::ALL)?;
        assert_enum_family(AiReasoningCapability::ALL)?;
        assert_enum_family(AiMemoryCapability::ALL)?;
        assert_eq!(
            "structured output".parse::<AiCapabilityKind>()?,
            AiCapabilityKind::StructuredOutput
        );
        Ok(())
    }
}
