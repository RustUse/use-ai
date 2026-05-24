#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiPromptError, PromptFormat, PromptId, PromptInstructionKind, PromptName, PromptPartKind,
        PromptStatus, PromptTemplate, PromptText, PromptVariableKind, PromptVariableName,
    };
}

macro_rules! prompt_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AiPromptError> {
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
            type Err = AiPromptError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AiPromptError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! prompt_enum {
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
            type Err = AiPromptError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiPromptError::UnknownLabel),
                }
            }
        }
    };
}

prompt_text_newtype!(PromptName);
prompt_text_newtype!(PromptId);
prompt_text_newtype!(PromptText);
prompt_text_newtype!(PromptVariableName);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromptTemplate {
    name: PromptName,
    text: PromptText,
    format: PromptFormat,
}

impl PromptTemplate {
    pub fn new(name: PromptName, text: PromptText, format: PromptFormat) -> Self {
        Self { name, text, format }
    }

    pub fn name(&self) -> &PromptName {
        &self.name
    }

    pub fn text(&self) -> &PromptText {
        &self.text
    }

    pub const fn format(&self) -> PromptFormat {
        self.format
    }
}

prompt_enum!(PromptVariableKind {
    String => "string",
    Number => "number",
    Boolean => "boolean",
    Json => "json",
    Text => "text",
    List => "list",
    Object => "object",
    FileRef => "file-ref",
    ImageRef => "image-ref",
    AudioRef => "audio-ref",
    Custom => "custom",
});

prompt_enum!(PromptPartKind {
    System => "system",
    Developer => "developer",
    User => "user",
    Assistant => "assistant",
    Tool => "tool",
    Context => "context",
    Example => "example",
    Constraint => "constraint",
    OutputFormat => "output-format",
    Metadata => "metadata",
});

prompt_enum!(PromptInstructionKind {
    Task => "task",
    Constraint => "constraint",
    Style => "style",
    Safety => "safety",
    Role => "role",
    Format => "format",
    Context => "context",
    Example => "example",
    ToolUse => "tool-use",
    Refusal => "refusal",
    Custom => "custom",
});

prompt_enum!(PromptFormat {
    PlainText => "plain-text",
    Markdown => "markdown",
    Json => "json",
    Xml => "xml",
    Yaml => "yaml",
    ChatMessages => "chat-messages",
    Custom => "custom",
});

prompt_enum!(PromptStatus {
    Draft => "draft",
    Active => "active",
    Deprecated => "deprecated",
    Archived => "archived",
    Experimental => "experimental",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiPromptError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for AiPromptError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI prompt metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown AI prompt metadata label"),
        }
    }
}

impl Error for AiPromptError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiPromptError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiPromptError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiPromptError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiPromptError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiPromptError, PromptFormat, PromptId, PromptInstructionKind, PromptName, PromptPartKind,
        PromptStatus, PromptTemplate, PromptText, PromptVariableKind, PromptVariableName,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiPromptError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiPromptError>,
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
    fn validates_prompt_text_newtypes() -> Result<(), AiPromptError> {
        assert_text_newtype!(PromptName, "support-triage");
        assert_text_newtype!(PromptId, "prompt-001");
        assert_text_newtype!(PromptText, "Classify the support request");
        assert_text_newtype!(PromptVariableName, "customer_tier");
        assert_eq!(PromptName::new("  "), Err(AiPromptError::Empty));
        Ok(())
    }

    #[test]
    fn models_prompt_templates() -> Result<(), AiPromptError> {
        let name = PromptName::new("support-triage")?;
        let text = PromptText::new("Classify the support request")?;
        let template = PromptTemplate::new(name, text, PromptFormat::Markdown);

        assert_eq!(template.name().as_str(), "support-triage");
        assert_eq!(template.text().as_str(), "Classify the support request");
        assert_eq!(template.format(), PromptFormat::Markdown);
        Ok(())
    }

    #[test]
    fn displays_and_parses_prompt_enums() -> Result<(), AiPromptError> {
        assert_enum_family(PromptVariableKind::ALL)?;
        assert_enum_family(PromptPartKind::ALL)?;
        assert_enum_family(PromptInstructionKind::ALL)?;
        assert_enum_family(PromptFormat::ALL)?;
        assert_enum_family(PromptStatus::ALL)?;
        assert_eq!(
            "plain text".parse::<PromptFormat>()?,
            PromptFormat::PlainText
        );
        assert_eq!(
            "nope".parse::<PromptFormat>(),
            Err(AiPromptError::UnknownLabel)
        );
        Ok(())
    }
}
