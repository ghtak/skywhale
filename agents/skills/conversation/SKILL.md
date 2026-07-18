---
name: conversation
description: Explore an idea through open-ended, keyword-led dialogue; clarify intent, verify decision-relevant facts, and preserve concise records of meaningful discoveries. Use before planning or implementation when the user wants to think aloud, find direction, compare approaches, or retain the reasoning behind an emerging direction.
---

# Conversation

Use this skill for exploratory dialogue, not formal requirements gathering. Help the user discover what they want by following their keywords, reactions, and emerging priorities.

Do not force a problem statement, implementation plan, acceptance criteria, or decision before the conversation is ready. Do not edit product code or begin implementation under this skill.

## Conversation approach

1. Read repository guidance and relevant prior records. Begin from the user's words; reflect the core idea and offer a small number of useful angles, examples, tensions, or adjacent concepts.
2. Verify facts when they materially affect a design judgement. Treat research as evidence, not as a decision; state important uncertainty or limitations.
3. Ask one focused question at a time only when it will meaningfully move the discussion forward. Prefer a choice, comparison, or concrete example over a requirements checklist.
4. Treat statements as provisional unless the user clearly confirms them. Distinguish:
   - **signals** — interests, dislikes, examples, and constraints expressed by the user;
   - **working hypotheses** — interpretations that guide the next question;
   - **evidence** — verified facts that inform a judgement;
   - **open threads** — promising questions not yet resolved.
5. Periodically synthesize the emerging direction: what seems important, what is confirmed, what remains unclear, and the most useful next thread. Keep this conversational rather than bureaucratic.
6. When enough direction has emerged, ask whether to continue exploring, save a conversation checkpoint, turn it into a specification, or plan implementation. Do not silently promote exploration into a settled plan.

## Conversation record

Create or update a record only when the user asks to save or summarize the discussion, or confirms that a meaningful checkpoint should be retained. A record captures the evolution of thinking; it is not a white paper or an implementation specification.

Unless the user specifies another location, create or update:

```text
docs/work/YYYY/MM/YYYY-MM-DD-<topic-slug>/
  conversation.md
```

Use the local date and a short lowercase hyphenated topic slug. Never overwrite an unrelated record. For a continued conversation about the same topic, append a dated checkpoint instead of rewriting prior thinking.

Use this compact structure:

```md
# Conversation: <topic>

## Starting point
## Signals from the conversation
## Evidence consulted
## Working hypotheses
## Direction emerging
## Open threads

## Checkpoint: YYYY-MM-DD
### What changed or became clearer
### Why it matters
### Next possible moves
```

Record decision-relevant ideas and changes in direction, not a verbatim transcript. Attribute confirmed decisions to the user when useful. Clearly label unresolved thoughts as provisional. Record sources or the limits of evidence only when they materially influenced the direction.

## Specification record

Create a `white-paper.md` only when the user explicitly asks to turn the exploration into a specification. Do not infer this from a request to save or summarize the conversation.

The white paper must separate accepted requirements from assumptions and open questions. Keep it implementation-independent and stop after creating or updating it; a separate planning or implementation request is required to continue.

## Boundaries

- Stay in exploration while uncertainty is productive.
- Do not treat research conclusions or enthusiasm for an idea as approval to implement it.
- Perform only read-only investigation needed for the conversation; follow repository guidance before any activity with side effects.
- If the user asks to move forward, summarize the emerging direction and ask for the appropriate next-step approval when it would cause meaningful changes.
