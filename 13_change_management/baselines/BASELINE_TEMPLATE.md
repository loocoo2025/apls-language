# 基线记录

基线名称：
日期：
Git Commit：
版本：
Baseline ID：
Baseline Type：PRODUCT / GOVERNANCE / MIXED
Baseline State：DRAFT / CANDIDATE / CURRENT / SUPERSEDED / ARCHIVED
Upstream Version（治理 Baseline 适用时）：
Upstream Commit（治理 Baseline 适用时）：

## 包含
- PRD：
- SRS：
- ADR：
- 架构：
- 详细设计：
- 测试：
- 构建产物：

## Candidate 证据

- Candidate 建立依据：
- 迁移 / Change Record：
- 精确 Review Target：
- 正式 Review Record / Decision：
- Remaining Risks：

## `CANDIDATE -> CURRENT` 采用

- 采用决定：ADOPTED / NOT_ADOPTED / DEFERRED
- 采用执行者：Human Project Owner / C00 / {{EXISTING_BASELINE_OWNER}}
- Authority Owner：
- Authorization Contract ID：
- 授权依据与精确 Action：`BASELINE_ADOPTION`
- 授权绑定的 Candidate ID / Target / Scope / Validity：
- 必要 C04 / 验证 / 采用前 Gate 证据：
- Open Finding：0 / {{COUNT}}
- 授权消费事件与终态：
- 当前事实与授权签发时一致：YES / NO / UNKNOWN
- 采用日期：
- 采用 Commit / Record-only Descendant：
- `POST_C04_RECORD_ONLY_DESCENDANT`：NO / YES
- Record-only 文件白名单与零漂移证据：
- Baseline Relearn：REQUIRED / RECOMMENDED / NOT_REQUIRED
- Baseline Relearn 状态：NOT_STARTED / IN_PROGRESS / COMPLETE / NOT_APPLICABLE

> C00 仅可在 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 第 5.1 节全部条件满足、且正确 Owner 对精确 Candidate 预授权 `BASELINE_ADOPTION` 时执行采用。模糊“批准”、过期授权、事实变化或缺失字段均不得用于采用。

## Formal Seal（如适用）

- 是否要求：YES / NO
- Seal 状态：NOT_REQUESTED / HUMAN_ISSUED / NOT_ISSUED
- Seal ID：
- 精确 Target / Purpose / Scope：
- Human Project Owner 明确决定与证据：

> Baseline Adoption 不自动产生 Formal Seal；Formal Seal 也不自动授权 Baseline Adoption、Release 或远程副作用。

本文件是 Baseline 记录实例模板，不改变 Baseline 的事实所有权。当前有效 Baseline 仍只由 `00_project/ai_context/BASELINE_INDEX.md` 指向；Candidate 不能在没有显式采用证据时自行成为 Current。
