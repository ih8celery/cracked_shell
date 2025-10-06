# Review Code (A2A + A2H)

## Introduction

Perform code review for staged changes and produce artifacts for both agent-to-agent (A2A) and agent-to-human (A2H) review flows.

## Inputs

- report_dir: current report directory
- staged_changes: diff of currently staged files
- platforms_tested: list of platforms verified (Web/iOS/Android)
- base_branch: branch being targeted for merge

## Outputs

- Updated `reports/*/review-comments.json` with findings, recommendations, responses, outcome
- Generated HTML reviews for staged and/or current changes

## Review Gates (must pass for approval)

- Baseline Standards: `pnpm lint`, `pnpm typecheck`, `pnpm test` pass
- Dev Server Verification checklist completed for impacted flows
- No error-level lint issues introduced
- Conventional Commit message plan ready
- Documentation alignment checked (`project-plan.md`, relevant docs under `docs/`)

## Parallel Subagent Reviews (run in parallel)

- Preference Reviewer: evaluate changes against accumulated comment history and expressed maintainer preferences; flag deviations and suggest alignments
- Simplicity Reviewer: assess code for simplicity and clarity; propose refactors that reduce complexity while preserving behavior
- Pattern Reviewer (expert pattern recognition): ensure solutions align with established patterns already accepted in the codebase; recommend applying existing patterns to similar problems in fixes and new features
- Database Performance Reviewer: review usage against current database choices (e.g., Supabase/SQLite); identify query inefficiencies, caching opportunities, and migration risks
- Security Reviewer: analyze inputs/outputs, data handling, and dependencies for security issues; note Expo Go constraints and mobile-specific risks
- Architecture Reviewer: evaluate how the change fits with current architecture; note conflicts, erosion risks, or places where ADRs/docs must be updated

For each subagent, provide:

- Scope: files/areas considered
- Findings: concise list with severity
- Recommendations: concrete, actionable steps
- References: code paths and/or ADR/doc links

## A2A Review Steps

1. Run validations

```bash
pnpm lint && pnpm typecheck && pnpm test
```

2. Inspect diff and scope

- Confirm only intended files are staged
- Verify no scope creep or unrelated changes

3. Assess correctness and design

- Type safety, error handling, and control flow clarity
- React Native/Expo constraints respected (no unsupported native deps without dev client)
- Performance and readability tradeoffs noted

4. Record findings

- Update `review-comments.json` with: findings, recommendations, responses, outcome (approve/request changes)

## A2H Review Steps

1. Add human-readable comments

- Populate `review-comments.json` with `{ file, line?, severity, title?, body, agent, at }`
- Focus on architecture, potential gotchas, and business logic clarity

2. Confirm documentation alignment

- Ensure any architectural changes are reflected or queued for docs

## Re-Review Cycle

- Implementor addresses feedback and updates staged changes
- Repeat validations and update `review-comments.json`
- Only approve when all gates pass

## Checklists

- Validation
  - Lint/typecheck/test green
  - Dev server smoke test notes captured
- Diff quality
  - Minimal scope, clear intent
  - No debug logs or stray TODOs (track in `project-plan.md`)

## Repository Integration Commands

```bash
# Generate staged review HTML
pnpm review:html:staged | cat

# Full review HTML for current report
pnpm review:html | cat
```

## Approval and Handoff

- Approve only when: gates pass, artifacts updated, and Conventional Commit message is ready
