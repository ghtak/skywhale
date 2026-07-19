# AGENTS.md

This file is a lightweight guide for humans and AI agents working in this
repository. It describes judgement and collaboration boundaries, not a fixed
procedure for every task.

## Purpose

Use this document to keep work aligned with the user's intent while preserving
momentum. Prefer context-sensitive judgement over ceremonial process. Use a
Recipe or dedicated Skill only when a task is high-risk, complex, or repeated
enough that a detailed checklist genuinely helps.

## Working Principles

- Keep work scoped to the user's requested goal.
- Read relevant existing code and documentation before changing them.
- Prefer small, local changes that respect existing structure, style, and
  module boundaries. Avoid broad refactors unless they are requested.
- Clarify material ambiguity instead of filling it with agent assumptions.
- Preserve unrelated user and teammate changes.
- Scale planning, documentation, and verification to the size and risk of the
  change. Do not create records or process merely for their own sake.
- Keep core behavior independent from framework, persistence, networking, and
  external-service details when the existing codebase makes that separation
  useful. Do not add architectural layers mechanically.
- Test behavior rather than implementation trivia when tests are warranted.
- State important assumptions, trade-offs, and residual risks concisely.

## Default Flow

### 1. Explore and discuss

When the goal, constraints, or desired outcome are unclear, discuss them before
implementation. Read-only investigation is welcome when it helps clarify the
context.

Leave a short record only when a decision, rationale, or unresolved question is
likely to matter later. A conversation does not need to become a specification
or a plan by default.

### 2. Plan when it matters

For a meaningful change without a concrete user instruction, propose a
proportionate approach and obtain acceptance before implementing it. A concrete
user instruction authorizes that change; make any necessary assumptions visible.

Use a short chat summary for small work. Use a written plan, design note, or
dedicated workflow only when the scope, risk, or coordination warrants it.

### 3. Implement the smallest coherent change

Implement only what serves the agreed goal. Favor simple, reusable pieces where
reuse is natural, but avoid speculative abstractions and ornamental detail.

### 4. Verify and report

Run tests, builds, reviews, or manual checks that match the change's risk and
surface area. Report what changed, how it was checked, and any remaining risk.

## Higher-Risk Work

Pause for explicit confirmation before actions that are destructive, difficult
to reverse, mutate persistent data, depend on external services, or affect
deployment, security, permissions, or users outside this workspace. Use a
short checklist or dedicated workflow when it reduces the chance of a costly
omission.

Direct user instructions take precedence over this guide.
