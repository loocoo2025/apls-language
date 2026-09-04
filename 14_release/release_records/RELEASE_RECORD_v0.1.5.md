# v0.1.5 发布记录

版本：`v0.1.5`

Commit：由不可变 Tag `v0.1.5^{commit}` 精确解析，并必须与 GitHub Release Target 一致。

发布日期：`2026-08-29`

Release Authorization ID：`HPO-RELEASE-v0.1.5-20260829`

Authority Owner：`Human Project Owner`

授权 Target / Scope：`loocoo2025/ai-engineering-governance` 的稳定版本 `v0.1.5`，限正式 Release Commit、`origin/main`、`v0.1.5` Tag、GitHub Release 和对应 Git archive。

## 发布 Gate

| 检查项 | 结果 | 证据 |
|---|---|---|
| 正式 C04 | RELEASE 前强制执行 | `05_reviews/C04-GOV-001-v0.1.5-release-review.md` 与 GitHub Release |
| Open Finding | 必须为 0 | 最终 Review Record |
| YAML / Markdown / Link / Index | 必须 PASS | Release Gate 机械检查 |
| Git whitespace / fsck | 必须 PASS | Release Gate 机械检查 |
| 敏感信息与产品事实扫描 | 必须 PASS | Release Gate 扫描 |
| 产品 Build / Test | NOT_APPLICABLE | 本版本不修改产品代码 |
| Release Authorization | PASS | Human Project Owner 当前明确指令 |

## 发布包

- GitHub 自动生成的 Source code archives；
- `ai-engineering-governance-v0.1.5.tar.gz`，必须由正式 Tag 使用 `git archive` 生成；
- Archive SHA-256 在 GitHub Release 和最终发布报告中记录。

## Release Notes

- `13_change_management/release_notes/RELEASE_NOTES_v0.1.5.md`

## 已知限制

- 机器合同需要具体 Runtime 或治理工具消费后才能实现 `TOOL_ENFORCED`；
- 无专用工具时继续使用 `PROCEDURAL_FALLBACK` 和明确证据闭环。

## 回滚点

- 上一公开版本：`v0.1.4-beta.1`；
- 回滚通过新的受控 Commit 或下游项目已记录的不可变升级前 Anchor 完成，不移动或删除已发布 Tag。

## Formal Seal

- 状态：`NOT_ISSUED`
- Seal ID：`NOT_APPLICABLE`
- 精确 Target / Purpose / Scope：`NOT_APPLICABLE`
- Human Project Owner 明确决定与证据：本次只授权正式 Release，没有单独签发 Formal Seal。

## Baseline Adoption

- 下游项目 Governance Baseline Adoption：`NOT_PERFORMED`
- 说明：发布上游稳定版本不会自动替任何使用该模板的项目采用新 Baseline。
