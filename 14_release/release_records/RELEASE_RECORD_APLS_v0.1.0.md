# 发布记录

版本：v0.1.0
Commit：`fd8b59536fcdfdff2f3b199b882c15d97edb1993`（Tag `v0.1.0` 精确指向该 Commit，即 `APLS-0.1-BASELINE-001` Anchor）
构建 ID：本地构建（Rust 1.98.0，`cargo test --locked --offline`，2026-09-06）
发布日期：2026-09-06
Release Authorization ID：DEC-038
Authority Owner：HUMAN_PROJECT_OWNER
授权 Target / Scope：私有仓库 `loocoo2025/apls-language` 的 v0.1.0 Tag + GitHub Release；不含公开仓库、Formal Seal

## 发布包

- Git Tag `v0.1.0`（annotated）→ Commit `fd8b5953...`
- GitHub Release `v0.1.0`：<https://github.com/loocoo2025/apls-language/releases/tag/v0.1.0>
- 无二进制分发包；用户按 `docs/APLS_0.1_USER_TUTORIAL.md` §2 从源码构建

## 测试报告

- `cargo test --locked --offline`（在精确发布 Commit 的干净副本上复跑）：Compiler 36 PASS / 1 ignored；CLI 7 PASS / 1 ignored
- `cargo +1.86.0 check --locked --offline`：PASS（MSRV）
- `python3 09_quality/traceability/validate_traceability.py`：PASS
- 既有证据：`VAL-APLS-C05-020 = PASS`；`FORMAL_C04_APLS_0_1_CANDIDATE_001`（FULL_SCOPE）；`FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001 = PASS`（Open Finding 0）

## 已知问题

- 两条 ignored 用例（Unicode 17.0.0 官方一致性、T0 资源边界黑盒）本次未联网重跑，沿用 C05 PASS 证据继承；
- Advisory A-03/A-04 仅记录（不阻断）；
- 后端中立，不绑定参考平台（DEC-037）。

## 回滚点

- 远程删除 Tag/Release 即可撤回发布；本地与远程 `main` 历史不受发布影响；
- 上一稳定点：`65ee4eb`（交接同步）/ Baseline 前候选 `3289037...`。

## Formal Seal（如适用）

- 状态：NOT_ISSUED
- Seal ID：无
- 精确 Target / Purpose / Scope：无
- Human Project Owner 明确决定与证据：未签发；发布授权（DEC-038）不产生 Formal Seal
