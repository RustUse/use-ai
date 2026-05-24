#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        PlanDependencyKind, PlanExecutionMode, PlanId, PlanName, PlanPriority, PlanReviewStatus,
        PlanRiskKind, PlanStatus, PlanStepId, PlanStepKind, PlanningError,
    };
}

macro_rules! planning_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, PlanningError> {
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
            type Err = PlanningError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = PlanningError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! planning_enum {
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
            type Err = PlanningError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(PlanningError::UnknownLabel),
                }
            }
        }
    };
}

planning_text_newtype!(PlanName);
planning_text_newtype!(PlanId);
planning_text_newtype!(PlanStepId);

planning_enum!(PlanStepKind {
    Think => "think",
    Retrieve => "retrieve",
    Transform => "transform",
    Generate => "generate",
    Validate => "validate",
    CallTool => "call-tool",
    AskUser => "ask-user",
    Review => "review",
    Deliver => "deliver",
    Stop => "stop",
    Custom => "custom",
});

planning_enum!(PlanStatus {
    Draft => "draft",
    Ready => "ready",
    Running => "running",
    Blocked => "blocked",
    Succeeded => "succeeded",
    Failed => "failed",
    Cancelled => "cancelled",
    Superseded => "superseded",
});

planning_enum!(PlanPriority {
    Low => "low",
    Normal => "normal",
    High => "high",
    Urgent => "urgent",
});

planning_enum!(PlanDependencyKind {
    Sequential => "sequential",
    Parallel => "parallel",
    Conditional => "conditional",
    Blocking => "blocking",
    Optional => "optional",
});

planning_enum!(PlanExecutionMode {
    Manual => "manual",
    Assisted => "assisted",
    Automated => "automated",
    Supervised => "supervised",
});

planning_enum!(PlanReviewStatus {
    NotReviewed => "not-reviewed",
    NeedsReview => "needs-review",
    Approved => "approved",
    Rejected => "rejected",
    Revised => "revised",
});

planning_enum!(PlanRiskKind {
    AmbiguousInput => "ambiguous-input",
    MissingContext => "missing-context",
    UnsafeAction => "unsafe-action",
    ToolFailure => "tool-failure",
    TimeLimit => "time-limit",
    CostLimit => "cost-limit",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanningError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for PlanningError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("planning metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown planning metadata label"),
        }
    }
}

impl Error for PlanningError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, PlanningError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(PlanningError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, PlanningError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(PlanningError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PlanDependencyKind, PlanExecutionMode, PlanId, PlanName, PlanPriority, PlanReviewStatus,
        PlanRiskKind, PlanStatus, PlanStepId, PlanStepKind, PlanningError,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), PlanningError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = PlanningError>,
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
    fn validates_planning_text_newtypes() -> Result<(), PlanningError> {
        assert_text_newtype!(PlanName, "resolve-ticket");
        assert_text_newtype!(PlanId, "plan-001");
        assert_text_newtype!(PlanStepId, "step-001");
        assert_eq!(PlanName::new("  "), Err(PlanningError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_planning_enums() -> Result<(), PlanningError> {
        assert_enum_family(PlanStepKind::ALL)?;
        assert_enum_family(PlanStatus::ALL)?;
        assert_enum_family(PlanPriority::ALL)?;
        assert_enum_family(PlanDependencyKind::ALL)?;
        assert_enum_family(PlanExecutionMode::ALL)?;
        assert_enum_family(PlanReviewStatus::ALL)?;
        assert_enum_family(PlanRiskKind::ALL)?;
        assert_eq!("call tool".parse::<PlanStepKind>()?, PlanStepKind::CallTool);
        Ok(())
    }
}
