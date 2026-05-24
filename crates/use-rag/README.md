# use-rag

RAG metadata primitives for `RustUse` AI workflows.

## Experimental

`use-rag` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_rag::{RagCorpusName, RagRetrieverKind, RagRetrievalMode};

let corpus = RagCorpusName::new("support-docs")?;

assert_eq!(corpus.as_str(), "support-docs");
assert_eq!("file search".parse::<RagRetrieverKind>()?, RagRetrieverKind::FileSearch);
assert_eq!(RagRetrievalMode::TopK.as_str(), "top-k");
# Ok::<(), use_rag::RagError>(())
```

## Scope

- RAG corpus, document, and chunk identifiers plus chunk, retriever, retrieval, ranking, citation, grounding, assembly, and freshness labels.
- Metadata only.

## Non-goals

- Retrieval, embedding computation, vector search, ranking execution, context assembly, crawling, or citation verification.

## License

Licensed under either Apache-2.0 or MIT.