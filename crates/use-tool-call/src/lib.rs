#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        ToolArgumentKind, ToolArgumentName, ToolCallError, ToolCallErrorKind, ToolCallId,
        ToolCallKind, ToolCallStatus, ToolChoiceKind, ToolName, ToolResultKind, ToolSchemaKind,
    };
}

macro_rules! tool_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, ToolCallError> {
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
            type Err = ToolCallError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = ToolCallError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! tool_enum {
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
            type Err = ToolCallError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(ToolCallError::UnknownLabel),
                }
            }
        }
    };
}

tool_text_newtype!(ToolName);
tool_text_newtype!(ToolCallId);
tool_text_newtype!(ToolArgumentName);

tool_enum!(ToolCallStatus {
    Pending => "pending",
    Running => "running",
    Succeeded => "succeeded",
    Failed => "failed",
    Cancelled => "cancelled",
    TimedOut => "timed-out",
    Rejected => "rejected",
});

tool_enum!(ToolCallKind {
    Function => "function",
    Api => "api",
    Search => "search",
    Database => "database",
    File => "file",
    Browser => "browser",
    Code => "code",
    Shell => "shell",
    Calendar => "calendar",
    Email => "email",
    Custom => "custom",
});

tool_enum!(ToolArgumentKind {
    String => "string",
    Number => "number",
    Boolean => "boolean",
    Json => "json",
    Array => "array",
    Object => "object",
    FileRef => "file-ref",
    ImageRef => "image-ref",
    AudioRef => "audio-ref",
    Custom => "custom",
});

tool_enum!(ToolResultKind {
    Text => "text",
    Json => "json",
    File => "file",
    Image => "image",
    Audio => "audio",
    Table => "table",
    Error => "error",
    Empty => "empty",
    Custom => "custom",
});

tool_enum!(ToolSchemaKind {
    JsonSchema => "json-schema",
    OpenApi => "open-api",
    FunctionSignature => "function-signature",
    Freeform => "freeform",
    Custom => "custom",
});

tool_enum!(ToolChoiceKind {
    None => "none",
    Auto => "auto",
    Required => "required",
    NamedTool => "named-tool",
    AnyTool => "any-tool",
});

tool_enum!(ToolCallErrorKind {
    Validation => "validation",
    Authorization => "authorization",
    Timeout => "timeout",
    NotFound => "not-found",
    RateLimited => "rate-limited",
    Execution => "execution",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolCallError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for ToolCallError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("tool-call metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown tool-call metadata label"),
        }
    }
}

impl Error for ToolCallError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, ToolCallError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ToolCallError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, ToolCallError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(ToolCallError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ToolArgumentKind, ToolArgumentName, ToolCallError, ToolCallErrorKind, ToolCallId,
        ToolCallKind, ToolCallStatus, ToolChoiceKind, ToolName, ToolResultKind, ToolSchemaKind,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), ToolCallError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = ToolCallError>,
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
    fn validates_tool_text_newtypes() -> Result<(), ToolCallError> {
        assert_text_newtype!(ToolName, "ticket-search");
        assert_text_newtype!(ToolCallId, "call-001");
        assert_text_newtype!(ToolArgumentName, "query");
        assert_eq!(ToolName::new("  "), Err(ToolCallError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_tool_enums() -> Result<(), ToolCallError> {
        assert_enum_family(ToolCallStatus::ALL)?;
        assert_enum_family(ToolCallKind::ALL)?;
        assert_enum_family(ToolArgumentKind::ALL)?;
        assert_enum_family(ToolResultKind::ALL)?;
        assert_enum_family(ToolSchemaKind::ALL)?;
        assert_enum_family(ToolChoiceKind::ALL)?;
        assert_enum_family(ToolCallErrorKind::ALL)?;
        assert_eq!(
            "timed out".parse::<ToolCallStatus>()?,
            ToolCallStatus::TimedOut
        );
        Ok(())
    }
}
