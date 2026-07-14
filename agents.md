# AGENTS.md

This file defines the high-level collaboration rules for humans and AI agents
working in this repository.

## Purpose

Use this document as a project-level guide, not a detailed playbook. It should
help an LLM make good judgment calls without forcing every task through a heavy
process. Detailed, repeatable workflows should live in Recipes or dedicated
Skills. Project-specific facts, decisions, and verified examples should live
in project knowledge or Memory docs. Agents should use those workflows and
knowledge sources when they fit the task and the user's intent.

Coding agents are execution harnesses for the user's intent. Agents must not
begin meaningful implementation work from their own assumptions. Meaningful
changes require either a concrete user instruction that defines the requested
change, or user acceptance of an agent-proposed spec or plan. If the request is
ambiguous, clarify the user's intent first, then proceed only after the user has
accepted the intended spec or plan.

## Working Principles

- Clarify intent before changing code.
- Keep work scoped to the requested goal.
- Read existing code before changing code.
- Prefer existing project structure, style, and module boundaries.
- Prefer small, local changes before considering broad redesigns.
- Implement reusable pieces when reuse is natural, but avoid speculative
  abstraction.
- Do not add ornamental, unrelated, or verbose implementation details on behalf
  of the user.
- Make architectural boundaries explicit when they matter to the requested
  change. Keep core behavior separate from framework glue, persistence,
  networking, external services, and other infrastructure concerns where the
  existing codebase supports that separation.
- Prefer dependency direction that keeps domain or core logic independent from
  delivery mechanisms and implementation details.
- Do not apply Clean Architecture, Hexagonal Architecture, or additional layers
  mechanically. Use boundary-oriented structure only when it clarifies
  ownership, reduces coupling, or improves testability for the requested work.
- Make testability and verification part of the design.
- Tests should cover behavior, not implementation trivia.
- Preserve unrelated user or teammate changes.
- Avoid broad refactors unless they are explicitly part of the task.
- If intent or assumptions are unclear, ask the user instead of filling gaps
  with agent-side assumptions.
- Record assumptions, trade-offs, and residual risks clearly.
- Treat this file as repository-level guidance for scope, intent, and judgment.
  Treat dedicated skills and project docs as detailed workflows for specific
  kinds of work.
- If instructions appear to conflict, follow direct user instructions first.
  Treat a concrete instruction to make a specific meaningful change as approval
  for that change. Then apply this file to preserve the user's intent and
  repository-level principles, while adapting detailed skills or project docs to
  the task's actual scope.

## Baseline Workflow

Use this section as the repository baseline when no more specific workflow
applies, and as the scope and judgment frame when using dedicated skills or
project docs.

For meaningful changes, keep this shape in view:

1. Understand the goal, scope, assumptions, and done criteria.
2. For meaningful changes that are not already specified by a concrete user
   instruction, capture the intended spec in a plan, issue, design note, PR
   description, or concise chat summary, depending on scope, and get user
   approval.
3. Plan the change at the level needed for the task.
4. Implement the smallest coherent, testable change.
5. Verify with appropriate tests or checks. Prefer dedicated skills or project
   docs for detailed verification workflows.
6. Refine for clarity, simplicity, and maintainability.
7. Review what changed, how it was verified, what residual risk remains, and
   whether the result still matches the original intent.

Agents may inspect files, run non-destructive checks, reproduce issues in a
non-destructive way, and gather context before proposing a plan. They must not
perform meaningful changes until the intended approach has been accepted, and
must ask first before investigation steps that mutate persisted state, depend on
external services, perform networked actions, or have destructive side effects.
