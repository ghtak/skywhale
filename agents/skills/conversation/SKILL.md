---
name: conversation
description: Explore an idea through open-ended, keyword-led dialogue; progressively clarify the user's intent and preserve concise records of meaningful discoveries. Use before planning or implementation, when the user wants to think aloud, find direction, or retain the reasoning behind an emerging direction.
---

# Conversation

Use this skill for exploratory dialogue, not formal requirements gathering. The purpose is to help the user discover what they want by following their keywords, reactions, and emerging priorities.

Do not force a problem statement, implementation plan, acceptance criteria, or decision before the conversation is ready. Do not edit product code or begin implementation under this skill.

## Conversation approach

1. Begin from the user's words. Reflect the core idea in plain language and offer a small number of relevant angles, examples, tensions, or adjacent concepts that can help the user react.
2. Ask one focused question at a time only when it will meaningfully move the discussion forward. Prefer inviting a choice, comparison, or concrete example over a requirements checklist.
3. Treat statements as provisional unless the user clearly confirms them. Gently distinguish:
   - **signals** — interests, dislikes, examples, and constraints the user has expressed;
   - **working hypotheses** — interpretations that may guide the next question;
   - **open threads** — promising questions not yet resolved.
4. Periodically give a brief synthesis when a direction starts to emerge: what seems important, what remains unclear, and the most useful next thread. Keep it conversational rather than bureaucratic.
5. When the user has found enough direction, explicitly ask whether to continue exploring, create a record, turn it into a specification, or plan implementation. Do not silently promote exploration into a settled plan.

## Track record

Create a record when the user asks to save or summarize the discussion, or confirms that a meaningful checkpoint should be retained. A record captures the evolution of thinking; it is not a white paper or an implementation specification.

Unless the user specifies another location, create or update:

```text
docs/work/YYYY/MM/YYYY-MM-DD-<topic-slug>/
  conversation.md
```

Use the local date and a short lowercase hyphenated topic slug. Never overwrite an unrelated record. For a continued conversation about the same topic, append a new dated checkpoint instead of rewriting prior thinking.

Use this compact structure:

```md
# Conversation: <topic>

## Starting point
## Signals from the conversation
## Working hypotheses
## Direction emerging
## Open threads

## Checkpoint: YYYY-MM-DD
### What changed or became clearer
### Why it matters
### Next possible moves
```

Record only decision-relevant ideas and changes in direction, not a verbatim transcript. Clearly label unresolved thoughts as provisional. Do not create a `white-paper.md` unless the user explicitly asks to turn the exploration into a specification.

## Boundaries

- Stay in exploration while uncertainty is productive.
- Do not assume that enthusiasm for an idea is approval to implement it.
- If the user asks to move forward, summarize the emerging direction and ask for the appropriate next-step approval when it would cause meaningful changes.
