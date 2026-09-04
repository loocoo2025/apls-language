# 专有名词、缩写与受控用语表

> 本文件用于快速解释模板中的缩写、编号前缀、角色代码和受控工程用语。
>
> 本文件是解释性索引，不替代各项规则的正式 Owner。如与工程规则、Testing Governance、`CURRENT_STATE.md` 或正式 Review Record 不一致，以相应权威文件为准。
>
> 收录范围：当前模板直接使用的工程缩写和受控名词，以及用户指定的常用扩展缩写。普通布尔值、一次性占位符和普通字段名不逐一收录。

## 1. 文档、需求与交付缩写

| 缩写 | 英文全称 | 中文含义 | 模板中的用途 / 备注 |
|---|---|---|---|
| `PRD` | Product Requirements Document | 产品需求文档 | 产品目标、用户、场景、产品级需求和验收标准的主文档。 |
| `SRS` | Software Requirements Specification / System Requirements Specification | 软件/系统需求规格说明 | 模板将其放在 `02_system_requirements/SRS.md`，承载系统需求基线。 |
| `ADR` | Architecture Decision Record | 架构决策记录 | 记录重大架构选择、备选方案、理由和后果。 |
| `STD` | Standard / Standardization Document | 标准/标准化文档 | 常用扩展缩写；当前模板未将其定义为强制编号前缀。 |
| `ICD` | Interface Control Document | 接口控制文档 | 常用扩展缩写；当前模板以接口需求和接口设计文件承载同类信息。 |
| `CR` | Change Request | 变更请求 | 用于变更申请及其影响分析和闭环。 |
| `TBD` | To Be Determined | 待确定 | 常用扩展缩写；模板中也常使用 `OPEN / UNKNOWN / MISSING` 表示尚未确定的不同状态。 |
| `RCA` | Root Cause Analysis | 根因分析 | 用于 Bug、现场问题和跨层故障的根因定位。 |
| `FAT` | Factory Acceptance Test | 工厂/出厂验收测试 | 常用扩展缩写；项目需要工厂验收时可纳入 Validation 产物。 |
| `SAT` | Site Acceptance Test | 现场验收测试 | 常用扩展缩写；项目需要现场验收时可纳入真实环境 Validation。 |
| `RTM` | Requirements Traceability Matrix | 需求追溯矩阵 | 常用扩展缩写；当前模板对应 `requirements_traceability.md` 及 Traceability Gate。 |
| `DoD` | Definition of Done | 完成定义 | 未满足 DoD 时不得声称开发完成。 |
| `N/A` | Not Applicable | 不适用 | 表示某字段或检查项经确认不适用，不等于未填写。 |

## 2. 需求、设计、评审与记录编号前缀

| 前缀 | 英文含义 | 中文含义 | 示例 / 说明 |
|---|---|---|---|
| `PRD-xxx` | Product Requirement | 产品需求项 | `PRD-001` |
| `SYS-xxx` | System Requirement | 系统功能需求 | `SYS-NET-001` |
| `NFR-xxx` | Non-Functional Requirement | 非功能需求 | 性能、可靠性、安全性等。 |
| `IF-xxx` | Interface Requirement | 接口需求 | `IF-001` |
| `AC-xxx` | Acceptance Criterion | 验收标准项 | `AC-001` |
| `SCN-xxx` | Scenario | 用户/业务场景 | `SCN-001` |
| `CON-xxx` | Constraint | 产品或系统约束 | `CON-001` |
| `ARCH-xxx` | Architecture Item | 架构设计项 | `ARCH-NET-002` |
| `DES-xxx` | Design Item | 详细设计项 | `DES-STORAGE-004` |
| `ADR-xxx` | Architecture Decision Record | 架构决策记录 | `ADR-003` |
| `TC-xxx` | Test Case | 测试用例 | `TC-NET-017` |
| `REL-xxx` | Reliability Item | 可靠性需求/测试目标 | `REL-001`；项目应在正式文件中说明其具体类型。 |
| `BUG-xxx` | Bug Record | 缺陷记录 | `BUG-017` |
| `RCA-xxx` | Root Cause Analysis Record | 根因分析记录 | `RCA-XXX` |
| `CR-xxx` | Change Request | 变更请求 | `CR-XXX` |
| `CHG-xxx` | Change Record | 变更分类/处理记录 | `CHG-XXX` |
| `DEC-xxx` | Decision Record | 项目决策记录 | `DEC-017` |
| `Q-xxx` | Question | 待澄清问题/质询项 | `Q-017` |
| `DISC-xxx` | Discussion Record | 方案讨论记录 | `DISC-001`；讨论本身不等于正式决策。 |
| `REV-xxx` | Review Finding / Review Record | 评审发现/评审记录 | `REV-001`；具体类型由所在 Review Record 决定。 |
| `ADV-xxx` | Advisory | 非阻断建议 | `ADV-001` |
| `FIELD-xxx` | Field Issue | 现场问题 | `FIELD-XXX` |
| `TASK-xxx` | Task | 任务记录 | `TASK-001` |
| `GOV-MIG-xxx` | Governance Migration | 治理模板迁移/升级记录 | `GOV-MIG-XXX` |
| `xxx / XXX` | Placeholder Sequence | 待项目替换的顺序号占位符 | 正式使用时替换为项目实际编号。 |

## 3. AI 治理角色与会话代码

| 代码 | 英文名称 | 中文职责 | 说明 |
|---|---|---|---|
| `AI` | Artificial Intelligence | 人工智能 | 本模板中的 AI 不自动获得决策或审批权。 |
| `C00` | Project Control | 项目总控/项目控制 | 维护当前状态、授权、阶段协调和事实所有权。 |
| `C01` | Requirements | 产品与系统需求 | 负责 PRD、SRS、需求质询和需求追溯。 |
| `C02` | Architecture and Design | 架构与详细设计 | 负责 ADR、架构、接口和详细设计。 |
| `C03` | Implementation | 编码实现 | 按已批准需求和设计实现代码与单元测试。 |
| `C04` | Independent Review | 独立评审 | 治理角色，不是某个 Model、Harness 或 CLI。 |
| `C05` | Verification and Release | 测试、验证、CI 与发布 | 负责证据、质量门禁和发布验证。 |
| `C06` | Issues and Change | Bug、现场问题与变更闭环 | 负责 BUG / RCA / CR 及变更追踪。 |
| `C03A` | Network Implementation | 网络实现子角色 | 大项目的可选 C03 拆分示例。 |
| `C03B` | Storage Implementation | 存储实现子角色 | 大项目的可选 C03 拆分示例。 |
| `C03C` | User Interface Implementation | 用户界面实现子角色 | 大项目的可选 C03 拆分示例。 |
| `C03D` | Device Implementation | 设备实现子角色 | 大项目的可选 C03 拆分示例。 |
| `CXX` | C-role Placeholder | 待指定的 C00～C06 角色 | 仅用作空白模板占位符。 |
| `R04 / R05` | Review/Gate Instance Identifier | 评审/门禁实例编号 | 模板中的项目级示例代码，不等于 C04/C05 角色；具体含义由项目 `CURRENT_STATE.md` 和正式 Review Record 定义。 |

## 4. 优先级、Finding、测试与风险分级

### 4.1 Question / Work Priority

| 级别 | 英文含义 | 中文解释 |
|---|---|---|
| `P0` | Priority 0 | 不回答就无法继续正确设计的问题。 |
| `P1` | Priority 1 | 不回答可能造成重大返工或架构错误的问题。 |
| `P2` | Priority 2 | 会明显影响当前代码质量、测试或可维护性的问题。 |
| `P3` | Priority 3 | 可以暂缓、后续优化的问题。 |

> P0～P3 用于 `QUESTION_PRIORITY / WORK_PRIORITY`，不是 C04 Finding Severity。

### 4.2 C04 Finding Severity

| 级别 | 英文全称 | 中文解释 |
|---|---|---|
| `S0` | Severity 0 — Critical | 严重安全、合规、数据完整性或不可逆风险；需停止相关推进并升级。 |
| `S1` | Severity 1 — Major | 关键 Current Truth、需求、边界、接口、架构或 Acceptance Threshold 冲突，或可能造成重大返工。 |
| `S2` | Severity 2 — Moderate | 对正确性、可测试性、可维护性、追溯性或受控交付有实质影响。 |
| `S3` | Severity 3 — Minor | 范围有限，但仍违反适用要求、标准或接受条件。 |

> S0～S3 都是阻断当前 `PASS` 的 Finding；Severity 决定风险表达、优先顺序和默认路由。

### 4.3 Testing Tier

| 级别 | 英文含义 | 中文解释 |
|---|---|---|
| `T0` | Test Tier 0 | 必须测试；覆盖核心功能、数据正确性、安全、严重现场故障等。 |
| `T1` | Test Tier 1 | 正常应该测试；覆盖主要路径、主要边界、异常与恢复场景。 |
| `T2` | Test Tier 2 | 只在有明确风险依据时进入正式范围；需说明风险、成本和收益。 |
| `T3` | Test Tier 3 | 默认禁止自动加入正式范围；需项目负责人明确批准。 |

### 4.4 Module Risk Level

| 级别 | 英文含义 | 中文解释 |
|---|---|---|
| `R1` | Risk Level 1 | 低风险；例如 UI 文案、普通显示和低影响设置。 |
| `R2` | Risk Level 2 | 普通风险；例如普通业务逻辑和一般数据处理。 |
| `R3` | Risk Level 3 | 高风险；例如网络、持久化、多线程、设备通信和状态机。 |
| `R4` | Risk Level 4 | 极高风险；例如数据安全、设备控制安全和安全关键功能。 |

## 5. Brownfield Migration 阶段代码

| 代码 | 英文名称 | 中文阶段 |
|---|---|---|
| `M0` | Safety Snapshot | 安全快照 |
| `M1` | Legacy Inventory | 旧项目盘点 |
| `M2` | Governance Bootstrap | 加入工程规则和 AI 上下文控制文件 |
| `M3` | Requirement Baseline Reconstruction | 重建当前需求基线 |
| `M4` | As-Is Architecture Reconstruction | 重建 As-Is 架构 |
| `M5` | Decision and Traceability Setup | 建立 ADR 和需求追溯 |
| `M6` | Test and Quality Baseline | 建立测试与质量基线 |
| `M7` | New Process Baseline Commit | 设置新流程基线 Commit |
| `M8` | Controlled Physical Reorganization | 小批量物理目录整理 |
| `M9` | Long-Term Technical Debt Governance | 长期技术债治理 |

> M0～M9 是旧项目迁移指南的默认阶段代码。具体项目可定义自己的 Milestone，但不得把新定义与旧项目迁移语义混用。

## 6. 软件工程、接口与质量缩写

| 缩写 | 英文全称 | 中文含义 | 备注 |
|---|---|---|---|
| `API` | Application Programming Interface | 应用程序编程接口 | 公共 API 变更通常需要正式影响分析和批准。 |
| `CLI` | Command-Line Interface | 命令行接口 | CLI 只是 Tool，不自动产生治理角色或审批权。 |
| `UI` | User Interface | 用户界面 | 也用于 C03C 示例子角色。 |
| `OS` | Operating System | 操作系统 | 环境和项目概览的常用字段。 |
| `CPU` | Central Processing Unit | 中央处理器 | 硬件架构中的处理器类型。 |
| `MCU` | Microcontroller Unit | 微控制器 | 嵌入式系统中的微控制处理单元。 |
| `TCP` | Transmission Control Protocol | 传输控制协议 | 架构和网络决策示例中使用。 |
| `MQTT` | MQTT Protocol; historically Message Queuing Telemetry Transport | MQTT 消息协议 | 物联网/消息通信协议；现行正式名称通常直接使用 MQTT。 |
| `DNS` | Domain Name System | 域名系统 | 网络故障和测试范围示例中使用。 |
| `CI` | Continuous Integration | 持续集成 | 自动执行 Build、Test、Static Analysis、Sanitizer、Coverage 和 Package。 |
| `CD` | Continuous Delivery / Continuous Deployment | 持续交付/持续部署 | 目录 `10_ci_cd/` 中的交付和部署能力。 |
| `E2E` | End-to-End | 端到端 | 覆盖多个组件或完整用户链路的测试。 |
| `KPI` | Key Performance Indicator | 关键绩效指标 | Coverage 在模板中是诊断信号，不应被机械当作 KPI。 |
| `SLA` | Service Level Agreement | 服务等级协议 | 只有存在明确 SLA 或性能需求时，相关性能测试才自动成为强制项。 |
| `PR` | Pull Request | 合并请求 | 代码审查和远程仓库协作单元。 |
| `ID` | Identifier | 唯一标识符 | 用于需求、任务、决策、Finding、测试和变更的唯一编号。 |
| `GPT` | Generative Pre-trained Transformer | 生成式预训练 Transformer | 模型家族名称；具体 Model 不等于治理 Role。 |
| `HEAD` | Git HEAD | Git 当前检出位置 | 正式 Review Target 不得只记录可移动的 HEAD，必须解析为完整 Commit Hash。 |

## 7. Sanitizer 与测试技术用语

| 缩写/术语 | 英文全称 | 中文含义 | 备注 |
|---|---|---|---|
| `ASan` | AddressSanitizer | 地址消毒器/内存地址错误检测器 | 检测越界、Use-After-Free 等内存问题。 |
| `UBSan` | UndefinedBehaviorSanitizer | 未定义行为检测器 | 检测整数溢出、非法移位等未定义行为。 |
| `TSan` | ThreadSanitizer | 线程消毒器/并发竞态检测器 | 用于检测数据竞争；属于高成本工具，按 Testing Governance 选用。 |
| `Fuzz` | Fuzz Testing | 模糊测试 | 使用大量变异/随机输入寻找崩溃和边界问题；需有明确风险依据。 |
| `Coverage` | Test Coverage | 测试覆盖率 | 用于诊断可能未测试的区域，不直接证明测试质量。 |
| `Mutation Testing` | Mutation Testing | 变异测试 | 通过人工变更代码检验测试是否能发现错误；不得无目的扩张。 |
| `Property-Based Testing` | Property-Based Testing | 基于性质的测试 | 使用不变量和自动生成输入检查行为；需有明确目的。 |

## 8. 核心治理专有名词

| 术语 | 中文解释 | 模板中的稳定语义 |
|---|---|---|
| `Current Truth` | 当前正式事实 | 当前最新、已正式确认、已落盘且未被替代的事实与决定。 |
| `One Fact, One Owner` | 一个事实、一个权威 Owner | 同一动态事实不得在多个文件重复维护。 |
| `Baseline` | 受控基线 | 在特定时点被正式接受的需求、决策、设计、代码和测试产物集合。 |
| `Baseline Anchor` | 基线锚点 | 将基线锁定到精确 Commit 或 Tag 的不可变引用。 |
| `Knowledge Continuation` | 知识接续 | Role、Task、Authority、Baseline 与 Gate 不变且上下文连续性可证明时，对新执行单元执行最小接续核验。 |
| `Baseline Relearn` | 基线重学习 | 连续性无法证明，或 Baseline、Owner、Gate、重大阶段/任务边界变化时，从当前正式基线重建上下文。 |
| `HANDOFF` | 上下文交接快照 | 用于短期连续工作，不是长期 Current Truth Owner。 |
| `Gate` | 门禁 | 进入下一阶段或执行发布等动作前必须满足的受控条件。 |
| `Review Target` | 评审目标 | 被 C04 评审的已冻结对象，必须绑定精确 Commit 或受控版本。 |
| `Review Record` | 评审记录 | 正式记录 Readiness、Finding、Severity、证据、关闭条件和 Gate Decision。 |
| `Review Readiness` | 评审就绪性 | 判断是否具备进入正式 C04 实质评审的前置条件。 |
| `Finding` | 正式评审发现 | 导致 Review Target 无法满足适用接受条件的缺陷、遗漏、矛盾、不可验证性或不可接受风险。 |
| `Advisory` | 非阻断建议 | 不影响当前接受，可与 `PASS` 并存。 |
| `Observation` | 非阻断观察 | 记录审查中发现的信息，但不构成当前 Finding。 |
| `Future Improvement` | 后续改进项 | 不要求在当前 Gate 前关闭的改进建议。 |
| `Exception` | 例外批准 | 由现有权限 Owner 对特定偏差做出范围受控的正式批准；C04 不批准 Exception。 |
| `Risk Acceptance` | 风险接受 | 由有权 Owner 明确批准接受某项风险，并记录范围、理由和边界。 |
| `Traceability` | 追溯性 | 建立产品需求、系统需求、设计、实现、测试与证据之间的受控关系。 |
| `Node Coverage` | 节点覆盖 | 检查所有必需 SYS / NFR / IF 节点是否出现在正式追溯集中。 |
| `Edge Consistency` | 关系边一致性 | 检查详细需求中的上游关系与追溯矩阵中的关系是否完全一致。 |
| `Formal Trace` | 正式追溯关系 | 可用于 Gate 和机械验证的受控追溯边。 |
| `Iterative V-Model` | 迭代式 V 模型 | 将需求、架构、设计、实现与对应验证活动配对，并通过迭代持续纠错。 |
| `As-Is` | 当前实际状态 | 已有项目此刻真实存在的结构、行为、接口、数据和部署状态。 |
| `To-Be` | 目标状态 | 经批准后希望达到的未来结构或行为；不得与 As-Is 混写。 |
| `Greenfield` | 全新项目 | 从空白或几乎空白状态开始的项目。 |
| `Brownfield` | 已有/旧项目 | 已有代码、文档、用户或运行历史，需受控迁移的项目。 |
| `Full Template` | 完整模板 | 适用于长周期、高风险、多阶段或需要正式追溯的项目。 |
| `Lite` | 轻量模板 | 保留核心治理原则的精简文件集。 |

## 9. Role、Model、Runtime、Harness、Session 与 Tool

| 术语 | 中文含义 | 治理语义 |
|---|---|---|
| `Role` | 角色 | 决定职责、权限和 Gate 身份。 |
| `Model` | 模型 | 提供推理和生成能力，不自动产生审批权。 |
| `Runtime` | Agent 运行时 | 执行 Agent Loop、权限拦截、状态和调用协议；不自动成为 Harness 或治理 Role。 |
| `Harness` | Agent 执行环境 | 承载任务编排、上下文、工具接入和用户交互。 |
| `Session` | 执行会话 | 绑定一次具体上下文、Role Assignment、任务和权限的物理执行实例。 |
| `Tool / CLI / API` | 工具/命令行/编程接口 | 只是被调用的机制，不因技术上可调用就获得治理角色。 |
| `Dynamic Role Profile` | 动态岗位责任说明书 | 将固定 Role Brief 投影到当前项目、任务、上下游、工具、权限、知识和执行环境；只能收窄授权。 |
| `Knowledge Manifest` | 知识清单 | 记录当前岗位和任务最小必读来源、已加载证据、按需检索范围和未解决 Rule Gap。 |
| `Rule Gap` | 规则缺口 | 必要检索后仍无法获得唯一规则时的失败关闭报告，分为 `RULE_NOT_FOUND / RULE_CONFLICT / VERSION_AMBIGUOUS`。 |
| `Interaction Contract` | 岗位交互合同 | 定义受控岗位交互的 Sender、Receiver、Action、Scope、Authority、输入、输出、副作用和终态。 |
| `Interaction Operation` | 岗位交互实例 | 按某个版本化 Interaction Contract 发起并审计的一次具体交互。 |
| `Authorization Contract` | 授权合同 | 精确绑定 Authority Owner、Action、Scope、Target、副作用、有效期、消费事件、终态、重试和对账。 |
| `Primary Executor` | 主执行者 | 承担 C00/C01/C02/C03/C05/C06 的默认连续执行。 |
| `Expert Escalation` | 专家升级 | 处理复杂、高风险或多次失败的精确技术问题。 |
| `Independent Reviewer` | 独立评审者 | 使用与实现/整改隔离的全新 C04 Session 执行正式评审。 |
| `Human Project Owner` | 人类项目负责人 | 承担产品目标、重大边界、验收阈值、重大风险和未授权发布等保留决策。 |
| `AUXILIARY` | 辅助输出 | 执行 Session 中调用其他 Model/Tool 所得的辅助分析，不是正式 C04 Gate 结论。 |

```text
Role != Model != Runtime != Harness != Session != Tool
SUBAGENT_PERMISSION <= CALLER_PERMISSION
AUXILIARY / ADVISORY != FORMAL C04
```

## 9.1 审核、裁决与执行保障

| 术语 | 中文含义 | 治理语义 |
|---|---|---|
| `INDEPENDENT_REVIEW` | 独立复审线 | 使用隔离的新 Session；分为正式 C04 与不产生 Gate 权威的非正式独立复审。 |
| `FORMAL_C04` | 正式 C04 | 针对冻结精确 Target、具备独立性证据并形成正式 Review Record 与 Gate Decision。 |
| `INFORMAL_INDEPENDENT` | 非正式独立复审 | 保持上下文隔离，但输出仅为 Advisory / Observation。 |
| `CONTEXTUAL_REVIEW` | 带上下文复审 | 在兼容授权和任务绑定下继续/恢复上下文，不产生正式 C04 权威。 |
| `SELF_REVIEW` | 自审 | 执行者对自身产物做 Diff、检查、构建或测试，不能替代独立 C04。 |
| `HUMAN_DETERMINATION` | 人类审定 | 由 Human Project Owner 对保留事项作出明确裁定。 |
| `Human Determination Package` | 人类审定包 | 面向负责人提供事实、选项、风险、后果、推荐、未授权事项和可复制回复格式的唯一待决包。 |
| `Formal Seal` | 正式封印 | Human Project Owner 对精确不可变 Package 作出的正式接受声明；不由其他 Action 自动推导。 |
| `PROCEDURAL_FALLBACK` | 程序性后备模式 | 无专用工具时用受控文件、Git、检查表、明确回复和证据执行同一治理规则。 |
| `TOOL_ENFORCED` | 工具强制模式 | 工具消费正式治理合同并机械检查权限、交互、状态、Gate 和审计。 |
| `Enforcement Evidence` | 执行保障证据 | 记录当前模式、机械能力、控制证据、降级项、剩余风险和对账要求。 |

## 10. Autonomy Mode 与授权字段

| 术语 | 中文含义 | 说明 |
|---|---|---|
| `MANUAL_GATE` | 人工门禁模式 | 进入下一 Gate、Milestone 或阶段前需明确人工批准。 |
| `SUPERVISED_AUTO` | 受监督自动模式 | 在已批准阶段和文件范围内连续自动执行，不自动跨越未批准 Gate。 |
| `FULL_AUTO` | 全自动模式 | 可在既定 Current Truth 和授权内跨越预授权 Gate，直到 `AUTHORIZED_UNTIL`。 |
| `AUTHORIZED_UNTIL` | 授权终点 | 当前自动执行最远允许到达的 Gate 或 Milestone。 |
| `PREAUTHORIZED_GATES` | 预授权门禁 | `FULL_AUTO` 允许自动跨越的 Gate 清单。 |
| `AUTO_ALLOWED` | 自动允许操作 | 只在当前 Autonomy Mode、文件权限和实际系统权限内有效。 |
| `READ_ONLY` | 只读 | 不得修改文件或外部系统。 |
| `NO_COMMIT` | 禁止提交 | 本任务不得执行 Git Commit。 |
| `NO_PUSH` | 禁止推送 | 本任务不得将变更推送到远程仓库。 |

## 11. 正式评审结论与关闭状态

| 状态/结论 | 中文含义 | 使用边界 |
|---|---|---|
| `READY` | 已就绪 | 在 Review Readiness 中表示具备正式评审前置条件。 |
| `REVIEW_NOT_READY` | 评审未就绪 | 前置状态，不是 Gate Decision；此时不输出正式 Finding、`PASS` 或 `CHANGES_REQUESTED`。 |
| `PASS` | 通过 | Readiness 为 READY、强制检查完成、证据完整且 Open Findings = 0。 |
| `CHANGES_REQUESTED` | 要求修改 | Readiness 为 READY，但存在必须关闭的 Open Finding。 |
| `OPEN` | 未关闭 | 问题、Finding、任务或质询项尚未闭环。 |
| `CLOSED_BY_FIX` | 通过整改关闭 | Finding 已修复，并由新独立 C04 Session 核验关闭。 |
| `CLOSED_BY_APPROVED_EXCEPTION` | 通过已批准例外关闭 | 正确 Owner 批准 Exception/Risk Acceptance 后，由新独立 C04 Session 核验关闭。 |
| `TRACEABILITY_CLOSED` | 追溯已闭合 | Node Coverage 与 Edge Consistency 通过，或所有差异已有正式批准和逐条解释。 |
| `TRACEABILITY_NOT_CLOSED` | 追溯未闭合 | 追溯机械检查存在未关闭差异。 |
| `FAIL` | 机械检查/测试失败 | 可用于测试和子检查结果；不是当前正式 C04 Gate Decision。 |
| `PASS_WITH_ACTIONS` | 带后续行动的通过 | 当前正式 C04 禁止使用；必须分类为 Finding 或非阻断 Advisory。 |
| `CONDITIONAL_PASS` | 有条件通过 | 当前正式 C04 禁止使用。 |
| `APPROVED_WITH_COMMENTS` | 带意见批准 | 当前正式 C04 禁止使用。 |

## 12. 决策、产物与任务的受控状态

| 状态 | 中文含义 | 是否可作为当前正式依据 |
|---|---|---|
| `DRAFT` | 草案 | 否 |
| `PROPOSED` | 已提出、尚未确认 | 否 |
| `DISCUSSED` | 已讨论、尚未正式决定 | 否 |
| `APPROVED` | 已正式批准 | 是 |
| `ACCEPTED` | 已正式接受，常用于 ADR | 是 |
| `CONFIRMED` | 已由有权负责人明确确认 | 是 |
| `CURRENT` | 当前正式有效 | 是 |
| `SUPERSEDED` | 已被新决定替代 | 否 |
| `REJECTED` | 已明确否决 | 否 |
| `ARCHIVED` | 已归档，仅保留历史 | 否 |
| `REFERENCE` | 仅供参考 | 否 |
| `UNKNOWN` | 无法确认 | 否 |
| `INFERRED` | 从现有资料推断，尚未确认 | 否 |
| `MISSING` | 所需文件、证据或输入缺失 | 否 |
| `DEFERRED` | 已明确延期 | 否；必须由相应 Owner 记录延期边界 |

## 13. 快速区分

```text
PRD / SRS / ADR
→ 文档与决策产物

C00～C06
→ 治理角色

P0～P3
→ Question / Work Priority

S0～S3
→ C04 Finding Severity

T0～T3
→ Testing Tier

R1～R4
→ Module Risk Level

M0～M9
→ Brownfield Migration Stage

PASS / CHANGES_REQUESTED
→ 正式 C04 Gate Decision

REVIEW_NOT_READY
→ 评审前置状态，不是 Gate Decision
```

## 14. v0.1.4+ 运行与升级术语

| 术语 | 中文解释 | 使用边界 |
|---|---|---|
| `Owner Decision Package` | 负责人决策包 | `Human Determination Package` 的兼容别名；新记录使用后者，原权威文件仍是事实来源。 |
| `Gate Package` | 门禁包 | 按同一决策边界组织相关产物；不得借合包跳过强制证据、追溯或独立评审。 |
| `Work Package` | 工作包 | 可递归拆分的有界执行单元，包含目标、边界、输入、输出、风险和完成条件；不自动新增人工 Gate。 |
| `Assurance Cadence` | 保障/评审频率 | 决定保障活动何时触发及采用何种 Profile；不得关闭非可选控制。 |
| `Persistent C00 Control Channel` | 持续逻辑 C00 控制通道 | 项目负责人持续使用的逻辑管理通道；底层物理 Session 可受控轮换。 |
| `Physical Session` | 物理会话 | 具体 AI 环境中的一次上下文实例，达到阈值或隔离触发时可交接到新实例。 |
| `Independent Session` | 独立会话 | 为满足角色、上下文或评审独立性而新建的隔离 Session；不自动扩大权限。 |
| `In-Session External-AI Tool Call` | 当前会话内外部 AI 工具调用 | 返回 Auxiliary/Advisory 结果，可静默执行但必须服从配置与权限；不等于新独立 Session 或正式 C04。 |
| `NEW_INDEPENDENT_SESSION_REQUEST` | 新独立会话请求包 | 机器可读的独立 Session 创建请求，包含触发原因、目标 Role、精确目标、最小输入、权限和返回路由。 |
| `GOVERNANCE_MIGRATION_COMMITTED` | 治理迁移已提交 | 升级迁移 Commit 已形成，但尚未证明已完成要求的独立评审、Baseline 采用和 Relearn。 |
| `READY_FOR_BASELINE_ADOPTION` | 可进入 Baseline 采用 | 精确 Candidate 已通过所需 C04，等待按既有 Baseline Owner 规则完成 `CANDIDATE -> CURRENT`。 |
| `GOVERNANCE_UPGRADE_COMPLETE` | 治理升级完成 | Candidate 已正式采用为 Current，所需 Baseline Relearn 和采用后检查已完成。 |
| `POST_C04_RECORD_ONLY_DESCENDANT` | C04 后仅记录后代 Commit | 只增加受控评审/迁移/Baseline 状态记录、且机械证明治理产物和产品事实零漂移的精确后代；不是通用免评审规则。 |
| `vX.Y.Z-candidate` | 内部候选身份 | 表示尚未正式采用或发布的审核候选，可在授权后绑定精确 Candidate Commit，但不是发布 Tag 或已发布 Prerelease。 |
