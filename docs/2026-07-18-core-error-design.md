# Core Error Contract Design

## Goal

Provide `skywhale-core` with one minimal, extensible public error contract. It
must let callers distinguish failures that require a policy decision while
allowing all other failures to retain their diagnostic context without bespoke
wrapping at every internal call site.

## Public contract

`skywhale_core::Error` is the single public error type exported by the crate.
The initial contract is a non-exhaustive `thiserror` enum with one catch-all
variant backed by `anyhow::Error`:

```rust
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
```

The crate re-exports this type at its root so public APIs can return
`Result<T, skywhale_core::Error>`. `thiserror` generates `From<anyhow::Error>`
for `Other`, so internal code can use `?` after adding diagnostic context with
`anyhow::Context`.

## Classification rule

An adapter or use-case adds a new `Error` variant only when a caller needs to
make a different recovery or response decision. Examples include a known
conflict, an absent resource, or temporary unavailability. The concrete
operation that makes the distinction is documented and tested when the variant
is added. Callers match these semantic variants to make their policy decision;
`Other` always maps to the generic failure policy.

An error does not receive a new variant merely because an external library or
service exposes a distinct code or message.

## Unclassified failures and observation

`Error::Other` preserves the original error chain and any context added with
`anyhow::Context`. It is not a semantic fault category: it means that no
caller-specific recovery or response policy applies.

`skywhale-core` does not choose a logging implementation and does not log. The
application entry point is the designated observation point: after all
semantic handling is complete, it logs the `Other` error chain once with safe
request and operation context. User-facing responses must use the generic
failure message rather than the transparent error display.

`skywhale-core` depends on `thiserror` and `anyhow`, but not on `sqlx` or a
future `skywhale-db` crate.

## Testing boundary

The initial core tests verify that an `anyhow::Error` converts to `Error::Other`,
that transparent display and source traversal retain its context chain, and
that `?` can propagate an `anyhow::Error` through a function returning the core
error. When a semantic variant is introduced, its tests verify the exact policy
classification and its display text. Application-entry-point tests own the
assertion that an unhandled `Other` error is logged exactly once.

## Compatibility

`#[non_exhaustive]` allows future policy-relevant variants without requiring
external consumers to match the enum exhaustively. The error string is
diagnostic context only; callers must not branch on it.

## Implementation verification

The core contract exposes only `Error::Other` initially. A future variant must
document the caller policy it enables and include a test for that policy. An
unhandled `Other` error is logged once by the application entry point; core and
adapters do not emit duplicate logs for it.
