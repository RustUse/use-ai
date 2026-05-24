#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AgentActionKind, AgentAutonomyLevel, AgentError, AgentHandoffKind, AgentId, AgentKind,
        AgentLoopKind, AgentMode, AgentName, AgentObservationKind, AgentStatus,
    };
}

macro_rules! agent_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AgentError> {
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
            type Err = AgentError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AgentError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! agent_enum {
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
            type Err = AgentError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AgentError::UnknownLabel),
                }
            }
        }
    };
}

agent_text_newtype!(AgentName);
agent_text_newtype!(AgentId);

agent_enum!(AgentKind {
    Assistant => "assistant",
    Planner => "planner",
    Executor => "executor",
    Researcher => "researcher",
    Coder => "coder",
    Reviewer => "reviewer",
    Critic => "critic",
    Router => "router",
    Orchestrator => "orchestrator",
    Worker => "worker",
    Custom => "custom",
});

agent_enum!(AgentStatus {
    Idle => "idle",
    Planning => "planning",
    Running => "running",
    WaitingForTool => "waiting-for-tool",
    WaitingForUser => "waiting-for-user",
    Succeeded => "succeeded",
    Failed => "failed",
    Cancelled => "cancelled",
    Paused => "paused",
});

agent_enum!(AgentMode {
    Manual => "manual",
    Assisted => "assisted",
    SemiAutonomous => "semi-autonomous",
    Autonomous => "autonomous",
    Supervised => "supervised",
});

agent_enum!(AgentAutonomyLevel {
    None => "none",
    Low => "low",
    Medium => "medium",
    High => "high",
    Full => "full",
});

agent_enum!(AgentLoopKind {
    SingleTurn => "single-turn",
    MultiTurn => "multi-turn",
    ReactLike => "react-like",
    PlanExecute => "plan-execute",
    ReflectAct => "reflect-act",
    RouterWorker => "router-worker",
    Custom => "custom",
});

agent_enum!(AgentHandoffKind {
    User => "user",
    Agent => "agent",
    Tool => "tool",
    HumanReviewer => "human-reviewer",
    System => "system",
    None => "none",
});

agent_enum!(AgentObservationKind {
    UserInput => "user-input",
    ToolResult => "tool-result",
    RetrievedContext => "retrieved-context",
    Memory => "memory",
    SystemEvent => "system-event",
    Error => "error",
    Custom => "custom",
});

agent_enum!(AgentActionKind {
    Respond => "respond",
    AskClarification => "ask-clarification",
    CallTool => "call-tool",
    Retrieve => "retrieve",
    Plan => "plan",
    Delegate => "delegate",
    Stop => "stop",
    Custom => "custom",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for AgentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("agent metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown agent metadata label"),
        }
    }
}

impl Error for AgentError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AgentError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AgentError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AgentError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AgentError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AgentActionKind, AgentAutonomyLevel, AgentError, AgentHandoffKind, AgentId, AgentKind,
        AgentLoopKind, AgentMode, AgentName, AgentObservationKind, AgentStatus,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AgentError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AgentError>,
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
    fn validates_agent_text_newtypes() -> Result<(), AgentError> {
        assert_text_newtype!(AgentName, "triage-agent");
        assert_text_newtype!(AgentId, "agent-001");
        assert_eq!(AgentName::new("  "), Err(AgentError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_agent_enums() -> Result<(), AgentError> {
        assert_enum_family(AgentKind::ALL)?;
        assert_enum_family(AgentStatus::ALL)?;
        assert_enum_family(AgentMode::ALL)?;
        assert_enum_family(AgentAutonomyLevel::ALL)?;
        assert_enum_family(AgentLoopKind::ALL)?;
        assert_enum_family(AgentHandoffKind::ALL)?;
        assert_enum_family(AgentObservationKind::ALL)?;
        assert_enum_family(AgentActionKind::ALL)?;
        assert_eq!(
            "waiting for tool".parse::<AgentStatus>()?,
            AgentStatus::WaitingForTool
        );
        Ok(())
    }
}
