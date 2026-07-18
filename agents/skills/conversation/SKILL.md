---
name: conversation
description: Facilitate an initial work-discovery conversation and turn agreed requirements into `conversation.md` and `white-paper.md`. Use when a user wants to clarify a task before planning or implementation, document requirements and decisions, or create a dated task record. Do not use for implementation, detailed execution planning, or testing.
---

# Conversation

Use this skill only for discovery and specification. Produce a concise, factual record of the conversation and an implementation-independent white paper. Stop after the white paper; do not create an implementation plan, edit product code, or run tests.

## Discovery workflow

1. Read repository guidance and existing task documentation before creating files.
2. Establish the task name, desired outcome, users or stakeholders, scope, constraints, non-goals, and measurable completion criteria.
3. Ask focused follow-up questions only for material unknowns. Keep a visible distinction between confirmed decisions, assumptions, and open questions.
4. Summarize the proposed scope in chat and obtain the user's acceptance before creating the task documents. Do not treat a request to explore an idea as approval to specify it as settled.
5. Create the documents after approval and report their paths. If the task remains materially ambiguous, stop with the unresolved questions instead.

## File placement

Unless repository guidance or the user specifies otherwise, create both files under:

```text
docs/work/YYYY/MM/YYYY-MM-DD-<task-slug>/
  conversation.md
  white-paper.md
```

Use the local date unless the user specifies a different date. Use a short lowercase hyphenated task slug. Do not overwrite an existing task record; use a distinct slug or ask the user how to proceed.

## Required documents

Create `conversation.md` with this structure:

```md
# Conversation: <task title>

## Context
## Confirmed decisions
## Questions and answers
## Assumptions
## Open questions
## Approval
- Status: approved for white-paper creation
- Date:
```

Capture decision-relevant content, not a verbatim transcript. Attribute decisions to the user when useful.

Create `white-paper.md` with this structure:

```md
# White Paper: <task title>

## Problem and context
## Objective
## Non-goals
## Requirements
## Constraints
## User or stakeholder scenarios
## Options considered
## Decision and rationale
## Acceptance criteria
## Assumptions and open questions
## Status
- Specification status: approved / pending approval
- Date:
```

State accepted decisions as requirements. Keep unresolved matters explicitly unresolved; do not invent technical choices or implementation details.

## Completion boundary

After creating or updating the two documents, stop. Tell the user that a separate planning or implementation request is needed to continue.
