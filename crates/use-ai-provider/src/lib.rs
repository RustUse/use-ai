#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        AiApiMode, AiBillingUnit, AiEndpointKind, AiEndpointName, AiProviderError, AiProviderId,
        AiProviderKind, AiProviderName, AiQuotaKind, AiRateLimitKind, AiRegionKind,
    };
}

macro_rules! provider_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, AiProviderError> {
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
            type Err = AiProviderError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = AiProviderError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! provider_enum {
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
            type Err = AiProviderError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(AiProviderError::UnknownLabel),
                }
            }
        }
    };
}

provider_text_newtype!(AiProviderName);
provider_text_newtype!(AiProviderId);
provider_text_newtype!(AiEndpointName);

provider_enum!(AiProviderKind {
    HostedApi => "hosted-api",
    CloudPlatform => "cloud-platform",
    LocalRuntime => "local-runtime",
    OpenSourceHost => "open-source-host",
    InternalGateway => "internal-gateway",
    Custom => "custom",
});

provider_enum!(AiEndpointKind {
    Chat => "chat",
    Completion => "completion",
    Responses => "responses",
    Embedding => "embedding",
    Rerank => "rerank",
    Image => "image",
    Audio => "audio",
    Realtime => "realtime",
    Batch => "batch",
    Moderation => "moderation",
    Custom => "custom",
});

provider_enum!(AiApiMode {
    Sync => "sync",
    Async => "async",
    Streaming => "streaming",
    Batch => "batch",
    Realtime => "realtime",
});

provider_enum!(AiRateLimitKind {
    RequestsPerMinute => "requests-per-minute",
    TokensPerMinute => "tokens-per-minute",
    ImagesPerMinute => "images-per-minute",
    ConcurrentRequests => "concurrent-requests",
    Custom => "custom",
});

provider_enum!(AiQuotaKind {
    HardLimit => "hard-limit",
    SoftLimit => "soft-limit",
    Burst => "burst",
    Trial => "trial",
    Unknown => "unknown",
});

provider_enum!(AiBillingUnit {
    InputToken => "input-token",
    OutputToken => "output-token",
    CachedToken => "cached-token",
    Request => "request",
    Image => "image",
    AudioSecond => "audio-second",
    ComputeSecond => "compute-second",
    Custom => "custom",
});

provider_enum!(AiRegionKind {
    Global => "global",
    Regional => "regional",
    Local => "local",
    Edge => "edge",
    Unknown => "unknown",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AiProviderError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for AiProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("AI provider metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown AI provider metadata label"),
        }
    }
}

impl Error for AiProviderError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, AiProviderError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(AiProviderError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, AiProviderError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(AiProviderError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AiApiMode, AiBillingUnit, AiEndpointKind, AiEndpointName, AiProviderError, AiProviderId,
        AiProviderKind, AiProviderName, AiQuotaKind, AiRateLimitKind, AiRegionKind,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), AiProviderError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = AiProviderError>,
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
    fn validates_provider_text_newtypes() -> Result<(), AiProviderError> {
        assert_text_newtype!(AiProviderName, "local-runtime");
        assert_text_newtype!(AiProviderId, "provider-001");
        assert_text_newtype!(AiEndpointName, "chat-primary");
        assert_eq!(AiProviderName::new("  "), Err(AiProviderError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_provider_enums() -> Result<(), AiProviderError> {
        assert_enum_family(AiProviderKind::ALL)?;
        assert_enum_family(AiEndpointKind::ALL)?;
        assert_enum_family(AiApiMode::ALL)?;
        assert_enum_family(AiRateLimitKind::ALL)?;
        assert_enum_family(AiQuotaKind::ALL)?;
        assert_enum_family(AiBillingUnit::ALL)?;
        assert_enum_family(AiRegionKind::ALL)?;
        assert_eq!(
            "hosted api".parse::<AiProviderKind>()?,
            AiProviderKind::HostedApi
        );
        Ok(())
    }
}
