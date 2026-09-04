# Project Agent Instructions

This repository is designed to be worked on by coding agents.

## Mandatory startup sequence

For every task:

1. Read `AI_START_HERE.md`.
2. Determine the smallest set of project documents needed for the task.
3. Read only those relevant documents.
4. If the task changes code or tests, read `00_project/governance/AI_TESTING_GOVERNANCE_RULES.md` before testing.
5. Inspect the relevant implementation before editing.
6. Make the smallest change that satisfies the requirement.
7. Run only the minimum validation justified by the change.
8. Review `git diff` before finishing.

Do not treat `AGENTS.md` as the complete product specification. It is a router and a set of engineering constraints.

## Requirement hierarchy

When instructions conflict, use this order:

1. The user's current explicit request.
2. Explicit acceptance criteria or current PRD requirements.
3. Project architecture/interface documentation.
4. Existing code behavior and tests.
5. General engineering conventions.

Do not silently invent product behavior to fill documentation gaps.

## Scope control

Unless explicitly requested:

- Do not refactor unrelated code.
- Do not rename unrelated symbols/files.
- Do not reorganize directories.
- Do not upgrade dependencies.
- Do not change build systems.
- Do not rewrite working code merely because another design looks cleaner.
- Do not move or rename project documentation automatically.
- Do not create extra abstractions for hypothetical future needs.

## Testing

`00_project/governance/AI_TESTING_GOVERNANCE_RULES.md` is mandatory for any decision about what to test, what tests to add, and how broadly to run tests.

Testing is validation of the requested change, not an opportunity to redesign the project.

## Git

Unless explicitly requested:

- Do not commit.
- Do not push.
- Do not create branches.
- Do not rewrite history.
- Preserve unrelated local changes.

## Completion report

At completion, report:

- Change summary.
- Files changed.
- Validation/tests run.
- Anything not completed, if applicable.

Keep the report short.

对于文档内容有不理解和不知道怎么做的地方向我提问。
不同文档内容之间有矛盾之处，或者后续我的开发指令中如果有和文档不一致的地方，你要指出来，然后让我裁决。
