# Conversation: Rust error handling conventions

## Context

The user intends to develop Rust error-handling foundations under `skywhale/skywhale/src`. The goal is to examine common Rust practices, establish a project-wide pattern, and later implement the related foundation. The workspace currently contains only a minimal library example and no existing error-handling convention.

## Confirmed decisions

- Use `thiserror` to define explicit error types when a caller needs to make a policy decision, such as conflict handling, retry, recovery, or conversion to a user-facing response.
- Treat `thiserror` and `anyhow` as complementary rather than competing tools.
- Keep meaningful error types until the last boundary at which a caller needs to distinguish them; use `anyhow` primarily for top-level propagation, diagnostic context, and final reporting.
- Keep `skywhale-core` independent of implementation crates such as `skywhale-db` and external drivers such as `sqlx`.
- Model core errors in implementation-independent terms such as repository, store, or use-case errors rather than database-driver errors.
- A database adapter selectively maps `sqlx` errors to core errors only when the result changes policy. Examples include missing data, conflicts caused by known uniqueness constraints, and temporary unavailability.
- Do not duplicate every `sqlx` variant or database error code in project error types. Unrecognized failures become a general failure category.
- Error return values communicate control and recovery policy. Detailed diagnostics belong in an error source chain or structured logs, not in public error strings.
- Avoid logging the same failure in every layer. If the original cause is preserved, record it at the shared top-level error boundary; if an adapter discards it while mapping to a generic core error, record it at the adapter immediately before conversion.
- Do not include sensitive values such as credentials, tokens, or full SQL parameters in diagnostics.

## Questions and answers

- Question: Are `thiserror` and `anyhow` the usual Rust error-handling approach?
  - Answer: They are a common combination. `thiserror` is suitable for explicit, actionable error contracts, and `anyhow` is suitable where concrete classification is no longer required.
- Question: Should `thiserror` errors support higher-level retry or conflict handling, while `anyhow` means only that an error occurred?
  - Answer: Explicit `thiserror` variants should support decisions such as retry and conflict handling. `anyhow` retains an error chain and accepts context, but should not be the normal mechanism for policy branching.
- Question: Can `anyhow` be wrapped inside `thiserror` for external integrations?
  - Answer: It may be useful inside an adapter, but should not be the public core error contract when callers need structured handling. The adapter should convert known conditions to core error variants before returning.
- Question: Is it practical to wrap all SQLx or database error codes in `thiserror`?
  - Answer: No. Map only codes and categories with application meaning; preserve or log all other driver errors as unexpected failures.
- Question: Does a core-level `DatabaseError(anyhow::Error)` solve the dependency issue?
  - Answer: It hides `sqlx` but also hides policy-relevant classification and makes core depend on `anyhow`. Prefer semantic core variants and, when diagnostics must travel upward, an opaque standard-library error source.
- Question: Should an unknown database failure be represented as `DatabaseError(Cow<str>)`?
  - Answer: No. A string loses structured cause information and can expose unstable or sensitive details. Use a generic failure variant; preserve the original source or record structured diagnostics at the designated observation boundary.

## Assumptions

- Future project structure may separate reusable core contracts from database adapters such as `skywhale-db`.
- A user-facing boundary such as an HTTP API, CLI, or application entry point will be introduced later.
- The exact set of core error variants will be driven by the first selected repository or use-case behavior, not prescribed globally in advance.

## Open questions

- Which first repository or use case will serve as the implementation example?
- Should unclassified infrastructure causes be preserved in a core error source chain, or logged in the adapter before converting to a source-free generic failure?
- Which user-facing delivery boundary will initially map core errors to responses: CLI, HTTP API, or another interface?

## Approval

- Status: approved for white-paper creation
- Date: 2026-07-18
