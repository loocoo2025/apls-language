# HDP-APLS-025 — APLS 0.1 产品 Baseline Adoption

```yaml
determination_id: HDP-APLS-025
status: APPROVED
selected_option: "Q1: A（采用 fd8b5953 为 APLS-0.1-BASELINE-001）；Q2: A（授权提交治理回执并 Push 私有远程）"
decision_owner: HUMAN_PROJECT_OWNER
prepared_date: 2026-09-06
approved_date: 2026-09-06
```

## 1. 必须决定什么

**Q1（主问）**：是否将精确 Commit `fd8b59536fcdfdff2f3b199b882c15d97edb1993`（Tree `c672fe950c4ef9a75af0fe4430254982a5ab175b`，256 文件）采用为 APLS 0.1 产品 Baseline（Baseline ID：`APLS-0.1-BASELINE-001`），即需求、语言规范、Canonical IR、编译器实现、测试与追溯层由候选转为 CURRENT。

**Q2（附带，独立 Action Class）**：是否授权 C00 将 Anchor 后治理回执（`CURRENT_STATE.md`、`BASELINE_INDEX.md`、本 HDP 状态更新、复审记录 `FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001.md` 等当前未提交文件）提交为一个新 Commit，并将 `main` Push 到私有远程 `loocoo2025/apls-language`。

## 2. 为什么必须由人裁决

- 正式 Baseline Adoption 默认由 Human Project Owner 明确采用；当前无覆盖本 Baseline 的精确预授权（`ROLE_INTERACTION_EXECUTION_POLICY.md` §5.1）。
- Commit 与 Push 是相互独立的 Action Class，不包含在 Q1 内。
- Release 与 Formal Seal 不在本 HDP 范围内。

## 3. 确认事实

- Baseline 成立条件逐项核对（`BASELINE_INDEX.md`）：
  1. 需求候选完成并通过适用质询：PRD 经 HDP-APLS-001 批准使用，F-06 措辞经 HDP-APLS-024 Q2=B 修订，F-08 已建立 AC/SYS/NFR/IF 最小完备集（27 ID、30 条 FORMAL_TRACE 边，`validate_traceability.py` PASS）；
  2. 精确 Git Anchor 存在：`fd8b5953...`（本地，未 Push）；
  3. 语言边界与验收阈值明确：`apls-zh-CN-0.1` 契约全套经 DEC-023 采用，F-02 冲突边界经 HDP-APLS-024 Q1=A 冻结；
  4. 适用的独立评审与验证完成：`VAL-APLS-C05-020 = PASS`；首次正式 C04 `FORMAL_C04_APLS_0_1_CANDIDATE_001`（FULL_SCOPE）与关闭复审 `FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001` 均由全新独立 Session 完成；
  5. Open Finding 为 0：复审判定 F-01～F-09 全部 CLOSED，无越权改动，三时点 Target 核验一致。
- 编译器验证现状：Compiler 36 PASS / 1 ignored（Unicode 官方一致性用例，需联网，既往 C05 已 PASS 并继承）、CLI 7 PASS / 1 ignored（T0 资源边界黑盒，既往已验证并继承）、MSRV 1.86 check 通过、追溯门禁 PASS。
- 已知残留（不阻断，如实列明）：两条 ignored 用例本轮未重跑（按 INHERITED_CLOSED 继承）；Advisory A-03/A-04 仅记录；Q-003（P2，参考后端绑定）保持 OPEN；远程 `origin/main` 仍停在 `65ee4eb`。

## 4. 选项

### Q1

- **Option A（推荐）**：采用 `fd8b5953...` 为 `APLS-0.1-BASELINE-001`。后果：Baseline 成立；后续普通改动进入增量治理；版本 `0.1.0-draft` 可摘掉 draft 标记（落盘时执行）。
- **Option B**：暂不采用。后果：维持无产品 Baseline 状态；需说明理由与期望的追加证据。

### Q2

- **Option A（推荐）**：授权提交治理回执并 Push `main` 到私有远程。后果：远程与本地一致；私有仓库性质不变，不等于发布。
- **Option B**：仅本地保留。后果：远程滞后于本地两个以上 Commit。

## 5. 风险与取舍

- Baseline Adoption 后，任何偏离都需走正式变更/复审链；这是预期约束而非风险。
- 两条 ignored 用例依赖既往证据继承；若要求本轮重跑，可拒绝 Q1 并要求先补验证（需联网下载 Unicode 官方数据）。

## 6. 推荐

Q1=A，Q2=A。

## 7. 各结论后果

- Q1=A：C00 将 `BASELINE_INDEX.md` 候选组成转为 CURRENT、登记 Baseline ID 与日期，更新 `CURRENT_STATE.md`；不自动触发 Release。
- Q1=B：维持现状，记录理由。
- Q2=A：C00 提交一个治理回执 Commit 并 Push；提交内容仅限治理/评审记录文件。
- Q2=B：本地保留，远程不变。

## 8. 明确不授权事项

即使全部批准，也不授权：Release、Tag、Formal Seal、公开仓库、邀请协作者、修改已评审 Target `fd8b5953...` 的任何内容（改写历史绝对禁止）、任何新产品功能开发。

## 9. 权威来源

- `05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md`、`05_reviews/FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001.md`
- `00_project/ai_context/BASELINE_INDEX.md`、`DECISION_INDEX.md`（DEC-028～DEC-033）、`CURRENT_STATE.md`
- `11_validation/validation_reports/TASK020_C05_CNL_VERIFICATION.md`

## 10. 可复制回复

```text
HDP-APLS-025: APPROVED
Q1 (BASELINE_ADOPTION): A | B
Q2 (COMMIT_AND_PUSH): A | B
```
