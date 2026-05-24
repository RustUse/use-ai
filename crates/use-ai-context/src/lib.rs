#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiContextCitation, AiContextError, AiContextFitStatus, AiContextId, AiContextItemId,
        AiContextItemKind, AiContextPriority, AiContextSourceKind, AiContextWindow,
        AiContextWindowSize, AiGroundingStatus,
    };
}

macro_rules! context_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AiContextError> {
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
            type Err = AiContextError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AiContextError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! context_enum {
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
            type Err = AiContextError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiContextError::UnknownLabel),
                }
            }
        }
    };
}

context_text_newtype!(AiContextId);
context_text_newtype!(AiContextItemId);
context_text_newtype!(AiContextCitation);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AiContextWindowSize(u32);

impl AiContextWindowSize {
    pub fn new(value: u32) -> Result<Self, AiContextError> {
        if value == 0 {
            Err(AiContextError::Zero)
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

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AiContextWindow {
    size: AiContextWindowSize,
}

impl AiContextWindow {
    pub const fn new(size: AiContextWindowSize) -> Self {
        Self { size }
    }

    pub const fn size(self) -> AiContextWindowSize {
        self.size
    }
}

context_enum!(AiContextItemKind {
    Message => "message",
    Document => "document",
    File => "file",
    WebPage => "web-page",
    Code => "code",
    Table => "table",
    Image => "image",
    Audio => "audio",
    ToolResult => "tool-result",
    Memory => "memory",
    RetrievedChunk => "retrieved-chunk",
    Metadata => "metadata",
    Custom => "custom",
});

context_enum!(AiContextSourceKind {
    UserProvided => "user-provided",
    SystemProvided => "system-provided",
    Retrieved => "retrieved",
    ToolGenerated => "tool-generated",
    Memory => "memory",
    Web => "web",
    File => "file",
    Database => "database",
    Api => "api",
    Synthetic => "synthetic",
    Unknown => "unknown",
});

context_enum!(AiContextPriority {
    Low => "low",
    Normal => "normal",
    High => "high",
    Critical => "critical",
});

context_enum!(AiContextFitStatus {
    Fits => "fits",
    Truncated => "truncated",
    Summarized => "summarized",
    Omitted => "omitted",
    Overflow => "overflow",
});

context_enum!(AiGroundingStatus {
    Grounded => "grounded",
    PartiallyGrounded => "partially-grounded",
    Ungrounded => "ungrounded",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiContextError {
    Empty,
    Zero,
    UnknownLabel,
}

impl fmt::Display for AiContextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI context metadata text cannot be empty"),
            Self::Zero => formatter.write_str("AI context numeric value must be positive"),
            Self::UnknownLabel => formatter.write_str("unknown AI context metadata label"),
        }
    }
}

impl Error for AiContextError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiContextError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiContextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiContextError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiContextError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiContextCitation, AiContextError, AiContextFitStatus, AiContextId, AiContextItemId,
        AiContextItemKind, AiContextPriority, AiContextSourceKind, AiContextWindow,
        AiContextWindowSize, AiGroundingStatus,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiContextError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiContextError>,
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
    fn validates_context_text_newtypes() -> Result<(), AiContextError> {
        assert_text_newtype!(AiContextId, "ctx-001");
        assert_text_newtype!(AiContextItemId, "item-001");
        assert_text_newtype!(AiContextCitation, "doc:1");
        assert_eq!(AiContextId::new("  "), Err(AiContextError::Empty));
        Ok(())
    }

    #[test]
    fn validates_context_windows() -> Result<(), AiContextError> {
        let size = AiContextWindowSize::new(8_192)?;
        let window = AiContextWindow::new(size);

        assert_eq!(size.get(), 8_192);
        assert_eq!(window.size().value(), 8_192);
        assert_eq!(AiContextWindowSize::new(0), Err(AiContextError::Zero));
        Ok(())
    }

    #[test]
    fn displays_and_parses_context_enums() -> Result<(), AiContextError> {
        assert_enum_family(AiContextItemKind::ALL)?;
        assert_enum_family(AiContextSourceKind::ALL)?;
        assert_enum_family(AiContextPriority::ALL)?;
        assert_enum_family(AiContextFitStatus::ALL)?;
        assert_enum_family(AiGroundingStatus::ALL)?;
        assert_eq!(
            "web page".parse::<AiContextItemKind>()?,
            AiContextItemKind::WebPage
        );
        Ok(())
    }
}
