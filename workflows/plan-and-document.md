# Plan and Document Workflow

## Purpose

Provide background and implementor agents with a repeatable recipe for producing
planning documents that match the clarity and rigor of the existing
`project-plan.md`. The goal is to make progress easy for humans to audit while
keeping future agents aligned with repository conventions.

## Canonical References

- Treat `project-plan.md` as the blueprint for tone, structure, and status
  tracking. Mirror its use of headings, priority calls, and implementation
  summaries.
- Review `AGENTS.md` for workflow expectations and quality gates.
- Link back to supporting docs (`architecture.md`, `docs/architecture/`,
  relevant `reports/` entries) whenever decisions reference them.

## Change Complexity Levels

### Minimal Change

- Applies to copy tweaks, one-file refactors, or low-risk build fixes.
- Keep the plan lightweight: note the intent, impacted file, and quick test/
  lint checks.
- Flag any follow-up work if the change uncovers debt but do not over-plan –
  the goal is fast, auditable documentation.

### Normal Change

- Covers most product enhancements and multi-file bug fixes.
- Follow the full plan structure below: architecture recap, phased tasks, and
  implementation summaries aligned with `project-plan.md` conventions.
- Call out risks, add acceptance criteria, and propose regression tests that
  match the surface area touched.

### Hard Change

- Triggers when the work reshapes critical flows (e.g., designing auth,
  altering schema, introducing new services) or spans multiple disciplines
  (documentation, data, frontend, backend).
- Start by interrogating the requirement: document open questions, related ADRs
  or reports, and any assumptions that need validation before coding.
- Allocate timeboxed research spikes and record references (internal docs,
  industry guides, experiments) so the plan captures due diligence.
- Add an **Impact + Risk Audit** subsection that maps user impact, data
  migration considerations, best-practice requirements, and compatibility with
  existing conventions. Explicitly document how you will avoid user-facing
  regressions or data loss.
- Produce a **Full Task Plan** that enumerates every discrete change stream
  (data, API, services, UI, docs, rollout) with owners, dependencies, and
  checkpoints. This plan should include discovery/research work alongside
  implementation tasks.
- Define a **Testing Strategy** that spans unit, integration, end-to-end, and
  manual smoke coverage. Call out tooling gaps and how to mitigate them before
  shipping.
- Outline a **Maintenance Strategy** describing how the change will lower
  long-term upkeep: documentation updates, observability hooks, migration
  rollbacks, or automation that keeps the feature affordable to maintain.
- Reconfirm best practices and coding conventions at each step; if deviations
  are required, note the rationale and follow-up work.

## Planning Workflow

### Required Sections

Include the following in this order so plans mirror `project-plan.md`:

1. **Instructions for AI** — short purpose statement (`project-plan.md:1-9`).
2. **Maintenance / Roadmap** — themed priorities; keep `Production Readiness`
   framing for production planning.
3. **Architecture Analysis** — current patterns, gaps, and references
   (`project-plan.md:11-48`).
4. **Phase Roadmaps** — numbered phases with task cards per phase.
5. **Completed Tasks** — dated log of shipped work.
6. **Features Summary** — `New`, `Approved`, `Proposed`, `Completed` entries
   referencing reports.

### Authoring Flow

1. Gather context from the latest plan, `AGENTS.md`, and supporting docs.
2. Capture the current state and risks before proposing execution.
3. Shape phases around week-sized goals and note dependencies.
4. Populate task cards (priority, effort, files, acceptance criteria) and update
   summaries once work lands.
5. Log outcomes in **Completed Tasks** and cross-link reports or ADRs.
6. Proofread formatting, typography, and tone against the canon.

## Task Cards

````markdown
##### Task X.Y: Short Title ✅/🚧/⏭️ Status

**Priority**: Critical | High | Medium | Low  
**Effort**: estimated duration or size  
**Files**: key paths to touch  
**Description**: one-sentence objective

**Acceptance Criteria**:

- Requirement — DONE / IN PROGRESS / TODO
- ...

**Implementation Summary**:
- ✅ Completed action with detail
- ⚠️ Known limitation or follow-up
- **Result**: Concise outcome statement
````

- Use emojis consistently: `✅` done, `🚧` in progress, `⏭️` skipped, `⚠️` caveat,
  `PARTIAL` for partial acceptance criteria.
- Capitalize verdicts (`DONE`, `DEFERRED`, `SKIPPED`) and surface blockers with
  inline `⚠️` notes.
- Implementation summaries must cite tangible code changes and finish with a
  bold `Result` describing impact (`project-plan.md:122-175`).

## Progress & Handover

- Record each merged change in **Completed Tasks** using `YYYY-MM-DD` dates, and
  add short descriptions in chronological order (`project-plan.md:360-372`).
- Log new initiatives under **Proposed Features** with concise summaries and
  report slugs before execution begins.
- Maintain **Approved / Proposed Features** entries with report paths, command
  snippets, and status updates (`project-plan.md:402-432`).
- Move features to **Approved** or **Completed** when work lands and sync the
  plan card status.
- Update `project-plan.md` and linked docs or FDRs alongside any major change so
  humans can audit progress.

## Quality Checklist

- Headings and sections match the canonical order.
- Every acceptance criteria list uses `-` bullets with verdicts in all caps.
- Implementation summaries reference tangible artifacts (files, hooks,
  services) and include a closing `Result` statement.
- Dates follow `YYYY-MM-DD` in Completed Tasks entries.
- Features sections reference report directories and key scripts.
- Links use repository-relative paths.
