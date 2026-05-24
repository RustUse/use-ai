#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

pub mod prelude {
    pub use crate::{
        RagChunkId, RagChunkKind, RagCitationKind, RagContextAssemblyKind, RagCorpusName,
        RagDocumentId, RagError, RagFreshnessStatus, RagGroundingStatus, RagRankerKind,
        RagRetrievalMode, RagRetrieverKind,
    };
}

macro_rules! rag_text_newtype {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl AsRef<str>) -> Result<Self, RagError> {
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
            type Err = RagError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl TryFrom<&str> for $name {
            type Error = RagError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }
    };
}

macro_rules! rag_enum {
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
            type Err = RagError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match normalized_label(value)?.as_str() {
                    $($label => Ok(Self::$variant),)+
                    _ => Err(RagError::UnknownLabel),
                }
            }
        }
    };
}

rag_text_newtype!(RagCorpusName);
rag_text_newtype!(RagDocumentId);
rag_text_newtype!(RagChunkId);

rag_enum!(RagChunkKind {
    Text => "text",
    Table => "table",
    Code => "code",
    Image => "image",
    Audio => "audio",
    Metadata => "metadata",
    Mixed => "mixed",
    Custom => "custom",
});

rag_enum!(RagRetrieverKind {
    Keyword => "keyword",
    Vector => "vector",
    Hybrid => "hybrid",
    Graph => "graph",
    Sql => "sql",
    Api => "api",
    FileSearch => "file-search",
    WebSearch => "web-search",
    Custom => "custom",
});

rag_enum!(RagRetrievalMode {
    TopK => "top-k",
    Threshold => "threshold",
    Filtered => "filtered",
    Hybrid => "hybrid",
    Recursive => "recursive",
    MultiQuery => "multi-query",
    Custom => "custom",
});

rag_enum!(RagRankerKind {
    None => "none",
    Score => "score",
    Reranker => "reranker",
    CrossEncoder => "cross-encoder",
    Diversity => "diversity",
    Recency => "recency",
    Custom => "custom",
});

rag_enum!(RagCitationKind {
    Document => "document",
    Url => "url",
    File => "file",
    LineRange => "line-range",
    PageRange => "page-range",
    Timestamp => "timestamp",
    Chunk => "chunk",
    Custom => "custom",
});

rag_enum!(RagGroundingStatus {
    Grounded => "grounded",
    PartiallyGrounded => "partially-grounded",
    Ungrounded => "ungrounded",
    Conflicting => "conflicting",
    Unknown => "unknown",
});

rag_enum!(RagContextAssemblyKind {
    Append => "append",
    Compress => "compress",
    Summarize => "summarize",
    MapReduce => "map-reduce",
    Windowed => "windowed",
    Hierarchical => "hierarchical",
    Custom => "custom",
});

rag_enum!(RagFreshnessStatus {
    Fresh => "fresh",
    Stale => "stale",
    Unknown => "unknown",
    TimeSensitive => "time-sensitive",
});

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RagError {
    Empty,
    UnknownLabel,
}

impl fmt::Display for RagError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("RAG metadata text cannot be empty"),
            Self::UnknownLabel => formatter.write_str("unknown RAG metadata label"),
        }
    }
}

impl Error for RagError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, RagError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(RagError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_label(value: &str) -> Result<String, RagError> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        Err(RagError::Empty)
    } else {
        Ok(trimmed.to_ascii_lowercase().replace(['_', ' '], "-"))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        RagChunkId, RagChunkKind, RagCitationKind, RagContextAssemblyKind, RagCorpusName,
        RagDocumentId, RagError, RagFreshnessStatus, RagGroundingStatus, RagRankerKind,
        RagRetrievalMode, RagRetrieverKind,
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

    fn assert_enum_family<T>(variants: &[T]) -> Result<(), RagError>
    where
        T: Copy + Eq + fmt::Debug + fmt::Display + FromStr<Err = RagError>,
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
    fn validates_rag_text_newtypes() -> Result<(), RagError> {
        assert_text_newtype!(RagCorpusName, "support-docs");
        assert_text_newtype!(RagDocumentId, "doc-001");
        assert_text_newtype!(RagChunkId, "chunk-001");
        assert_eq!(RagCorpusName::new("  "), Err(RagError::Empty));
        Ok(())
    }

    #[test]
    fn displays_and_parses_rag_enums() -> Result<(), RagError> {
        assert_enum_family(RagChunkKind::ALL)?;
        assert_enum_family(RagRetrieverKind::ALL)?;
        assert_enum_family(RagRetrievalMode::ALL)?;
        assert_enum_family(RagRankerKind::ALL)?;
        assert_enum_family(RagCitationKind::ALL)?;
        assert_enum_family(RagGroundingStatus::ALL)?;
        assert_enum_family(RagContextAssemblyKind::ALL)?;
        assert_enum_family(RagFreshnessStatus::ALL)?;
        assert_eq!(
            "file search".parse::<RagRetrieverKind>()?,
            RagRetrieverKind::FileSearch
        );
        Ok(())
    }
}
