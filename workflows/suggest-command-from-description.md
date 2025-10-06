# Suggest Command From Description

## Introduction

Given a natural-language goal, synthesize safe, idempotent terminal commands tailored to this repository and environment.

## Inputs

- goal: natural-language description of the desired action
- cwd: working directory (default repo root)
- constraints: optional (platform, non-interactive, dry-run)
- context: optional paths or files related to the goal

## Outputs

- Proposed command(s) ready to run (single-line per command)
- Rationale and safety notes
- Any environment prerequisites

## Safety and Quality Gates

- Idempotent where possible; otherwise provide clear rollback guidance
- Non-interactive flags included (e.g., `--yes`, piping to `| cat` for pagers)
- macOS environment assumptions explicit (pnpm preferred for frontend; jj > git if available)
- Avoid destructive operations unless explicitly requested and justified
- Prefer absolute paths for clarity when operating on files within this repo

## Synthesis Steps

1. Parse goal and determine intent

- Read-only vs write vs network operations
- Repository vs system-level actions

2. Choose tools and flags

- Prefer `pnpm` scripts and `make` targets when available
- Use `jj` for SCM if present; otherwise `git`
- Add non-interactive flags; append `| cat` for commands that would page

3. Construct commands

- Use one command per step; chain only when safe and short
- Provide absolute paths for file operations within this repo

4. Validate against constraints

- Ensure commands are safe, idempotent, and compatible with macOS

## Output Template

````markdown
### Proposed Commands

```bash
<command 1>
<command 2>
```
````

### Rationale

- Why these commands and how they satisfy the goal

### Safety Notes

- Idempotence, rollback steps, and potential side effects

### Prerequisites

- Environment or tools required before running

````

## Examples

- Generate staged review HTML

```bash
pnpm review:html:staged | cat
````

- Initialize a new report directory

```bash
pnpm review:init -- --name "Add feature X"
```
