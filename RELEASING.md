# Releasing

This repository uses a focused-first release flow rather than publishing the
`use-ai` facade crate first.

## Current release state

Publish the focused crates before the `use-ai` umbrella crate.

## Current automation

The repository is prepared for release automation with `release-plz.toml` and a
focused-first publish order. Workflow files can be added when this repository is
ready for CI.

## Initial publish order

For the first public crates.io wave, publish in this order:

1. `use-ai-prompt`
2. `use-ai-message`
3. `use-ai-context`
4. `use-ai-role`
5. `use-ai-model`
6. `use-ai-provider`
7. `use-ai-capability`
8. `use-tool-call`
9. `use-agent`
10. `use-reasoning`
11. `use-planning`
12. `use-rag`
13. `use-ai-memory`
14. `use-guardrail`
15. `use-ai-eval`
16. `use-ai`

The `use-ai` facade should come last after the focused crates are visible on
crates.io.

## Follow-up release automation

After the initial manual crates.io wave is complete, the repository can use
`release-plz` for follow-up releases.

## Permanent version warning

Published crates.io versions are permanent. Verify the crate metadata,
packaging, and changelog inputs before any real publish.