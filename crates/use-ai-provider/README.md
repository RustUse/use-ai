# use-ai-provider

AI provider and endpoint metadata primitives for `RustUse`.

## Experimental

`use-ai-provider` is experimental while `use-ai` remains below `0.3.0`.

## Example

```rust
use use_ai_provider::{AiApiMode, AiProviderKind, AiProviderName};

let provider = AiProviderName::new("local-runtime")?;

assert_eq!(provider.as_str(), "local-runtime");
assert_eq!("hosted api".parse::<AiProviderKind>()?, AiProviderKind::HostedApi);
assert_eq!(AiApiMode::Streaming.as_str(), "streaming");
# Ok::<(), use_ai_provider::AiProviderError>(())
```

## Scope

- Provider names and identifiers, endpoint names, provider and endpoint kinds, API modes, rate limits, quotas, billing units, and region labels.
- Metadata only.

## Non-goals

- Authentication, network calls, quota tracking, billing calculation, request routing, or provider SDKs.

## License

Licensed under either Apache-2.0 or MIT.