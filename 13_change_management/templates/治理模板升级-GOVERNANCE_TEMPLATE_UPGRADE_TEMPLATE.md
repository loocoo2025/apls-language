# 工程治理模板跨版本升级执行协议与迁移记录模板

> 用途：将任何已采用本框架的项目，从其当前治理版本受控升级到指定稳定版本；未指定目标时，升级到上游最新稳定版本。
>
> 本文件同时定义升级执行流程和 `GOV-MIG` 迁移记录结构。执行流程是本文件的稳定事实，项目实际升级结果写入由本文件实例化出的迁移记录。
>
> 这是治理规则迁移，不自动等于产品变更。不得因为模板升级而重新设计或重写已批准的产品、需求、接口、架构、代码或测试。

---

## 0. 事实所有权与适用边界

本文件负责：

- 治理升级前置检查；
- 当前版本识别和目标版本锁定；
- 跨版本差异分析与迁移顺序；
- 治理文件和项目事实的合并边界；
- 升级验证、Commit 和报告格式；
- `GOV-MIG` 迁移记录模板。

本文件不成为新的项目 Current Truth Owner：

```text
当前阶段 / Gate / 授权 / 当前焦点 / 下一步
→ CURRENT_STATE.md

当前 Baseline 身份与组成
→ BASELINE_INDEX.md

当前有效决定
→ DECISION_INDEX.md

当前任务
→ ACTIVE_TASKS.md

当前未决问题
→ OPEN_QUESTIONS.md

对话拓扑与生命周期
→ CONVERSATION_MAP.md

历史迁移事件
→ GOV-MIG 迁移记录 / Git
```

同一动态事实不得为了升级方便复制到多个文件。

本协议适用于：

- 任意旧版到最新稳定版；
- 任意旧版到负责人指定的精确版本和 Commit；
- 已知旧版本；
- 版本记录不完整但仍能安全识别治理文件边界的 `UNKNOWN_LEGACY` 项目；
- Greenfield 和 Brownfield 项目中途治理升级。

“兼容任意版本”表示本协议能够自动发现升级路径、逐版本读取迁移信息并执行安全的语义迁移，不表示 AI 可以自动批准未来的破坏性治理变更。涉及重大 Owner、Current Truth、Acceptance Threshold、产品目标或风险权限变化时，仍必须停止并请求项目负责人批准。

---

## 1. 默认执行参数

调用者未提供覆盖值时，使用以下默认值：

```text
UPSTREAM_REPOSITORY:
https://github.com/loocoo2025/ai-engineering-governance.git

TARGET_SELECTOR:
LATEST_STABLE_VERSION

TARGET_VERSION:
AUTO

TARGET_COMMIT:
AUTO

ALLOW_PRERELEASE:
NO

ALLOW_BREAKING_GOVERNANCE_CHANGE:
NO

AUTONOMY_MODE:
FULL_AUTO

AUTHORIZED_UNTIL:
GOVERNANCE_UPGRADE_VALIDATED_AND_LOCALLY_COMMITTED

ALLOW_LOCAL_COMMIT:
YES

ALLOW_PUSH:
NO

ALLOW_RELEASE:
NO

ALLOW_REMOTE_MUTATION:
NO

ALLOW_PRODUCT_CHANGE:
NO
```

调用者可以明确提供：

- `TARGET_VERSION`；
- `TARGET_COMMIT`；
- 替代的可信 `UPSTREAM_REPOSITORY`；
- 是否允许预发布版；
- 是否已经批准特定破坏性治理变化；
- 是否只评估、不实施；
- 是否允许本地 Commit。

显式目标必须同时解析到精确 Tag 和完整 Commit。只给出浮动 `main`、`HEAD`、`latest` 分支或网页展示短 Hash，不满足正式升级条件。

### 1.1 本次预授权 Gate

```text
PREAUTHORIZED_GATES:
- PHASE_0_READ_ONLY_UPGRADE_READINESS_CHECK
- UPSTREAM_TARGET_RESOLUTION
- CROSS_VERSION_DIFF_ANALYSIS
- GOVERNANCE_SEMANTIC_MIGRATION
- GOVERNANCE_CONSISTENCY_VALIDATION
- BASELINE_RELEARN_PREPARATION
- LOCAL_GIT_COMMIT
```

如果当前项目明确要求治理变更经过正式 C04，则允许在具备真正独立 Session 的前提下运行 C04 循环；无法建立独立 C04 时，不得伪造评审结论，应停在 `READY_FOR_INDEPENDENT_C04`。发起独立 Session 时必须引用 `AI_CONVERSATION_ORCHESTRATION_RULES.md` 第 41.5 节的当前请求 Schema，并分别证明 `INDEPENDENT_SESSION_CREATION`、适用的 `FORMAL_C04_DISPATCH` 和适用的 `REAL_MODEL_INVOCATION` 授权；本协议不复制请求字段。

### 1.2 权限继承

```text
Role != Model != Runtime != Harness != Session != Tool
SUBAGENT_PERMISSION <= CALLER_PERMISSION
AUXILIARY / ADVISORY != FORMAL C04
```

任何子 Agent、Model、Runtime、Harness、Session、CLI、API 或其他工具都不得扩大本协议授权。读取远程上游是允许的只读操作；push、PR、Release、远程设置变更和破坏性操作默认禁止。

---

## 2. 升级目标与不变量

升级目标：

- 使用精确、可复现的上游治理版本；
- 保留当前正式产品事实；
- 补齐新增治理文件、规则和必要状态字段；
- 补齐目标版本要求的 Dynamic Role Profile、Knowledge Manifest、Interaction / Authorization 和 Enforcement Mode；Dynamic Role Profile 必须包含当前或适用 Gate 与适用事实 Owner 绑定；
- 消除旧治理规则与目标版本之间的冲突；
- 保留项目已经批准的本地治理扩展；
- 形成独立、可回退的治理升级 Commit；
- 在继续原项目工作前准备 Baseline Relearn。

整个升级默认保持：

```text
CHANGE_TYPE: GOVERNANCE_CHANGE
PRODUCT_BEHAVIOR_CHANGE: NO
REQUIREMENT_CHANGE: NO
ARCHITECTURE_CHANGE: NO
INTERFACE_CHANGE: NO
CODE_CHANGE: NO
TEST_SCOPE_CHANGE: NO
PRODUCT_CURRENT_TRUTH_CHANGE: NO
GOVERNANCE_BASELINE_CHANGE: YES
RELEASE_CHANGE: NO
```

如果实际迁移必须将任一项改为 `YES`，停止自动升级，单独建立正式 Change，并请求对应 Owner 批准。

---

## 3. Phase 0：只读升级条件检查

阶段名称：

```text
PHASE_0_READ_ONLY_UPGRADE_READINESS_CHECK
```

在输出 `UPGRADE_READINESS: READY` 前禁止：

- 修改、创建、移动或删除项目文件；
- `git add`、`git commit`、`git stash`；
- `git checkout`、`git switch`、`git clean`、`git reset`；
- 自动提交、隐藏或丢弃已有修改；
- 自动完成正在进行的正式 C04；
- 改变项目阶段、Gate、授权或任务状态；
- 在项目仓库内 fetch、merge 或写入上游历史。

上游检查必须在只读缓存或临时目录中完成，不得污染目标项目仓库。临时目录只可删除本次创建且已精确确认的内容。

### 3.1 Git 工作区检查

执行：

```bash
git rev-parse --show-toplevel
git rev-parse --verify HEAD
git branch --show-current
git status --porcelain=v1 --untracked-files=all
git status --short --branch
git diff --check
```

机械判定：

- `git status --porcelain=v1 --untracked-files=all` 输出为空：`WORKTREE_CLEAN: PASS`；
- 存在 staged、unstaged、untracked、deleted、renamed、conflict 或子模块变化：`WORKTREE_CLEAN: FAIL`。

失败时列出精确文件和状态后停止。不得自动 stash、commit、clean、reset 或删除未跟踪文件。

记录：

```text
PRE_UPGRADE_PROJECT_COMMIT: {{FULL_COMMIT_HASH}}
PRE_UPGRADE_BRANCH: {{BRANCH}}
```

分支为空或 Git 状态无法解析时标记 `UNKNOWN` 并停止，除非项目已有明确批准的 detached-HEAD 工作流程。

### 3.2 当前执行 Session 角色检查

如果当前 Session 已经作为 Independent Reviewer 发起，并已绑定冻结 Review Target、精确 Commit、Review Readiness 或正式 Review Record，则：

```text
CURRENT_SESSION_IS_FORMAL_C04: YES
UPGRADE_READINESS: NOT_READY
```

正式 C04 Session 不得自行切换为 C00、Primary Executor 或 Governance Migration Executor。必须使用新的 C00 / Primary Executor Session 执行升级。

普通只读分析、设计咨询、代码咨询或调用某个模型/CLI，本身不构成正式 C04。

### 3.3 项目正式 C04 状态检查

依次只读检查：

1. `CURRENT_STATE.md`；
2. `ACTIVE_TASKS.md`；
3. 当前正式 C04 Review Record；
4. 当前冻结 Review Target；
5. `CONVERSATION_MAP.md`；
6. 最新相关 HANDOFF，仅作辅助证据，不作为 Current Truth Owner。

满足以下任一条件时，判定 `FORMAL_C04_IN_PROGRESS: YES`：

- `CURRENT_STATE.md` 明确记录正式 C04 正在进行；
- 当前 Gate 明确处于 C04 实质评审阶段；
- `ACTIVE_TASKS.md` 存在 `IN_PROGRESS` 的正式 C04 任务；
- Review Target 已冻结、Readiness 为 `READY`，但正式 Review Record 尚无最终 `PASS / CHANGES_REQUESTED`；
- Review Record 明确标记 `REVIEW_IN_PROGRESS`；
- 正式 Reviewer 尚未结束当前独立评审 Session。

如果 Review 已输出 `PASS` 或 `CHANGES_REQUESTED`，且独立 Reviewer 已停止，则原正式 C04 不再进行。存在尚未关闭的 Finding 但项目已进入 Primary Executor 整改阶段，也不等于原 C04 仍在进行；旧 Review Record 仍不得重写。

`CONVERSATION_MAP.md` 中存在生命周期为 `ACTIVE` 的 C04 对话，单独看不足以证明正式 C04 正在进行。对话生命周期与正式 Review 状态不得混为一体。

如果各权威文件互相矛盾、已经存在正式评审启动迹象但对应 Review Record 缺失，或无法确认正式 C04 是否仍在进行：

```text
FORMAL_C04_IN_PROGRESS: UNKNOWN
UPGRADE_READINESS: NOT_READY
```

不得把 `UNKNOWN` 猜成 `NO`。

### 3.4 关键执行窗口检查

确认当前没有：

- 正在执行的 Release；
- 正在执行的生产部署或远程危险操作；
- 尚未形成检查点的重大需求、架构、接口或数据迁移；
- 正在合并或解决冲突的 Git 操作；
- 需要在同一冻结 Review Target 中保持规则不变的活动。

项目不必处于某个固定 M 阶段，但必须位于可回退的干净检查点。升级不得自动推进当前阶段、Gate 或里程碑。

### 3.5 Current Truth 可判定性检查

必须能够明确识别适用项目中的：

- 当前 PRD 和 SRS；
- 当前正式接口；
- 当前 ACCEPTED ADR；
- 当前 As-Is / 已批准架构；
- 当前详细设计；
- 当前代码 Commit；
- 当前测试 Baseline；
- 当前 Baseline；
- 当前阶段、Gate、授权和下一步；
- 当前正式 Review 状态。
- 当前 Dynamic Role Profile、Knowledge Manifest、Interaction / Authorization 和 Enforcement Mode（目标版本要求时）。

尚未建立的对象可以标记 `MISSING / NOT_APPLICABLE`，但不得编造。若同一事实存在多个冲突 Owner 或无法判断当前有效版本：

```text
CURRENT_TRUTH_CHECK: FAIL
UPGRADE_READINESS: NOT_READY
```

停止并列出冲突，等待项目负责人裁决。

### 3.6 前置检查判定矩阵

| 检查项 | 允许继续的唯一结果 |
|---|---|
| Git Repository Valid | `PASS` |
| Worktree Clean | `PASS` |
| Current Session Is Formal C04 | `NO` |
| Formal C04 In Progress | `NO` |
| Critical Execution In Progress | `NO` |
| Clean Checkpoint | `PASS` |
| Current Truth Check | `PASS` |
| Upstream Target Resolution | `PASS` |
| Product/Requirement/Architecture/Interface/Code/Test Change Required | 全部 `NO` |

全部满足：

```text
UPGRADE_READINESS: READY
NEXT_ACTION: BEGIN_GOVERNANCE_MIGRATION
```

存在任一 `FAIL / YES / UNKNOWN`：

```text
UPGRADE_READINESS: NOT_READY
NEXT_ACTION: STOP_AND_REPORT
```

### 3.7 Readiness 输出

开始修改前先输出：

```text
UPGRADE_READINESS_REPORT

PROJECT_ROOT:
CURRENT_BRANCH:
PRE_UPGRADE_PROJECT_COMMIT:
WORKTREE_CLEAN:
CURRENT_SESSION_IS_FORMAL_C04:
FORMAL_C04_IN_PROGRESS:
FORMAL_C04_EVIDENCE:
CRITICAL_EXECUTION_IN_PROGRESS:
CLEAN_CHECKPOINT:
CURRENT_TRUTH_CHECK:
CURRENT_GOVERNANCE_VERSION:
TARGET_GOVERNANCE_VERSION:
UPSTREAM_TARGET_COMMIT:
UPSTREAM_TARGET_RESOLUTION:
BREAKING_CHANGE_DETECTED:
PRODUCT_CHANGE_REQUIRED:
REQUIREMENT_CHANGE_REQUIRED:
ARCHITECTURE_CHANGE_REQUIRED:
INTERFACE_CHANGE_REQUIRED:
CODE_CHANGE_REQUIRED:
TEST_SCOPE_CHANGE_REQUIRED:
UPGRADE_READINESS:
```

`READY` 后按本协议自动继续，不重复请求确认。`NOT_READY` 时不得修改任何文件。

---

## 4. 当前版本识别

按以下顺序确定 `CURRENT_GOVERNANCE_VERSION` 和当前上游锚点：

1. `BASELINE_INDEX.md` 中已经存在的唯一治理框架版本/上游 Commit；
2. 最新一份已完成、未被替代的 `GOV-MIG` 迁移记录；
3. 项目采用模板时记录的精确 Tag / Commit；
4. 当前治理文件与上游历史稳定版本的内容匹配结果；
5. 无法精确识别时标记 `UNKNOWN_LEGACY`。

不得根据文件日期、聊天记忆、目录名称或“看起来像某版本”伪造精确版本。

### 4.1 已知版本

若版本和 Commit 都能验证，使用上游旧版本、上游目标版本和项目当前文件执行三方语义比较：

```text
OLD_UPSTREAM
+ TARGET_UPSTREAM
+ CURRENT_PROJECT
→ CONTROLLED_SEMANTIC_MERGE
```

### 4.2 UNKNOWN_LEGACY

`UNKNOWN_LEGACY` 不必因为缺少版本号自动失败，但必须：

- 建立完整治理文件清单；
- 将现有规则按事实 Owner 分类；
- 使用目标版本逐项做两方语义差异分析；
- 默认保留项目事实和已批准本地扩展；
- 不自动删除无法确认来源的文件；
- 对任何实质冲突标记 `UNKNOWN` 并停止；
- 在迁移记录中明确无法建立精确旧版本 Diff。

若无法区分治理规则与产品事实，则不得继续自动升级。

### 4.3 已经是目标版本或高于目标版本

- 当前版本和上游 Commit 已精确等于目标：输出 `ALREADY_CURRENT`，不创建空 Commit；
- 当前版本高于所选目标：判定为潜在降级，输出 `DOWNGRADE_NOT_AUTHORIZED` 并停止；
- 当前项目含有目标版本之后的本地治理扩展：保留扩展，检查兼容性，不得静默回退。

内部 `vX.Y.Z-candidate` 只表示上游未发布的审核前工作身份，不是可供正式项目采用的 Tag。除非项目负责人明确授权测试该候选、提供精确不可变 Commit 并接受非发布版本风险，否则升级目标必须是正式版本或标准 SemVer Prerelease。

---

## 5. 目标版本解析

### 5.1 显式目标优先

调用者同时给出 `TARGET_VERSION` 和 `TARGET_COMMIT` 时：

1. 从可信上游读取对应 Tag；
2. 对 annotated Tag 解析其 dereferenced Commit；
3. 确认完整 Commit 与调用者提供值一致；
4. 确认该 Commit 可读取且属于上游历史；
5. 不一致则停止。

显式精确目标优先于默认的最新稳定版本选择。

### 5.2 最新稳定版本

`TARGET_SELECTOR: LATEST_STABLE_VERSION` 的稳定定义：

1. 同时读取可信上游 Git Tags 和可用的 GitHub Release 元数据；
2. 候选 Tag 必须严格匹配正式 SemVer `vMAJOR.MINOR.PATCH`，且目标 `CHANGELOG.md` 包含对应版本；
3. 排除带有 `alpha / beta / rc / preview` 等后缀的 Tag；
4. 某 Tag 已关联 GitHub Release 时，Draft 或 Prerelease 标记会将其排除；
5. 没有关联 GitHub Release、但满足正式 SemVer、Commit 可解析且 `CHANGELOG` 有对应版本的 Tag，仍可作为稳定候选；
6. 在全部稳定候选中选择 SemVer 最高版本；
7. 将 Tag 解析为完整不可变 Commit；
8. 同时记录选择时间、Tag、Tag Object（如适用）和 Commit；
9. 不使用浮动 `main / HEAD / latest branch` 作为正式目标。

如果 Release 元数据、Tag 和 Commit 互相冲突，标记 `UPSTREAM_TARGET_RESOLUTION: UNKNOWN` 并停止。

上游不可访问时，不得把本地缓存自动宣称为最新版本。只有负责人提供并批准精确目标版本、Tag Object（如适用）和 Commit，且可信本地副本能够验证三者关系时才可离线继续。

### 5.2.1 显式 Prerelease 目标

Prerelease 永远不得被 `LATEST_STABLE_VERSION` 自动选中。只有同时满足以下条件才允许解析 Prerelease 目标：

```text
ALLOW_PRERELEASE: YES
TARGET_VERSION: {{EXACT_PRERELEASE_TAG}}
TARGET_COMMIT: {{FULL_IMMUTABLE_COMMIT}}
```

执行者必须确认：

1. 调用者明确选择了该 Prerelease，而不是只要求“升级到最新”；
2. Tag 精确匹配 `TARGET_VERSION`，并解析到 `TARGET_COMMIT`；
3. 可用的 GitHub Release 元数据将其标记为 Prerelease，且不是 Draft；
4. 目标 `CHANGELOG.md` 和专用 Migration Notes 明确包含该版本；
5. 已建立升级前回滚锚点；
6. 不使用浮动 Branch、`HEAD` 或“latest beta”作为目标。

Tag、Release 元数据和 Commit 任一不一致时，停止为 `UPSTREAM_TARGET_RESOLUTION: UNKNOWN`。`ALLOW_PRERELEASE: YES` 只授权采用调用者指定的精确 Prerelease，不授权自动选择其他 Prerelease。

### 5.3 目标自身完整性

在临时只读副本中检查：

- 目标 Commit 可解析；
- 目标 `CHANGELOG.md` 和 Release Notes 可读取；
- 目标 `TEMPLATE_FILE_INDEX.md`（如存在）与目标 Git 文件集一致；
- 没有 `.git`、`.DS_Store`、`__MACOSX` 被当作模板内容；
- 目标版本提供的专用 Migration Notes / Upgrade Instruction 已读取；
- 升级路径中的每个中间稳定版本可识别。

目标版本自己的、更具体迁移说明优先于本通用协议的机械默认值，但不得突破调用者权限和本协议的产品事实保护边界。

---

## 6. 跨版本升级路径

如果跨越多个版本：

1. 枚举从当前版本之后到目标版本之间的全部稳定版本；显式选择 Prerelease 时，再把该精确目标 Prerelease 作为路径终点；
2. 按 SemVer 正序阅读每个中间稳定版本和精确目标版本的 `CHANGELOG`、Release Notes 和 Migration Notes；
3. 建立累计差异表；
4. 识别新增、修改、重命名、废弃和删除的治理项；
5. 先按中间版本迁移语义，最终文件以目标版本为准；
6. 不要求在项目 Git 历史中为每个中间版本分别建立 Commit，除非目标迁移说明明确要求；
7. 迁移记录必须列出实际跨越的版本序列。

示例：

```text
v0.1.0
→ 读取 v0.1.1 迁移语义
→ 读取 v0.1.2 迁移语义
→ ...
→ 形成精确目标版本的最终治理状态
```

Prerelease 只允许作为显式精确终点，不得作为中间“最新稳定版本”被自动插入路径。

### 6.1 破坏性或重大治理变化

满足以下任一条件时标记 `BREAKING_CHANGE_DETECTED: YES`：

- Major Version 跨越；
- 改变 One Fact One Owner 分配；
- 新增或改变 Current Truth 来源；
- 改变 C00～C06 基本职责；
- 改变产品、需求、架构或风险批准 Owner；
- 降低或改变 Acceptance Threshold；
- 改变正式 Gate / Release 权限；
- 要求重写已批准项目事实；
- 上游明确标记 Breaking Change 或 Manual Migration Required。

默认 `ALLOW_BREAKING_GOVERNANCE_CHANGE: NO`。未获得负责人对精确变化的明确批准时停止，不得把“升级到最新”解释为接受所有未来破坏性变化。

---

## 7. 文件分类与合并规则

不得把目标模板整个目录直接覆盖到项目。每个差异先分类。

### 7.1 A 类：框架拥有的稳定治理规则

典型包括：

- `AI_START_HERE.md`；
- `AI_ENGINEERING_RULES_V2.md`；
- `AI_CONVERSATION_ORCHESTRATION_RULES.md`；
- Testing Governance；
- Context Reset / Baseline Relearn Rules；
- Role Briefs；
- 独立评审 Prompt；
- Review Templates；
- 治理变更模板；
- 目标版本新增的正式治理规则文件。

处理：

- 未经项目修改的旧上游文件可更新为目标版本；
- 含项目批准扩展的文件必须三方语义合并；
- 项目扩展与新上游冲突时不得静默覆盖；
- 稳定规则不得复制进动态状态文件。

### 7.2 B 类：项目动态事实 Owner

包括：

- `CURRENT_STATE.md`；
- `BASELINE_INDEX.md`；
- `DECISION_INDEX.md`；
- `ACTIVE_TASKS.md`；
- `OPEN_QUESTIONS.md`；
- `CONVERSATION_MAP.md`；
- 当前 Review Records；
- 项目 HANDOFFS。

处理：

- 永远不得整文件覆盖；
- 只合并目标版本新增的字段、约束和引用；
- 保留全部当前项目值和事实所有权；
- 不因为升级自动推进阶段、Gate、任务或对话生命周期；
- 当前治理版本如需进入 Baseline，只在 `BASELINE_INDEX.md` 的 Baseline 组成中维护一次，不复制到 `CURRENT_STATE.md`、`DECISION_INDEX.md`、`ACTIVE_TASKS.md` 或 `CONVERSATION_MAP.md`。

### 7.3 C 类：产品与交付事实

包括但不限于：

- PRD / SRS / Acceptance Criteria；
- 接口和协议需求；
- ADR 正文；
- 架构和详细设计；
- 业务代码；
- 产品测试、测试数据和测试结论；
- 构建、部署和 Release 产物；
- Bug、RCA、现场事实。

默认全部禁止修改。若目标治理版本要求产品文件发生实质变化，停止并建立独立 Change。

### 7.4 D 类：仓库展示与社区文件

包括：

- 产品项目 `README.md`；
- 产品 `CHANGELOG.md`；
- `LICENSE`；
- `CONTRIBUTING.md`；
- `SECURITY.md`；
- 社区和发布展示文件。

不得用框架仓库的公开展示内容覆盖产品项目文件。只有文件仍被项目明确认定为上游治理资产，且不存在产品特有事实时，才按目标迁移说明处理。

### 7.5 新增、重命名和删除

- 新增：只添加目标版本必需的治理文件；不得顺便复制无关模板内容；
- 重命名：旧文件与旧上游精确一致且无引用风险时可受控迁移，否则做语义合并并保留可追溯关系；
- 删除：只有旧文件与旧上游内容一致、目标明确废弃、没有项目事实、没有当前引用且目标迁移说明允许时才可删除；
- 无法确认用途的文件：标记 `UNKNOWN`，不得删除；
- 历史文件：由 Git 保存，不得为了“干净”重写历史。

---

## 8. 正式迁移步骤

只有 `UPGRADE_READINESS: READY` 后执行。

1. 创建 `GOV-MIG-{{DATE}}-{{TARGET_VERSION}}` 迁移记录实例；
2. 记录升级前项目完整 Commit、当前治理版本和目标上游完整 Commit；
3. 建立逐版本迁移路径；
4. 建立新旧规则差异表；
5. 对每个目标差异做 A/B/C/D 分类；
6. 更新 A 类稳定治理文件；
7. 对 B 类动态 Owner 进行最小字段级语义合并；
8. 保持 C 类产品事实不变；
9. 默认不覆盖 D 类产品仓库文件；
10. 增加目标版本必需的新治理文件；
11. 建立或迁移 Dynamic Role Profile、Knowledge Manifest、Interaction / Authorization 和 Enforcement Mode，并对照 Current Truth、Role Brief、Task 与 Authorization 校验当前或适用 Gate 和适用事实 Owner 绑定；
12. 将现有 Task 映射到目标版本状态机，不自动推进状态；
13. 受控处理明确的重命名和废弃项；
14. 检查旧表述是否与目标规则冲突；
15. 检查 One Fact One Owner；
16. 记录所有保留的项目扩展和 Remaining Risks；
17. 根据第 9 节执行验证；
18. 根据第 10 节形成独立本地 Commit；
19. 输出最终报告后停止，不继续产品开发。

### 8.1 历史 Review Record

- 已完成的旧版 Review Record 保留原文和原结论；
- 不用新规则追溯重写已关闭历史；
- 迁移记录注明其形成时采用的治理版本；
- 已结束但仍有 Open Finding 的 Review，保留原 Finding 和关闭条件；
- 整改后按目标版本建立新的精确 Review Target 和独立 C04；
- 不得在同一正式 Review Record 中途切换评审规则。

### 8.2 Baseline Relearn 判定

迁移记录必须机械选择一个结果：

| 结果 | 判定条件 | 后续动作 |
|---|---|---|
| `REQUIRED` | 启动顺序、Current Truth / Baseline Owner、C00～C06 Role、权限、Review / Testing / Traceability、Autonomy、Session / Handoff 或其他执行语义发生实质变化 | 完成 Baseline 采用后、继续产品开发或发起下一次正式 C04 前，必须从升级后的正式文件执行 Baseline Relearn |
| `RECOMMENDED` | 治理语义有实质改进，但不改变当前执行入口、Owner、权限或 Gate 行为 | 在下一个安全检查点完成；未完成前必须记录 Remaining Risk |
| `NOT_REQUIRED` | 只修改术语解释、展示文字、Release 元数据或不影响运行语义的文档 | 记录理由后可继续既有工作 |

不得按文件数量或版本号机械判断。语义影响不明确时标记 `UNKNOWN` 并停止，由正确 Owner 裁决。

本协议引入 Persistent C00、独立 Session 路由、保障频率或外部 AI 权限配置等运行语义的版本，`BASELINE_RELEARN` 必须为 `REQUIRED`。旧聊天记忆不得覆盖新治理规则。

---

## 9. 验证

### 9.1 必做机械检查

```bash
git diff --check
git status --short --branch
git diff --name-status
git diff --stat
```

Commit 后：

```bash
git show --check --stat --oneline HEAD
git status --short --branch
```

### 9.2 必做语义检查

确认：

- 所有中间稳定版本的迁移信息已读取；
- 目标版本和 Commit 已精确锁定；
- 项目 Current Truth 未改变；
- 产品需求、接口、架构、代码和测试结论未改变；
- 当前阶段、Gate 和授权未自动推进；
- A 类治理规则达到目标语义；
- B 类动态事实没有被整文件覆盖；
- C 类产品文件没有变化；
- D 类产品仓库文件没有被框架展示内容覆盖；
- 项目批准的治理扩展仍存在；
- 旧冲突规则已修正或明确阻断；
- 没有新增重复 Current Truth Owner；
- 没有把 C04 绑定到某个 Model、Runtime、Harness、Session 或 Tool；
- 没有把辅助调用宣称为正式 C04；
- 没有扩大子 Agent 或工具权限；
- Dynamic Role Profile / Knowledge Manifest 不成为新的 Current Truth Owner，且 Profile 的当前或适用 Gate、适用事实 Owner 绑定完整、未过期、无冲突；
- Interaction / Authorization 的 Action、Scope、Target、Side Effect、消费事件和终态完整；
- `PROCEDURAL_FALLBACK / TOOL_ENFORCED` 没有形成两套治理语义；
- Task 状态转换符合目标版本的正式状态机；
- 历史 Review Record 未被重写；
- `GOV-MIG` 记录完整。

### 9.3 测试边界

纯 Markdown 治理迁移不运行产品全量测试，也不新增产品测试。

如果目标版本修改了治理脚本、校验器或可执行工具：

1. 先完整阅读目标版本 Testing Governance；
2. 运行能够验证本次治理工具变化的最小目标测试；
3. 不借升级扩大产品测试范围；
4. 记录命令和结果。

### 9.4 变更范围检查

Commit 前必须逐项审查 `git diff --name-status`。发现产品文件、未知文件或无关修改时停止，不得为了完成升级把它们一起提交。

---

## 10. Commit 规则

验证通过且 `ALLOW_LOCAL_COMMIT: YES` 时，创建独立本地 Commit：

```text
chore(governance): upgrade framework from {{FROM_VERSION}} to {{TARGET_VERSION}}
```

当前版本为 `UNKNOWN_LEGACY` 时：

```text
chore(governance): upgrade legacy framework to {{TARGET_VERSION}}
```

Commit 只能包含：

- 目标版本治理迁移；
- `GOV-MIG` 迁移记录；
- 必要的治理状态/基线字段合并；
- 目标版本明确要求的治理文件新增、重命名或安全删除。

不得包含产品文件、无关格式化或用户已有修改。

迁移 Commit 无法在其自身内容中记录自己的最终 Hash。不得为制造自引用 Hash 而反复 amend 或重写历史。精确 `GOVERNANCE_UPGRADE_COMMIT` 由 Git 生成后写入最终报告；后续 Baseline 记录按项目既有 Anchor 规则引用该 Commit。

默认：

```text
PUSH: NO
TAG: NO
PR: NO
RELEASE: NO
REMOTE_MUTATION: NO
```

除非调用者另行明确授权，否则本地 Commit 后停止。

### 10.1 升级状态与 Baseline 采用

迁移 Commit 不等于升级完成。所有升级都必须按以下状态推进：

1. 迁移 Commit 形成后，记录 `GOVERNANCE_MIGRATION_COMMITTED`；
2. 将目标治理 Baseline 建立为 `CANDIDATE`，不得直接覆盖 `CURRENT`；
3. 完成目标版本和项目当前规则要求的 C04、验证或采用 Gate；
4. 所有适用前置条件满足后，记录 `READY_FOR_BASELINE_ADOPTION`；
5. 按 `BASELINE_INDEX.md` 和 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 的既有 Owner 与授权规则显式执行 `CANDIDATE -> CURRENT`；
6. 完成要求的 Baseline Relearn 和采用后检查后，才记录 `GOVERNANCE_UPGRADE_COMPLETE`。

如果项目尚未单独管理 Governance Baseline，迁移记录必须把治理版本和精确 Upstream Commit 作为当前项目 Baseline 的一个受控组成采用，不得以“不单列”为由跳过明确采用结果。

Baseline 采用是否需要 Human Project Owner 再次批准，由项目当前 Baseline 配置和保留决策边界决定。C00 只有在正确 Owner 已对精确 Candidate、`BASELINE_ADOPTION` Action、Scope、Target、Validity、消费事件和终态完成预授权，必要 C04 / 验证已通过、Open Finding 为 0 且 Current Truth 未变化时，才可执行采用。缺少任一条件或涉及产品目标、重大架构裁定、Acceptance Threshold、重大风险、Formal Seal、Release 时，必须请求 Human Determination。

### 10.2 目标或项目规则要求正式 C04 时

本协议不要求每个纯展示性升级都新增 C04。但目标版本的保障节奏策略或项目当前规则明确要求治理升级经过正式 C04 时：

1. 将升级内容 Commit 作为冻结 Candidate Review Target；
2. 立即记录其完整不可变 Commit Hash；
3. 使用新的独立 C04 Session，不继承迁移实现 Session 的私有上下文；
4. C04 只执行 Finding、Severity、关闭条件和 `PASS / CHANGES_REQUESTED`；
5. C04 对精确 Candidate Commit 输出 `PASS` 后，才能进入 `READY_FOR_BASELINE_ADOPTION`；
6. `CHANGES_REQUESTED` 后由 C00 / Primary Executor 在授权内整改并形成新 Commit；
7. 新 Commit 必须作为新的 Review Target，由新的独立 C04 Session 复审；
8. 不得 amend、重写或 force-update 已经接受评审的 Candidate Commit；
9. 无法建立独立 Session 时输出 `READY_FOR_INDEPENDENT_C04`，不得伪造 `PASS`。

如果旧规则与目标规则对本次迁移应使用哪套 C04 判定语义存在冲突，停止并请求项目负责人明确迁移 Gate，不得由执行者自行选择有利规则。

### 10.3 `POST_C04_RECORD_ONLY_DESCENDANT`

正式 C04 的 `PASS` 默认只适用于被评审的精确 Commit。只有同时满足以下条件，才允许把 C04 后的记录 Commit 认定为 `POST_C04_RECORD_ONLY_DESCENDANT`，并在不重跑 C04 的情况下完成 Baseline 采用：

1. Descendant 是被评审 Commit 的直接可验证后代；
2. 变更严格限制在项目预先声明的迁移记录、Review Record、Baseline 采用记录或动态状态 Anchor 文件白名单内；
3. 变更只记录已经发生的 Commit、C04 结论、采用决定和状态引用，不改变任何治理规则或产品事实；
4. 对被评审治理文件、产品文件、测试证据和 Review Target 的机械 Diff 为零；
5. C04 Review Record 明确记录被评审 Commit，采用记录同时记录该 Commit 和 Record-only Descendant；
6. 验证命令、文件白名单、零漂移证据和执行者写入迁移记录。

出现以下任一情况时，该例外立即失效，必须形成新的精确 Review Target 和新的独立 C04：

- 修改被评审治理文件或其语义；
- 修改产品事实、需求、接口、架构、代码、测试结论或 Acceptance Threshold；
- 修改 Review Target、Finding、关闭条件或 C04 Decision；
- 变更文件超出已声明白名单；
- 无法机械证明零漂移。

`POST_C04_RECORD_ONLY_DESCENDANT` 是精确目标继承条件，不是新的 Owner、Role 或宽泛的“文档变更免评审”规则。

---

## 11. 正式停止条件

出现以下任一情况必须停止：

- 工作区不干净；
- 当前 Session 是正式 C04；
- 正式 C04 正在进行或状态为 `UNKNOWN`；
- Current Truth 冲突或 Owner 不明确；
- 无法锁定精确目标 Tag/Commit；
- 上游目标完整性检查失败；
- 检测到未授权 Breaking Change；
- 必须改变产品目标、需求、接口、架构、代码或测试结论；
- 必须降低 Acceptance Threshold；
- 必须接受重大风险；
- 必须执行未授权 Release 或远程操作；
- 目标文件与项目批准扩展发生无法自动裁决的冲突；
- 无法区分治理规则与项目事实；
- 验证失败；
- 独立 C04 为项目硬要求但无法建立真正独立 Session。

停止时输出：

```text
GOVERNANCE_UPGRADE_BLOCKED

BLOCKING_CONDITION:
EVIDENCE:
FILES_AFFECTED:
OWNER_REQUIRED:
SAFE_NEXT_ACTION:
WORKTREE_MUTATED: NO / YES
```

Phase 0 阻断必须保证 `WORKTREE_MUTATED: NO`。

---

## 12. 最终输出

成功时输出：

```text
GOVERNANCE_UPGRADE_REPORT

PROJECT:
PREVIOUS_GOVERNANCE_VERSION:
PREVIOUS_UPSTREAM_COMMIT:
TARGET_GOVERNANCE_VERSION:
TARGET_UPSTREAM_COMMIT:
VERSIONS_TRAVERSED:
PRE_UPGRADE_PROJECT_COMMIT:
GOVERNANCE_UPGRADE_COMMIT:
MIGRATION_STATE:
CANDIDATE_IDENTITY:
C04_REVIEW_TARGET:
C04_DECISION:
BASELINE_CANDIDATE_ID:
BASELINE_ADOPTION_DECISION:
BASELINE_ADOPTION_EVIDENCE:
CURRENT_DYNAMIC_ROLE_PROFILE:
CURRENT_KNOWLEDGE_MANIFEST:
CURRENT_INTERACTION:
CURRENT_AUTHORIZATION:
ENFORCEMENT_MODE:
TASK_STATE_MAPPING:
POST_C04_RECORD_ONLY_DESCENDANT:
CURRENT_STAGE_BEFORE:
CURRENT_STAGE_AFTER:
FILES_ADDED:
FILES_MODIFIED:
FILES_RENAMED:
FILES_REMOVED:
FILES_INTENTIONALLY_NOT_CHANGED:
PROJECT_EXTENSIONS_PRESERVED:
CURRENT_TRUTH_PRESERVED:
PRODUCT_REQUIREMENTS_CHANGED:
INTERFACE_CHANGED:
ARCHITECTURE_CHANGED:
CODE_CHANGED:
TEST_SCOPE_CHANGED:
REVIEW_GOVERNANCE_STATUS:
BASELINE_RELEARN_STATUS:
VALIDATION_COMMANDS:
VALIDATION_RESULTS:
GIT_DIFF_CHECK:
GIT_SHOW_CHECK:
GIT_STATUS:
PUSH_PERFORMED: NO
REMAINING_RISKS:

FINAL_STATUS:
GOVERNANCE_UPGRADE_COMPLETE
```

迁移 Commit 已形成、但尚未完成要求的独立 C04 时输出：

```text
FINAL_STATUS:
GOVERNANCE_MIGRATION_COMMITTED
```

独立 C04 已 `PASS`、但尚未完成 Baseline 采用时输出：

```text
FINAL_STATUS:
READY_FOR_BASELINE_ADOPTION
```

无需升级时输出：

```text
FINAL_STATUS:
ALREADY_CURRENT
```

等待项目既有正式 C04 时输出：

```text
FINAL_STATUS:
READY_FOR_INDEPENDENT_C04
```

阻断时输出：

```text
FINAL_STATUS:
GOVERNANCE_UPGRADE_BLOCKED
```

---

## 13. GOV-MIG 迁移记录实例模板

执行升级时，从本节实例化一份项目级记录。项目已有正式治理迁移目录时沿用现有位置；没有约定时，默认保存为：

```text
13_change_management/change_requests/GOV-MIG-{{YYYYMMDD}}-{{TARGET_VERSION}}.md
```

### 13.1 基本信息

- Migration ID：`GOV-MIG-{{YYYYMMDD}}-{{TARGET_VERSION}}`
- 日期：
- 项目：
- 迁移前模板版本：
- 迁移前上游 Commit：
- 迁移后模板版本：
- 迁移后上游 Commit：
- 跨越版本：
- 迁移前项目 Commit：
- 当前 Baseline ID：
- 当前阶段 / Gate：

### 13.2 分类

```text
CHANGE_TYPE: GOVERNANCE_CHANGE
PRODUCT_BEHAVIOR_CHANGE: NO
REQUIREMENT_CHANGE: NO
ARCHITECTURE_CHANGE: NO
INTERFACE_CHANGE: NO
CODE_CHANGE: NO
TEST_SCOPE_CHANGE: NO
PRODUCT_CURRENT_TRUTH_CHANGE: NO
GOVERNANCE_BASELINE_CHANGE: YES
```

### 13.3 Readiness 结果

| 检查项 | 结果 | 证据 |
|---|---|---|
| Worktree Clean | | |
| Current Session Is Formal C04 | | |
| Formal C04 In Progress | | |
| Critical Execution In Progress | | |
| Clean Checkpoint | | |
| Current Truth Check | | |
| Upstream Target Resolution | | |
| Breaking Change Detected | | |
| Upgrade Readiness | | |

### 13.4 新旧规则差异

| 版本 / 项目 | 旧规则 | 新规则 | 分类 | 迁移动作 |
|---|---|---|---|---|
| | | | A / B / C / D | |

### 13.5 当前事实保留检查

- 当前 APPROVED 产品需求：
- 当前 CONFIRMED 系统需求：
- 当前 ACCEPTED ADR：
- 当前正式接口：
- 当前架构：
- 当前详细设计：
- 当前代码 Commit：
- 当前测试 Baseline：
- 当前发布结论：
- Current Truth 冲突：`NO / YES / UNKNOWN`

### 13.6 文件迁移

| 文件 | 分类 | 动作 | 项目事实是否保留 | 验证 |
|---|---|---|---|---|
| | A / B / C / D | ADD / MODIFY / RENAME / REMOVE / PRESERVE | | |

### 13.6.1 可执行治理迁移

- Dynamic Role Profile（含当前或适用 Gate、适用事实 Owner 绑定及校验证据）：
- Knowledge Manifest：
- 当前 Interaction Contract / Operation：
- 当前 Authorization Contract / 状态：
- Enforcement Mode：`PROCEDURAL_FALLBACK / TOOL_ENFORCED`
- Tool Enforcement / Procedural Evidence：
- 现有 Task 状态映射：
- Rule Gap / Compatibility Alias：

### 13.7 项目扩展与例外

- 保留的项目治理扩展：
- 与目标版本的兼容性：
- 正式批准的例外：
- 未解决冲突：

### 13.8 Baseline Relearn

```text
BASELINE_RELEARN: REQUIRED / RECOMMENDED / NOT_REQUIRED / UNKNOWN
BASELINE_RELEARN_STATUS: NOT_STARTED / IN_PROGRESS / COMPLETE / NOT_APPLICABLE
```

理由：

继续原项目工作前的动作：

### 13.9 验证

- `git diff --check`：
- 目标版本一致性：
- Current Truth 保留：
- One Fact One Owner：
- 产品文件未修改：
- 必要的治理工具测试：
- Remaining Risks：

### 13.10 迁移后状态

- Candidate Governance Baseline ID（如项目适用）：
- Candidate 建立证据：
- `CANDIDATE -> CURRENT` 采用决定：
- 采用执行者 / Authority Owner：
- Authorization Contract ID / Action / Target / Scope / Validity：
- 授权消费事件 / 终态 / 对账证据：
- 采用时间与证据：
- Formal Seal：`NOT_REQUIRED / NOT_REQUESTED / HUMAN_ISSUED / NOT_ISSUED`
- Formal Seal ID / Target（如适用）：
- Current Governance Baseline ID（采用后）：
- 目标 Upstream Commit：
- 治理升级 Commit：由本记录所在 Git Commit / 最终报告解析
- 正式 C04 Review Target：
- 正式 C04 Decision / Record：
- `POST_C04_RECORD_ONLY_DESCENDANT`：`NO / YES`
- Record-only 文件白名单与零漂移证据：
- 当前 PRD：
- 当前 SRS：
- 当前 ACCEPTED ADR：
- 当前架构：
- 当前设计：
- 当前测试：
- 当前阶段 / Gate：保持升级前事实，除非另有正式批准

---

## 14. 给 Codex / AI 的最短调用指令

将本文件提供给 Codex 后，只需发送：

```text
请以 C00 / Primary Executor 身份，完整阅读并严格执行
《工程治理模板跨版本升级执行协议与迁移记录模板》中的升级流程。

使用文件默认上游和 LATEST_STABLE_VERSION，先执行只读
UPGRADE_READINESS_GATE；只有结果为 READY 才自动完成治理语义迁移、
验证并创建独立本地 Commit。不要 push，不要修改任何产品事实；
阻断或状态不明确时保持工作区不变并输出报告。
```

如果要升级到负责人指定版本，追加：

```text
TARGET_VERSION: {{VERSION}}
TARGET_COMMIT: {{FULL_COMMIT_HASH}}
```

如果目标是 Prerelease，还必须追加：

```text
ALLOW_PRERELEASE: YES
TARGET_VERSION: {{EXACT_PRERELEASE_TAG}}
TARGET_COMMIT: {{FULL_PRERELEASE_COMMIT_HASH}}
```
