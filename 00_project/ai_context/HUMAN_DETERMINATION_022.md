# HDP-APLS-022 — APLS 0.1 候选首次正式 C04 授权

```yaml
determination_id: HDP-APLS-022
status: APPROVED
selected_option: A
decision_owner: HUMAN_PROJECT_OWNER
prepared_date: 2026-09-04
approved_date: 2026-09-05
```

## 1. 必须决定什么

是否授权以不可变 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e` 为唯一 Review Target，由全新独立 Session 对 APLS 0.1 候选执行首次正式 C04，为后续产品 Baseline Adoption 提供 Gate 结论。

## 2. Review Readiness

```yaml
review_line: FORMAL_C04
reason_code: FORMAL_C04_INITIAL
review_purpose: APLS_0_1_PRODUCT_BASELINE_READINESS
target_commit: 3289037bee1aab64dfa2d58188379a68dcfa601e
target_tree: 51edf42da73237bcb3408e234d2cbb2095758fa1
target_file_count: 249
full_immutable_commit: PRESENT
target_retrievable: YES
target_reproducible: YES
applicable_baseline: APLS_BASELINE_NOT_YET_ESTABLISHED_CANDIDATE_PREPARATION
independent_session: REQUIRED_NEW_SESSION
review_scope: FULL_SCOPE
review_record: 05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md
```

- TASK-018 已完成实现与增量独立复审闭合；TASK-020 的 `VAL-APLS-C05-020 = PASS`。
- Target 是完整本地 Git Commit，Tree、文件数和对象完整性已经核验。
- 当前工作区中的提交后治理回执不属于 Review Target；Reviewer 必须从精确 Commit 重建事实。
- 本次属于首次正式 C04。即使项目默认 `DELTA_ONLY`，现行保障节奏仍要求首次正式 C04 使用 `FULL_SCOPE`。

## 3. 选项

### Option A — 发起首次正式 C04（推荐）

C00 创建一个全新独立 C04 Session；Reviewer 只读审查精确 Commit，并仅在预定义位置写入正式 Review Record。Reviewer 独立检查 Review Readiness、需求与架构、受控自然语言契约、Canonical Frame/IR、Compiler/CLI 实现、测试与追溯闭合，并按风险复核现有 C05 证据。

允许的最小执行边界：

- 从精确 Commit 读取正式文件、源码、测试和治理证据；
- 在 `/private/tmp` 物化该 Commit 的只读审查副本并产生构建输出；
- 运行 Traceability Validator，以及与首次正式 Gate 直接相关的锁定离线构建和测试；
- 仅写入 `05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md`；
- 输出 `REVIEW_NOT_READY`，或在 Ready 后输出 `PASS / CHANGES_REQUESTED`、S0～S3 Finding 和关闭条件。

明确不允许：

- 修改 Review Target、当前 Compiler/CLI Source、测试、设计、Schema、Cargo Manifest 或 Lock；
- Reviewer 参与整改、替自己关闭 Finding，或把非正式复审结论当作正式 Gate；
- Commit、Push、Tag、Branch、PR、Baseline Adoption、Release 或 Formal Seal；
- 使用工作区可移动状态代替精确 Commit，或把 PASS 外推到后代 Commit。

### Option B — 暂缓

保留当前不可变候选，不启动正式 C04；项目维持 `NO_FORMAL_C04_NO_PRODUCT_BASELINE`。

## 4. 完成条件

- 全新独立 C04 Session 和精确 Target 均被正式记录；
- 开始、写入 Review Record 前、结束三个时点均核验 Commit/Tree 身份；
- Reviewer 未修改 Target，仅写入预定义正式 Review Record；
- Review Record 明确范围、证据、Finding、Open Finding 数和唯一 Gate Decision；
- 结果返回 C00；Baseline Adoption 仍须另行裁决，不自动发生。

## 5. 可复制回复

```text
HDP-APLS-022: APPROVED
OPTION: A
```
