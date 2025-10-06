# Implement Plan (Execute FDR)

## Introduction

Translate an approved FDR into small, reviewable code edits that satisfy Baseline Standards and align with repository workflows.

## Inputs

- fdr_path: path to the approved FDR
- task_name: short human-readable goal (used by reports)
- scope: impacted areas (files, packages, platforms)
- env: required environment variables or secrets

## Outputs

- Working code changes merged via small commits

## Preconditions

- FDR gates passed (completeness, traceability, feasibility, alignment)

## Setup

- Ensure local environment is ready: `pnpm install`, iOS/Android tooling if needed

## Implementation Loop

Repeat the following for each small, independent change:

1. Prepare and scope

- Identify the minimal set of files to change
- Add or update tests first when feasible

2. Make the edit

- Implement changes with clear, readable code and TypeScript strictness
- Avoid introducing new `any` types; respect Expo/React Native constraints

3. Validate locally (can be run together)

```bash
pnpm lint && pnpm typecheck && pnpm test
```

4. Dev Server Verification

- Start dev server: `pnpm start`
- Confirm the app renders without redboxes and shows the home screen
- Navigate core flows affected by this change
  - For navigation changes: switch tabs, open the `Profile` modal, go back
  - Validate no `<StackNavigator>` or navigator nesting errors appear
- Platform-specific checks
  - Android back button navigates back and closes modals before exiting
  - Deep links open the correct screen (if applicable)

5. Commit the change

```bash
jj commit -m "<conventional commit message>" <changed files>
```

6. Iterate

- Move to the next small change until the feature is complete

## Parallelization Guidance

- You may run `pnpm lint`, `pnpm typecheck`, and `pnpm test` in parallel via your shell

## Checklists

- Before coding
  - FDR approved and linked
  - Branch created and up to date with base
- During implementation
  - Small, isolated edits with tests
  - Lint/typecheck/test green
- Before commit
  - Dev Server Verification passed
  - Conventional Commit message used
- After commit
  - `project-plan.md` updated if status changes

## Rollback Plan

- Keep changes small to allow easy git/jj revert
- If a change causes regressions, revert the single commit, fix in a follow-up branch, and re-run the review cycle
