# White Paper: AI-assisted artifact-driven workflow

## Problem and context

AI collaboration commonly turns a stated goal into a research summary, a proposed design, and an implementation plan. This is efficient but omits the intermediate investigation, experiments, tests, and interface decisions through which a design earns confidence.

The intended work style instead begins with a high-level direction and makes intermediate results first-class. Each result informs the next work unit and may revise the emerging design.

## Objective

Support an AI-assisted workflow in which a person sets the direction, delegates small bounded work units, reviews their artifacts, and composes the next units of work as the design becomes more concrete.

## Non-goals

- Fully autonomous decomposition and execution of a project without human direction.
- Treating an exploratory conclusion as an approved implementation decision.
- Replacing intermediate evidence with a single final design document.
- Defining a concrete software architecture or implementing tools in this record.

## Requirements

- Work must be decomposable into small, bounded units with explicit inputs and expected artifacts.
- Intermediate artifacts must be available as inputs to later investigation, implementation, testing, abstraction, composition, and integration work.
- The high-level direction, constraints, and unresolved questions must remain visible while the lower-level composition evolves.
- Design decisions must be revisable when new implementation or test evidence appears.
- The record must connect a purpose, the evidence gathered, and the resulting change in design judgement.
- A transition from exploration to specification, planning, or implementation must be explicit.
- The workflow must distinguish detailed development, which establishes or revises foundations bottom-up, from general development, which composes established foundations top-down.
- When general development reveals a foundational uncertainty or breaks an existing boundary, the affected work must return to detailed development rather than inventing an unvalidated local solution.

## Constraints

- AI outputs may be incomplete or provisional and require human review.
- The process should remain lightweight during early exploration; it should not impose a complete requirements template prematurely.
- Records should capture decision-relevant changes, not full transcripts.

## User or stakeholder scenarios

### Authentication direction

A project needs an authentication capability. The workflow investigates relevant approaches and decision criteria, creates a minimal foundational implementation and tests, extracts a generalized interface from validated behavior, composes service-level flows, and then integrates them with project conventions and operational needs.

### Evolving design

An early research artifact suggests a suitable direction. A later test exposes a constraint, causing the interface and service composition to change. The record shows why the earlier direction changed instead of presenting only the latest design.

### Two development tracks

When a project has established its harness and conventions, a routine feature uses general development: existing skills, project knowledge, and interfaces are composed top-down. When the work introduces an unanswered question about a foundational technology, boundary, or interface, detailed development investigates and validates that portion bottom-up before the result re-enters the general-development path.

## Options considered

### Goal-to-design agentic flow

An agent receives a goal, researches, recommends a design, and proceeds toward implementation. This is fast but tends to hide intermediate evidence and makes design evolution harder to inspect.

### Artifact-driven composition flow

A goal directs the work, while AI performs bounded tasks that yield explicit artifacts. Human review and those artifacts guide the next composition. This preserves the user's preferred balance of top-down direction and bottom-up validation.

## Decision and rationale

Use an artifact-driven composition flow. Treat AI primarily as an executor and synthesizer of bounded work units, rather than an autonomous owner of project direction. Keep the direction top-down, but let research, implementation, and tests supply evidence that refines the design from the bottom up.

Classify work by the uncertainty it carries. Use detailed development to create or change the reusable harness, and use general development to apply that harness. This avoids repeating foundational investigation for routine work while preserving a clear path for revisiting assumptions when a project boundary changes.

## Acceptance criteria

- A work topic can be expressed as a direction plus an initial bounded task.
- Each completed task produces an artifact that can inform the next task or revise the design.
- A reviewer can reconstruct the relationship between the goal, material evidence, and design changes.
- A design is not considered implementation-ready merely because an AI proposed it; appropriate investigation or validation artifacts support it.
- The workflow can record meaningful checkpoints without requiring a full specification at every step.

## Assumptions and open questions

The workflow assumes a human remains involved in direction setting and approval. The taxonomy of work units, the templates for their artifacts, checkpoint indexing, and the automation boundary remain open.

## Status

- Specification status: approved
- Date: 2026-07-18
