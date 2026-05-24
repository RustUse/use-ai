#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiMemoryConfidence, AiMemoryConflictKind, AiMemoryError, AiMemoryId, AiMemoryKind,
        AiMemoryOperationKind, AiMemoryRetentionKind, AiMemoryScope, AiMemorySensitivity,
        AiMemorySourceKind, AiMemoryStatus,
    };
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AiMemoryId(String);

impl AiMemoryId {
    pub fn new(value: impl AsRef<str>) -> Result<Self, AiMemoryError> {
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

impl AsRef<str> for AiMemoryId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for AiMemoryId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for AiMemoryId {
    type Err = AiMemoryError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<&str> for AiMemoryId {
    type Error = AiMemoryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct AiMemoryConfidence(f64);

impl AiMemoryConfidence {
    pub fn new(value: f64) -> Result<Self, AiMemoryError> {
        if !value.is_finite() {
            return Err(AiMemoryError::NonFinite);
        }
        if !(0.0..=1.0).contains(&value) {
            return Err(AiMemoryError::OutOfRange);
        }
        Ok(Self(value))
    }

    pub const fn value(self) -> f64 {
        self.0
    }
}

macro_rules! memory_enum {
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
            type Err = AiMemoryError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiMemoryError::UnknownLabel),
                }
            }
        }
    };
}

memory_enum!(AiMemoryKind {
    Preference => "preference",
    Fact => "fact",
    Instruction => "instruction",
    Summary => "summary",
    Skill => "skill",
    Relationship => "relationship",
    Project => "project",
    Workflow => "workflow",
    Correction => "correction",
    Custom => "custom",
});

memory_enum!(AiMemoryScope {
    Session => "session",
    User => "user",
    Project => "project",
    Organization => "organization",
    Global => "global",
    Tool => "tool",
    Agent => "agent",
});

memory_enum!(AiMemoryStatus {
    Active => "active",
    Superseded => "superseded",
    Deleted => "deleted",
    Archived => "archived",
    PendingReview => "pending-review",
});

memory_enum!(AiMemorySourceKind {
    UserProvided => "user-provided",
    Inferred => "inferred",
    Imported => "imported",
    ToolGenerated => "tool-generated",
    SystemGenerated => "system-generated",
    Unknown => "unknown",
});

memory_enum!(AiMemoryRetentionKind {
    Ephemeral => "ephemeral",
    Session => "session",
    LongTerm => "long-term",
    UntilRevoked => "until-revoked",
    PolicyControlled => "policy-controlled",
});

memory_enum!(AiMemorySensitivity {
    Public => "public",
    Internal => "internal",
    Private => "private",
    Sensitive => "sensitive",
    Restricted => "restricted",
    Unknown => "unknown",
});

memory_enum!(AiMemoryOperationKind {
    Create => "create",
    Read => "read",
    Update => "update",
    Delete => "delete",
    Merge => "merge",
    Suppress => "suppress",
    Forget => "forget",
});

memory_enum!(AiMemoryConflictKind {
    Duplicate => "duplicate",
    Contradiction => "contradiction",
    Stale => "stale",
    ScopeMismatch => "scope-mismatch",
    SensitivityMismatch => "sensitivity-mismatch",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiMemoryError {
    Empty,
    NonFinite,
    OutOfRange,
    UnknownLabel,
}

impl fmt::Display for AiMemoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI memory metadata text cannot be empty"),
            Self::NonFinite => formatter.write_str("AI memory confidence must be finite"),
            Self::OutOfRange => formatter.write_str("AI memory confidence must be in 0.0..=1.0"),
            Self::UnknownLabel => formatter.write_str("unknown AI memory metadata label"),
        }
    }
}

impl Error for AiMemoryError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiMemoryError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiMemoryError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiMemoryError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiMemoryError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiMemoryConfidence, AiMemoryConflictKind, AiMemoryError, AiMemoryId, AiMemoryKind,
        AiMemoryOperationKind, AiMemoryRetentionKind, AiMemoryScope, AiMemorySensitivity,
        AiMemorySourceKind, AiMemoryStatus,
    };
    use core::{fmt, str::FromStr};

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiMemoryError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiMemoryError>,
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
    fn validates_memory_ids() -> Result<(), AiMemoryError> {
        let id = AiMemoryId::new(" memory-001 ")?;

        assert_eq!(id.as_str(), "memory-001");
        assert_eq!(id.value(), "memory-001");
        assert_eq!(id.as_ref(), "memory-001");
        assert_eq!(id.to_string(), "memory-001");
        assert_eq!(AiMemoryId::try_from("memory-001")?, id);
        assert_eq!(id.into_string(), "memory-001".to_string());
        assert_eq!(AiMemoryId::new("  "), Err(AiMemoryError::Empty));
        Ok(())
    }

    #[test]
    fn validates_memory_confidence() -> Result<(), AiMemoryError> {
        assert_eq!(AiMemoryConfidence::new(0.0)?.value(), 0.0);
        assert_eq!(AiMemoryConfidence::new(1.0)?.value(), 1.0);
        assert_eq!(
            AiMemoryConfidence::new(-0.1),
            Err(AiMemoryError::OutOfRange)
        );
        assert_eq!(AiMemoryConfidence::new(1.1), Err(AiMemoryError::OutOfRange));
        assert_eq!(
            AiMemoryConfidence::new(f64::NAN),
            Err(AiMemoryError::NonFinite)
        );
        Ok(())
    }

    #[test]
    fn displays_and_parses_memory_enums() -> Result<(), AiMemoryError> {
        assert_enum_family(AiMemoryKind::ALL)?;
        assert_enum_family(AiMemoryScope::ALL)?;
        assert_enum_family(AiMemoryStatus::ALL)?;
        assert_enum_family(AiMemorySourceKind::ALL)?;
        assert_enum_family(AiMemoryRetentionKind::ALL)?;
        assert_enum_family(AiMemorySensitivity::ALL)?;
        assert_enum_family(AiMemoryOperationKind::ALL)?;
        assert_enum_family(AiMemoryConflictKind::ALL)?;
        assert_eq!(
            "policy controlled".parse::<AiMemoryRetentionKind>()?,
            AiMemoryRetentionKind::PolicyControlled
        );
        Ok(())
    }
}
