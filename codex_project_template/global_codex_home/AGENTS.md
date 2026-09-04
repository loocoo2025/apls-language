# Global Codex Operating Rules

This file defines default behavior for all projects. A repository's own `AGENTS.md` may add more specific rules.

## 1. Work narrowly

- Do only the task that was requested.
- Prefer the smallest correct change.
- Do not perform unrelated refactors, cleanup, renaming, formatting, dependency upgrades, or architecture changes.
- Do not scan the entire repository by default. Read only the files needed to understand and complete the current task.
- Do not invent requirements when the project documentation is silent or ambiguous.

## 2. Respect project instructions

- If the repository contains `AGENTS.md`, follow it.
- If it points to `AI_START_HERE.md`, read that file before implementation.
- Read deeper documents only when they are relevant to the current task.
- More specific project instructions override these global defaults.

## 3. Git safety

Unless the user explicitly asks:

- Do not create or switch branches.
- Do not commit.
- Do not push.
- Do not rebase.
- Do not force-push.
- Do not rewrite history.
- Do not discard unrelated local changes.

You may use read-only Git commands such as `git status`, `git diff`, `git log`, and `git show` when useful.

## 4. Dependency and network safety

Unless the task genuinely requires it:

- Do not add or upgrade dependencies.
- Do not modify lockfiles.
- Do not run package installers.
- Do not access the network.
- Do not change system-wide configuration.

If any of these are necessary, explain why before doing them when approval is required.

## 5. Testing discipline

- Follow the repository's testing policy when present.
- Prefer the smallest targeted validation that can detect mistakes introduced by the change.
- Do not run the full test suite by default.
- Do not create tests merely to increase coverage.
- Do not invent speculative edge cases unrelated to requirements.
- Do not add performance, stress, fuzz, security, compatibility, or end-to-end testing unless the task or risk clearly requires it.

## 6. Output discipline

Keep the final report concise. Normally include only:

1. What changed.
2. Which files changed.
3. What validation/tests were run.
4. Any remaining issue or decision that truly needs human attention.

Avoid dumping long command output unless it is needed to diagnose a failure.


AI 对 stash、branch、tag、commit、历史记录等治理资产，默认不得执行删除性操作。
