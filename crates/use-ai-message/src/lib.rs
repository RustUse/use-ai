#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiConversationId, AiMessageContentRef, AiMessageError, AiMessageFormat, AiMessageId,
        AiMessagePartKind, AiMessageRole, AiMessageSource, AiMessageStatus, AiMessageTokenCount,
        AiMessageVisibility,
    };
}

macro_rules! message_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AiMessageError> {
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
            type Err = AiMessageError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AiMessageError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! message_enum {
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
            type Err = AiMessageError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiMessageError::UnknownLabel),
                }
            }
        }
    };
}

message_text_newtype!(AiMessageId);
message_text_newtype!(AiConversationId);
message_text_newtype!(AiMessageContentRef);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AiMessageTokenCount(u32);

impl AiMessageTokenCount {
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(self) -> u32 {
        self.0
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

impl fmt::Display for AiMessageTokenCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

message_enum!(AiMessageRole {
    System => "system",
    Developer => "developer",
    User => "user",
    Assistant => "assistant",
    Tool => "tool",
    Function => "function",
    Observer => "observer",
    Evaluator => "evaluator",
    Unknown => "unknown",
});

message_enum!(AiMessagePartKind {
    Text => "text",
    Image => "image",
    Audio => "audio",
    Video => "video",
    File => "file",
    ToolCall => "tool-call",
    ToolResult => "tool-result",
    Citation => "citation",
    ThoughtSummary => "thought-summary",
    Metadata => "metadata",
    Custom => "custom",
});

message_enum!(AiMessageStatus {
    Draft => "draft",
    Pending => "pending",
    Streaming => "streaming",
    Complete => "complete",
    Failed => "failed",
    Cancelled => "cancelled",
    Redacted => "redacted",
});

message_enum!(AiMessageFormat {
    PlainText => "plain-text",
    Markdown => "markdown",
    Json => "json",
    Xml => "xml",
    Yaml => "yaml",
    Multipart => "multipart",
    Custom => "custom",
});

message_enum!(AiMessageVisibility {
    Internal => "internal",
    UserVisible => "user-visible",
    Hidden => "hidden",
    Redacted => "redacted",
});

message_enum!(AiMessageSource {
    Human => "human",
    Model => "model",
    Tool => "tool",
    System => "system",
    Import => "import",
    Synthetic => "synthetic",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiMessageError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for AiMessageError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI message metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown AI message metadata label"),
        }
    }
}

impl Error for AiMessageError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiMessageError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiMessageError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiMessageError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiMessageError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiConversationId, AiMessageContentRef, AiMessageError, AiMessageFormat, AiMessageId,
        AiMessagePartKind, AiMessageRole, AiMessageSource, AiMessageStatus, AiMessageTokenCount,
        AiMessageVisibility,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiMessageError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiMessageError>,
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
    fn validates_message_text_newtypes() -> Result<(), AiMessageError> {
        assert_text_newtype!(AiMessageId, "msg-001");
        assert_text_newtype!(AiConversationId, "conv-001");
        assert_text_newtype!(AiMessageContentRef, "content:001");
        assert_eq!(AiMessageId::new("  "), Err(AiMessageError::Empty));
        Ok(())
    }

    #[test]
    fn validates_token_counts() {
        let zero = AiMessageTokenCount::new(0);
        let count = AiMessageTokenCount::new(42);

        assert_eq!(zero.value(), 0);
        assert_eq!(count.get(), 42);
        assert_eq!(count.to_string(), "42");
    }

    #[test]
    fn displays_and_parses_message_enums() -> Result<(), AiMessageError> {
        assert_enum_family(AiMessageRole::ALL)?;
        assert_enum_family(AiMessagePartKind::ALL)?;
        assert_enum_family(AiMessageStatus::ALL)?;
        assert_enum_family(AiMessageFormat::ALL)?;
        assert_enum_family(AiMessageVisibility::ALL)?;
        assert_enum_family(AiMessageSource::ALL)?;
        assert_eq!(
            "user visible".parse::<AiMessageVisibility>()?,
            AiMessageVisibility::UserVisible
        );
        Ok(())
    }
}
