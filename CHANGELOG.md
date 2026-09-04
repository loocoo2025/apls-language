# 变更日志

## [0.1.5] - 2026-08-29

> 正式稳定版本；Tag、GitHub Release 与完整 Commit 必须保持一致。

### 新增
- 增加岗位交互与可执行治理权威政策，以及对应的机器可读治理合同目录。
- 增加 Dynamic Role Profile、Knowledge Manifest、Rule Gap、Interaction Contract / Operation 和通用 Authorization 生命周期。
- 增加 `INDEPENDENT_REVIEW / CONTEXTUAL_REVIEW / SELF_REVIEW / HUMAN_DETERMINATION` 四条审核与裁决运行线。
- 增加只保留给 Human Project Owner 的 Formal Seal，以及 `PROCEDURAL_FALLBACK / TOOL_ENFORCED` 双执行保障模式。
- 增加正式 Task 状态转换矩阵和 C04 独立性证据字段。

### 变更
- 稳定分离规则扩展为 `Role != Model != Runtime != Harness != Session != Tool`；C00～C06 仍为固定标准岗位。
- 项目启动改为最小必要知识加载、按需检索和规则缺口失败关闭，不再要求每个岗位默认读取全部治理文件。
- 逻辑 C00 在 Model / Runtime / Harness 切换时保持连续；物理上下文根据证据执行 Knowledge Continuation Check 或 Baseline Relearn。
- Baseline Adoption 仍允许 C00 执行，但只限正确 Owner 对精确 Candidate 和 `BASELINE_ADOPTION` Action 完成预授权、必要 Gate 已通过且 Current Truth 未变化的情形。
- 新系统边界、公共接口、跨系统依赖、安全/数据完整性设计和重大不可逆架构取舍统一进入 Human Determination。

### 修复
- 独立 Session 请求改为分别表达 Session 创建、正式 C04 Dispatch 与真实 Model 调用的适用授权，并对 `NOT_APPLICABLE` 进行失败关闭校验。
- Dynamic Role Profile 机器合同补齐当前或适用 Gate 与适用事实 Owner 绑定及其就绪性校验。
- Requirement、Architecture、Design 与 Code Review Template 统一支持 Finding 的 `Default Route` 字段。

### 兼容与升级
- 不改变 Current Truth、One Fact One Owner、C00～C06 基本职责、V 模型、ADR、Testing Governance、Traceability、C04 Finding/Decision Matrix 或既有产品事实。
- 从 v0.1.4-beta.1 采用时必须增加两个新治理 Authority / Contract 文件，生成当前 Dynamic Role Profile / Knowledge Manifest，选择 Enforcement Mode，映射现有 Task 状态，并补齐当前 Interaction / Authorization。
- 本版本改变启动、权限、交互、Review Line、Task 和 Session 连续性语义；正式采用后 `BASELINE_RELEARN: REQUIRED`。
- `v0.1.5` 是稳定发布目标；已有项目采用时仍必须锁定精确 Tag 与完整 Commit，并按治理升级协议完成 Candidate、正式 C04、显式 Baseline Adoption 和 Baseline Relearn。

## [0.1.4-beta.1] - 2026-08-27
### 新增
- 增加可配置的保障/评审频率权威规则，并固定不可关闭控制与强制正式 C04 触发类别。
- 增加模型无关的外部 AI 交互配置，区分当前 Session 内 Auxiliary/Advisory 调用与新独立 Session 创建。
- 增加负责人决策包、Gate Package、Work Package、完整反馈闭环以及 Candidate Baseline 采用记录字段。

### 变更
- 建立持续逻辑 C00 控制通道；物理 Session 仍按上下文阈值和独立性触发受控交接。
- 正式 C04 可评审可检索、可复现的当前或历史精确 Commit，结论只适用于该精确目标。
- 明确 `GOVERNANCE_MIGRATION_COMMITTED -> READY_FOR_BASELINE_ADOPTION -> GOVERNANCE_UPGRADE_COMPLETE`，并限定 C04 后仅记录后代 Commit 的适用条件。
- 统一负责人审批说明、首次术语解释、阈值审批、四种决定结果、文档变更分类、影响范围复核与语义别名处理。
- Prerelease 升级必须显式选择精确 Tag 和 Commit，并保留稳定版本与回滚锚点。

### 修复
- 统一启动顺序：始终先读 `AI_START_HERE.md`，启动提示不再维护竞争性阅读清单。

### 升级
- `v0.1.3` 项目直接使用既有治理升级协议；`v0.1.0`～`v0.1.2` 项目先以只读方式取得最新独立升级协议，再执行同一受控语义迁移；`UNKNOWN_LEGACY` 无法安全识别治理边界时停止。
- 本 Prerelease 不会被 `LATEST_STABLE_VERSION` 选中。采用时必须显式提供 `ALLOW_PRERELEASE: YES`、`TARGET_VERSION: v0.1.4-beta.1` 和发布后公布的完整精确 Commit，并验证 Tag、Release 元数据和 Commit 一致。
- 升级只改变治理体系，不自动改变项目所处 M5 或其他阶段，不重开已经完成的产品阶段，并保留 PRD、SRS、ADR、架构、代码、测试和产品 Current Truth。
- 本版本改变启动、Session、权限路由和保障频率语义，采用后 `BASELINE_RELEARN: REQUIRED`；采用前必须保留可回滚锚点。

## [0.1.3] - 2026-08-25
### 新增
- 将现有术语表扩展为中英文专有名词、缩写、编号前缀、治理角色、分级和受控状态索引。
- 增加可复用的跨版本治理升级执行协议，支持已知版本和 `UNKNOWN_LEGACY` 项目升级到精确指定版本或上游最新稳定版本。

### 变更
- 将治理模板升级记录扩展为完整执行流程：自动解析稳定 SemVer Tag、逐版本读取迁移信息、分类合并治理文件并形成独立本地 Commit。
- 增加只读 `UPGRADE_READINESS_GATE`，在工作区不干净、正式 C04 进行中、Current Truth 冲突、目标版本不可验证或存在未授权 Breaking Change 时停止升级。

## [0.1.2] - 2026-08-25
### 新增
- 增加最小 Review Decision Matrix 和正式 Review Readiness 前置状态。
- 为 Code Review 增加标准 Finding 记录、Severity、关闭状态和 Advisory 结构。

### 变更
- 分离 `QUESTION_PRIORITY / WORK_PRIORITY` P0～P3 与 `C04_FINDING_SEVERITY` S0～S3。
- 统一 Requirement、Architecture、Design 和 Code Review 的 Readiness、Finding、Advisory 与 `PASS / CHANGES_REQUESTED` 语义。
- 明确 Open Finding 阻断 `PASS`，非阻断项使用 Advisory / Observation / Future Improvement，正式 Exception 仍由现有 Decision / Risk Owner 批准。

## [0.1.1] - 2026-08-24
### 修复
- 澄清 `Role != Model != Harness != Tool`，明确高能力模型和工具调用不产生正式决策权或治理角色。
- 明确辅助调用不等于正式 C04，并规定子 Agent、子模型和工具不得扩大调用者授权。

## [0.1.0] - 2026-08-24
### 新增
- 增加 `Primary Executor + Expert Escalation + Independent Reviewer + Human Authority` 可配置执行路由。
- 增加 `SUPERVISED_AUTO` 默认 Autonomy Envelope、最小 Escalation Package 和模型替换恢复规则。
- 执行槽位增加 Model/Harness 双维配置，并完整定义 `MANUAL_GATE / SUPERVISED_AUTO / FULL_AUTO` 授权边界。
- 增加 Apache License 2.0、贡献指南、安全政策和 Full/Lite 开源采用指南。
- 准备独立干净历史的首次公开 Release Candidate。
### 变更
- C00～C06 与具体 Model/Harness 解耦；当前路由和授权只由 `CURRENT_STATE.md` 维护。
- P0/P1 默认先进入 Expert Escalation；修改 Current Truth、改变产品目标、降低 Acceptance Threshold、接受重大风险或执行未预授权重大 Gate/Release 时请求项目负责人。
- C04 强制使用全新独立上下文，并允许高能力 Reviewer fallback 而不改变评审标准。
- C04 形成 Finding 后停止且不得自行关闭，由 Primary Executor / C00 组织 Expert 整改和新独立 C04 复审。
- 旧项目迁移提示同步采用 Expert 优先、人工权威事项再转负责人的路由。
- 重写仓库首页与启动说明，明确框架定位、架构、五分钟入门、Agent 支持和 Full/Lite 差异。
### 修复
-
