#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        GuardrailAction, GuardrailCheckKind, GuardrailError, GuardrailId, GuardrailKind,
        GuardrailName, GuardrailPolicyArea, GuardrailResultKind, GuardrailSeverity,
        GuardrailStatus, GuardrailViolationKind,
    };
}

macro_rules! guardrail_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, GuardrailError> {
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
            type Err = GuardrailError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = GuardrailError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! guardrail_enum {
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
            type Err = GuardrailError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(GuardrailError::UnknownLabel),
                }
            }
        }
    };
}

guardrail_text_newtype!(GuardrailName);
guardrail_text_newtype!(GuardrailId);

guardrail_enum!(GuardrailKind {
    Input => "input",
    Output => "output",
    ToolUse => "tool-use",
    Retrieval => "retrieval",
    Memory => "memory",
    Policy => "policy",
    Format => "format",
    RateLimit => "rate-limit",
    CostLimit => "cost-limit",
    HumanReview => "human-review",
    Custom => "custom",
});

guardrail_enum!(GuardrailAction {
    Allow => "allow",
    Block => "block",
    Redact => "redact",
    Transform => "transform",
    Warn => "warn",
    Escalate => "escalate",
    RequireReview => "require-review",
    Refuse => "refuse",
    Unknown => "unknown",
});

guardrail_enum!(GuardrailSeverity {
    Informational => "informational",
    Low => "low",
    Medium => "medium",
    High => "high",
    Critical => "critical",
});

guardrail_enum!(GuardrailStatus {
    Enabled => "enabled",
    Disabled => "disabled",
    Shadow => "shadow",
    Testing => "testing",
    Deprecated => "deprecated",
});

guardrail_enum!(GuardrailPolicyArea {
    Safety => "safety",
    Security => "security",
    Privacy => "privacy",
    Compliance => "compliance",
    Copyright => "copyright",
    Pii => "pii",
    Secrets => "secrets",
    Abuse => "abuse",
    Quality => "quality",
    Custom => "custom",
});

guardrail_enum!(GuardrailCheckKind {
    Moderation => "moderation",
    PiiDetection => "pii-detection",
    SecretDetection => "secret-detection",
    JailbreakDetection => "jailbreak-detection",
    PromptInjectionDetection => "prompt-injection-detection",
    CitationCheck => "citation-check",
    SchemaCheck => "schema-check",
    ToolPermissionCheck => "tool-permission-check",
    Custom => "custom",
});

guardrail_enum!(GuardrailResultKind {
    Passed => "passed",
    Failed => "failed",
    Warning => "warning",
    Skipped => "skipped",
    Error => "error",
    Unknown => "unknown",
});

guardrail_enum!(GuardrailViolationKind {
    UnsafeContent => "unsafe-content",
    Pii => "pii",
    Secret => "secret",
    PromptInjection => "prompt-injection",
    Jailbreak => "jailbreak",
    PolicyViolation => "policy-violation",
    SchemaViolation => "schema-violation",
    ToolMisuse => "tool-misuse",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardrailError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for GuardrailError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("guardrail metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown guardrail metadata label"),
        }
    }
}

impl Error for GuardrailError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, GuardrailError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(GuardrailError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, GuardrailError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(GuardrailError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        GuardrailAction, GuardrailCheckKind, GuardrailError, GuardrailId, GuardrailKind,
        GuardrailName, GuardrailPolicyArea, GuardrailResultKind, GuardrailSeverity,
        GuardrailStatus, GuardrailViolationKind,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), GuardrailError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = GuardrailError>,
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
    fn validates_guardrail_text_newtypes() -> Result<(), GuardrailError> {
        assert_text_newtype!(GuardrailName, "pii-redaction");
        assert_text_newtype!(GuardrailId, "guardrail-001");
        assert_eq!(GuardrailName::new("  "), Err(GuardrailError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_guardrail_enums() -> Result<(), GuardrailError> {
        assert_enum_family(GuardrailKind::ALL)?;
        assert_enum_family(GuardrailAction::ALL)?;
        assert_enum_family(GuardrailSeverity::ALL)?;
        assert_enum_family(GuardrailStatus::ALL)?;
        assert_enum_family(GuardrailPolicyArea::ALL)?;
        assert_enum_family(GuardrailCheckKind::ALL)?;
        assert_enum_family(GuardrailResultKind::ALL)?;
        assert_enum_family(GuardrailViolationKind::ALL)?;
        assert_eq!(
            "require review".parse::<GuardrailAction>()?,
            GuardrailAction::RequireReview
        );
        Ok(())
    }
}
