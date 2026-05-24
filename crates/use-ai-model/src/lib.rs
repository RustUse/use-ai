#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiModelContextWindow, AiModelDeploymentKind, AiModelError, AiModelFamily, AiModelId,
        AiModelInterfaceKind, AiModelKind, AiModelLifecycleStage, AiModelModality, AiModelName,
        AiModelOutputLimit, AiModelReasoningMode,
    };
}

macro_rules! model_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AiModelError> {
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
            type Err = AiModelError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AiModelError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! model_positive_u32 {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        impl $name {
            pub fn new(value: u32) -> Result<Self, AiModelError> {
                if value == 0 {
                    Err(AiModelError::Zero)
                } else {
                    Ok(Self(value))
                }
            }

            pub const fn value(self) -> u32 {
                self.0
            }

            pub const fn get(self) -> u32 {
                self.0
            }
        }
    };
}

macro_rules! model_enum {
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
            type Err = AiModelError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiModelError::UnknownLabel),
                }
            }
        }
    };
}

model_text_newtype!(AiModelName);
model_text_newtype!(AiModelId);
model_text_newtype!(AiModelFamily);
model_positive_u32!(AiModelContextWindow);
model_positive_u32!(AiModelOutputLimit);

model_enum!(AiModelKind {
    Chat => "chat",
    Completion => "completion",
    Embedding => "embedding",
    Reranker => "reranker",
    ImageGeneration => "image-generation",
    ImageEditing => "image-editing",
    SpeechToText => "speech-to-text",
    TextToSpeech => "text-to-speech",
    Multimodal => "multimodal",
    ToolUsing => "tool-using",
    Reasoning => "reasoning",
    Custom => "custom",
});

model_enum!(AiModelModality {
    Text => "text",
    Image => "image",
    Audio => "audio",
    Video => "video",
    Code => "code",
    Embedding => "embedding",
    Multimodal => "multimodal",
    Custom => "custom",
});

model_enum!(AiModelInterfaceKind {
    ChatCompletions => "chat-completions",
    Responses => "responses",
    Completions => "completions",
    Embeddings => "embeddings",
    Realtime => "realtime",
    Batch => "batch",
    Custom => "custom",
});

model_enum!(AiModelReasoningMode {
    None => "none",
    Hidden => "hidden",
    SummaryOnly => "summary-only",
    VisibleScratchpad => "visible-scratchpad",
    ToolAugmented => "tool-augmented",
    Unknown => "unknown",
});

model_enum!(AiModelDeploymentKind {
    HostedApi => "hosted-api",
    SelfHosted => "self-hosted",
    Local => "local",
    Edge => "edge",
    Browser => "browser",
    Embedded => "embedded",
    Unknown => "unknown",
});

model_enum!(AiModelLifecycleStage {
    Experimental => "experimental",
    Preview => "preview",
    Stable => "stable",
    Deprecated => "deprecated",
    Retired => "retired",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiModelError {
    Empty,
    Zero,
    UnknownLabel,
}

impl fmt::Display for AiModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI model metadata text cannot be empty"),
            Self::Zero => formatter.write_str("AI model numeric value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown AI model metadata label"),
        }
    }
}

impl Error for AiModelError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiModelError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiModelError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiModelError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiModelError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiModelContextWindow, AiModelDeploymentKind, AiModelError, AiModelFamily, AiModelId,
        AiModelInterfaceKind, AiModelKind, AiModelLifecycleStage, AiModelModality, AiModelName,
        AiModelOutputLimit, AiModelReasoningMode,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiModelError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiModelError>,
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
    fn validates_model_text_newtypes() -> Result<(), AiModelError> {
        assert_text_newtype!(AiModelName, "reasoning-chat");
        assert_text_newtype!(AiModelId, "model-001");
        assert_text_newtype!(AiModelFamily, "reasoning");
        assert_eq!(AiModelName::new("  "), Err(AiModelError::Empty));
        Ok(())
    }

    #[test]
    fn validates_model_limits() -> Result<(), AiModelError> {
        let context = AiModelContextWindow::new(128_000)?;
        let output = AiModelOutputLimit::new(4_096)?;

        assert_eq!(context.value(), 128_000);
        assert_eq!(output.get(), 4_096);
        assert_eq!(AiModelContextWindow::new(0), Err(AiModelError::Zero));
        assert_eq!(AiModelOutputLimit::new(0), Err(AiModelError::Zero));
        Ok(())
    }

    #[test]
    fn displays_and_parses_model_enums() -> Result<(), AiModelError> {
        assert_enum_family(AiModelKind::ALL)?;
        assert_enum_family(AiModelModality::ALL)?;
        assert_enum_family(AiModelInterfaceKind::ALL)?;
        assert_enum_family(AiModelReasoningMode::ALL)?;
        assert_enum_family(AiModelDeploymentKind::ALL)?;
        assert_enum_family(AiModelLifecycleStage::ALL)?;
        assert_eq!(
            "image generation".parse::<AiModelKind>()?,
            AiModelKind::ImageGeneration
        );
        Ok(())
    }
}
