#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiInstructionAuthority, AiParticipantId, AiParticipantKind, AiPersonaKind, AiRoleError,
        AiRoleName, AiRoleScope, AiRoleStatus,
    };
}

macro_rules! role_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AiRoleError> {
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
            type Err = AiRoleError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AiRoleError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! role_enum {
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
            type Err = AiRoleError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiRoleError::UnknownLabel),
                }
            }
        }
    };
}

role_text_newtype!(AiRoleName);
role_text_newtype!(AiParticipantId);

role_enum!(AiParticipantKind {
    User => "user",
    Assistant => "assistant",
    System => "system",
    Developer => "developer",
    Tool => "tool",
    Agent => "agent",
    Evaluator => "evaluator",
    Observer => "observer",
    Unknown => "unknown",
});

role_enum!(AiInstructionAuthority {
    System => "system",
    Developer => "developer",
    User => "user",
    Tool => "tool",
    Retrieved => "retrieved",
    Memory => "memory",
    Unknown => "unknown",
});

role_enum!(AiRoleScope {
    Conversation => "conversation",
    Session => "session",
    Task => "task",
    Tool => "tool",
    Agent => "agent",
    Organization => "organization",
    Global => "global",
});

role_enum!(AiPersonaKind {
    Assistant => "assistant",
    Critic => "critic",
    Planner => "planner",
    Researcher => "researcher",
    Coder => "coder",
    Reviewer => "reviewer",
    Tutor => "tutor",
    Analyst => "analyst",
    Operator => "operator",
    Custom => "custom",
});

role_enum!(AiRoleStatus {
    Active => "active",
    Inactive => "inactive",
    Deprecated => "deprecated",
    Experimental => "experimental",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiRoleError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for AiRoleError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI role metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown AI role metadata label"),
        }
    }
}

impl Error for AiRoleError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiRoleError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiRoleError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiRoleError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiRoleError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiInstructionAuthority, AiParticipantId, AiParticipantKind, AiPersonaKind, AiRoleError,
        AiRoleName, AiRoleScope, AiRoleStatus,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiRoleError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiRoleError>,
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
    fn validates_role_text_newtypes() -> Result<(), AiRoleError> {
        assert_text_newtype!(AiRoleName, "assistant");
        assert_text_newtype!(AiParticipantId, "participant-001");
        assert_eq!(AiRoleName::new("  "), Err(AiRoleError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_role_enums() -> Result<(), AiRoleError> {
        assert_enum_family(AiParticipantKind::ALL)?;
        assert_enum_family(AiInstructionAuthority::ALL)?;
        assert_enum_family(AiRoleScope::ALL)?;
        assert_enum_family(AiPersonaKind::ALL)?;
        assert_enum_family(AiRoleStatus::ALL)?;
        assert_eq!(
            "system".parse::<AiInstructionAuthority>()?,
            AiInstructionAuthority::System
        );
        Ok(())
    }
}
