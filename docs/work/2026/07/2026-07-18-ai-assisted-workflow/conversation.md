# Conversation: AI-assisted artifact-driven workflow

## Context

The user described a preferred way of advancing work: set a top-down direction, delegate small AI-assisted work units, use their outputs to refine the design, and repeatedly compose the next units of work. The goal of the discussion was to distinguish this style from a typical autonomous-agent workflow and retain the resulting direction.

## Confirmed decisions

- The `conversation` skill should support open-ended, keyword-led exploration before formal specification or implementation.
- The desired AI collaboration model preserves intermediate work rather than collapsing directly from a goal to a recommended design.
- Small components are AI-delegated work units with clear inputs and expected artifacts, not necessarily code or product components.
- The overall direction is held top-down; bottom-up evidence and artifacts refine the design and determine the next composition of work.
- A useful record must preserve the connection between purpose, evidence, and changes in design judgement.
- Exploration records should be distinct from a final implementation plan or a white-paper-level specification.
- Development is classified into two complementary tracks: detailed development and general development.
- Detailed development proceeds bottom-up to establish or revise foundations; general development proceeds top-down by composing established skills, project knowledge, and interfaces.
- A general-development task that exposes an unanswerable foundational question should split that portion into the detailed-development track.

## Questions and answers

### What is the role of the small components?

They are bounded work units delegated to AI. Their outputs are used to improve the design and select or compose subsequent work.

### How does the approach progress for an example such as authentication?

It starts with an intended outcome such as authentication; investigates relevant approaches and criteria; implements and tests foundational technology; generalizes proven interfaces; composes services from those interfaces; and integrates the result into the project.

### Where does it differ from a typical agentic workflow?

Typical agentic flows often compress the path into goal, recommendation, and implementation. The preferred flow retains investigation, experiments, tests, and interface extraction as explicit intermediate artifacts.

### How do detailed and general development differ?

Detailed development addresses uncertain technology, boundaries, or interfaces. It builds evidence bottom-up through investigation, experiments, implementation, and tests until reusable foundations exist. General development works within those established foundations, composing existing skills, knowledge, interfaces, and project conventions top-down to deliver a requested change.

## Assumptions

- Human judgement remains responsible for setting or changing the high-level direction.
- AI may propose, execute, and synthesize bounded work units, but it should not silently promote an exploratory result into an approved implementation plan.
- The workflow may later be supported by dedicated skills or other tooling, but no tooling architecture was decided in this conversation.
- A task may move between the two tracks as its uncertainty changes; the classification is about the kind of unanswered question, not task size.

## Open questions

- What common artifact types and templates should be used for each work stage?
- How should accumulated checkpoints be indexed so that prior evidence can be reused across projects?
- Which composition decisions should require explicit human approval, and which can be automated?
- Whether the repository's local `conversation` skill should become the default workflow source instead of the global skill.

## Approval

- Status: approved for white-paper creation
- Date: 2026-07-18
