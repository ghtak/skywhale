# Core Error Contract Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expose one extensible `skywhale_core::Error` type that preserves unclassified errors and their `anyhow` context while reserving future variants for policy-relevant failures.

**Architecture:** Add a focused `error` module containing the public non-exhaustive error enum and re-export it from the crate root. The transparent `Other(anyhow::Error)` variant is the catch-all path for failures without caller-specific handling; future semantic variants share the same enum only when they require a distinct policy. The core never logs—application entry points log unhandled `Other` chains after semantic handling is complete.

**Tech Stack:** Rust 2024 edition, `thiserror` 2, `anyhow` 1, Cargo unit tests.

## Global Constraints

- Add `anyhow = "1"` and `thiserror = "2"` as direct dependencies of `skywhale-core`.
- Do not add `sqlx`, a logging framework, a database adapter, or a delivery framework to `skywhale-core`.
- The only initial public variant is `Error::Other(#[from] anyhow::Error)` with `#[error(transparent)]`.
- Mark the enum `#[non_exhaustive]`; future variants are permitted only when callers need a distinct recovery or response policy.
- Do not branch on error display text or expose `Other`'s text to end users.
- Log an unhandled `Other` error chain only at a future application entry point, after policy-relevant variants have been handled.

---

## File Structure

- Modify: `skywhale/skywhale-core/Cargo.toml` — declares the two direct error-handling dependencies.
- Modify: `skywhale/skywhale-core/src/lib.rs` — declares the private implementation module and re-exports the public contract.
- Create: `skywhale/skywhale-core/src/error.rs` — owns `Error`, its conversion behavior, and focused unit tests.

### Task 1: Declare error-contract dependencies

**Files:**
- Modify: `skywhale/skywhale-core/Cargo.toml`

**Interfaces:**
- Consumes: Cargo workspace at `skywhale/Cargo.toml`.
- Produces: `anyhow` and `thiserror` for the core crate; `anyhow::Error` is accepted by the public `Error` conversion in Task 2.

- [ ] **Step 1: Write the dependency specification**

Replace the empty dependency table with:

```toml
[dependencies]
anyhow = "1"
thiserror = "2"
```

- [ ] **Step 2: Fetch and compile the dependency graph**

Run: `cargo check -p skywhale-core`

Expected: Cargo resolves `anyhow` and `thiserror`, then completes with `Finished` and no compiler errors.

- [ ] **Step 3: Verify prohibited direct dependencies remain absent**

Run: `cargo tree -p skywhale-core --depth 1`

Expected: the direct dependency list contains `anyhow` and `thiserror`; it contains neither `sqlx` nor a logging crate.

- [ ] **Step 4: Commit**

```bash
git add skywhale/skywhale-core/Cargo.toml skywhale/Cargo.lock
git commit -m "build: add core error dependencies"
```

### Task 2: Define and export the catch-all error contract

**Files:**
- Create: `skywhale/skywhale-core/src/error.rs`
- Modify: `skywhale/skywhale-core/src/lib.rs`

**Interfaces:**
- Consumes: `anyhow::Error`, `thiserror::Error` from Task 1.
- Produces: `skywhale_core::Error`, a `std::error::Error` implementation, and `From<anyhow::Error>` for use with `?`.

- [ ] **Step 1: Write the failing public-API test**

Create `skywhale/skywhale-core/src/error.rs` with this test before the implementation:

```rust
#[cfg(test)]
mod tests {
    use super::Error;
    use anyhow::Context;
    use std::error::Error as StdError;

    #[test]
    fn converts_anyhow_errors_transparently() {
        let source = anyhow::Error::msg("connection refused")
            .context("loading account record");
        let error = Error::from(source);

        assert_eq!(error.to_string(), "loading account record");
        assert!(StdError::source(&error).is_some());
    }

    #[test]
    fn question_mark_converts_anyhow_errors() {
        fn load() -> Result<(), Error> {
            Err(anyhow::anyhow!("connection refused"))
                .context("loading account record")?;
            Ok(())
        }

        let error = load().expect_err("the test operation must fail");
        assert_eq!(error.to_string(), "loading account record");
    }
}
```

- [ ] **Step 2: Run the new test module to verify it fails**

Run: `cargo test -p skywhale-core error::tests -- --nocapture`

Expected: FAIL because module `error` and type `Error` have not been implemented.

- [ ] **Step 3: Implement the public contract**

Replace `skywhale/skywhale-core/src/lib.rs` with:

```rust
mod error;

pub use error::Error;
```

Add this implementation above the tests in `skywhale/skywhale-core/src/error.rs`:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
```

- [ ] **Step 4: Run focused tests to verify conversion and source behavior**

Run: `cargo test -p skywhale-core error::tests -- --nocapture`

Expected: both tests PASS. The first proves transparent display and source availability; the second proves `anyhow::Context` and `?` work without manual conversion.

- [ ] **Step 5: Run the crate test suite and lint checks**

Run: `cargo test -p skywhale-core`

Expected: PASS with the two error-contract tests and no remaining arithmetic-example test.

Run: `cargo clippy -p skywhale-core --all-targets -- -D warnings`

Expected: PASS with no warnings.

- [ ] **Step 6: Review the public surface**

Run: `cargo doc -p skywhale-core --no-deps`

Expected: PASS; generated documentation exposes `skywhale_core::Error` and does not expose the internal `error` module.

- [ ] **Step 7: Commit**

```bash
git add skywhale/skywhale-core/src/lib.rs skywhale/skywhale-core/src/error.rs
git commit -m "feat: add core catch-all error contract"
```

### Task 3: Record the contract boundary for future variants

**Files:**
- Modify: `docs/superpowers/specs/2026-07-18-core-error-design.md`

**Interfaces:**
- Consumes: `skywhale_core::Error` from Task 2.
- Produces: a verified implementation note defining when a future semantic variant may be added and where unhandled errors are logged.

- [ ] **Step 1: Add the implementation verification record**

Append this section to the design document:

```markdown
## Implementation verification

The core contract exposes only `Error::Other` initially. A future variant must
document the caller policy it enables and include a test for that policy. An
unhandled `Other` error is logged once by the application entry point; core and
adapters do not emit duplicate logs for it.
```

- [ ] **Step 2: Verify the recorded constraints match the implementation**

Run: `rg -n "Other|semantic variant|logged once" docs/superpowers/specs/2026-07-18-core-error-design.md skywhale/skywhale-core/src/error.rs`

Expected: the design document and error implementation both identify `Other` as the initial catch-all path; no logging call appears in the core implementation.

- [ ] **Step 3: Commit**

```bash
git add docs/superpowers/specs/2026-07-18-core-error-design.md
git commit -m "docs: record core error contract verification"
```

## Final Verification

- [ ] Run `cargo fmt --check` from `skywhale`.
- [ ] Run `cargo test --workspace` from `skywhale`.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings` from `skywhale`.
- [ ] Run `git diff --check` from the repository root.
- [ ] Review `git status --short` and confirm only the planned files plus Cargo.lock changed; preserve the pre-existing user changes outside this scope.
