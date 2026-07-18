# White Paper: Rust error handling conventions

## Problem and context

Skywhale needs a Rust error-handling approach that supports consistent recovery decisions, preserves architectural dependency direction, and provides useful operational diagnostics. The project currently has no established error pattern.

## Objective

Define an implementation-independent error-handling convention for core contracts and their external adapters, suitable for later application across the project.

## Non-goals

- Reproducing all error variants or error codes exposed by SQLx or a database server.
- Making `anyhow` the public error contract for core operations that need policy decisions.
- Exposing driver messages, database internals, or sensitive data directly to end users.
- Selecting a concrete database, API framework, retry library, logging framework, or application delivery interface.
- Implementing code, an implementation plan, or tests in this task record.

## Requirements

- Use explicit `thiserror` error types for conditions that change caller behavior, including conflict handling, retry, recovery, and user-facing response mapping.
- Keep core error contracts independent of adapter crates and external driver types.
- Name core error contracts after their functional boundary, such as repository, store, or use case, rather than a concrete database technology.
- Map external errors selectively at adapter boundaries.
- Preserve an unclassified error's cause for diagnostics when needed, without adding a concrete external dependency to core.
- Use structured diagnostic logging at one designated observation point for a failure; avoid duplicate logs across layers.
- Treat error strings as presentation or diagnostic context, not as the basis for control flow.
- Keep sensitive information out of error messages and logs.

## Constraints

- `skywhale-core` must not depend on a potential `skywhale-db` adapter or `sqlx`.
- `anyhow` may be used for internal propagation and at a top-level execution boundary, but it must not erase classifications before a caller has completed relevant policy decisions.
- The set of semantic errors must remain small and derived from concrete behavior.

## User or stakeholder scenarios

### Duplicate value during persistence

A database adapter recognizes a known unique-constraint violation and maps it to a semantic conflict error. An upper layer maps that error to the appropriate response without inspecting a database code.

### Temporary persistence outage

An adapter detects a pool timeout or connection failure that is classified as temporary unavailability. An upper layer applies its configured retry or availability policy.

### Unclassified persistence failure

An adapter receives an unexpected SQLx error. The public result becomes a generic repository failure, while the original cause and operation context remain available through a source chain or a single structured log entry.

### Final application failure

At an application entry point where no further classification is needed, the error is converted to or propagated as `anyhow::Error` with contextual diagnostic information and reported once.

## Options considered

### Expose SQLx errors from core contracts

This retains all driver detail but reverses dependency direction and couples core callers to the persistence implementation.

### Wrap every database code in project-specific errors

This makes the project responsible for mirroring an evolving external error surface and provides little value for codes that do not change application policy.

### Use `DatabaseError(anyhow::Error)` as the core fallback

This hides the driver dependency but erases policy classification and creates an unnecessary `anyhow` dependency in core. It is acceptable as an adapter-local convenience when no structured handling is required.

### Semantic core errors with selective adapter mapping

Core exposes only application-relevant error categories. Adapters translate known external failures and preserve or log unknown causes. This maintains dependency direction and supports both recovery and diagnostics.

## Decision and rationale

Adopt semantic core error contracts implemented with `thiserror`. Each contract exposes only the small set of failure categories that affect policy. Persistence adapters own SQLx inspection and conversion.

For an unclassified failure, the default public category is a generic failure, not a driver-specific or string-only error. If the cause must reach a common observer, core may accept an opaque standard-library error source such as `Box<dyn std::error::Error + Send + Sync>`. Otherwise, the adapter must record the raw failure and safe operation context immediately before conversion.

Use `anyhow` at internal convenience boundaries and at the final execution boundary after all policy-relevant classification has occurred.

## Acceptance criteria

- A future core crate can define and return repository or use-case errors without depending on SQLx or a database adapter crate.
- A database adapter can map a known duplicate constraint, missing row, and temporary outage into separate semantic categories.
- Unknown SQLx errors do not require a dedicated core variant or database-code mirror.
- Upper layers can decide retry, conflict response, or generic failure without downcasting SQLx or parsing error strings.
- Diagnostic information is either preserved as an opaque source or logged exactly once at the chosen observation boundary.
- No user-facing message requires exposing a raw database error or sensitive operational value.

## Assumptions and open questions

- The first concrete feature will determine the initial semantic error set.
- The location of the common observation boundary depends on the future delivery mechanism.
- Retry count, backoff, idempotency requirements, and logging framework remain to be decided during implementation planning.

## Status

- Specification status: approved
- Date: 2026-07-18
