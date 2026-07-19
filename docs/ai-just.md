# AI-friendly project commands

Use `just` from the repository root or any subdirectory. It finds the root
`Justfile` and runs the Rust workspace commands consistently.

## Safe default workflow

After a source change, run:

```powershell
just check
```

`check` runs the following validation-only recipes in order:

| Recipe | Purpose |
| --- | --- |
| `just format-check` | Checks formatting without modifying source files. |
| `just lint` | Runs Clippy for all targets; warnings fail the command. |
| `just test` | Runs the test suite. |
| `just build` | Compiles the workspace without running it. |

Each recipe returns a non-zero exit code on failure. Read and address the
failure before changing the `Justfile` or weakening a check.

## Guardrails for AI agents

- Prefer `just check` after implementation; use an individual recipe to narrow
  down a failure.
- `format-check` is intentionally read-only for source files. Do not replace it
  with `cargo fmt` unless source formatting changes are explicitly requested.
- Do not add recipes that deploy, delete data, run migrations, or use secrets
  without an explicit human approval step.
- Keep recipe names stable and descriptions concise so humans, CI, and agents
  share the same workflow.
