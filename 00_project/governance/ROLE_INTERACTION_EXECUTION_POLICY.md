# 岗位交互与可执行治理政策

> 本文件是固定岗位、动态岗位责任说明书、岗位知识加载、标准交互、通用授权生命周期、四条审核/裁决运行线、Formal Seal 和执行保障模式的唯一权威来源。
>
> 本文件不重新定义 C04 Finding/Decision、保障触发频率、Session 创建格式、项目 Current Truth、Baseline、任务实例或运行配置当前值。

---

## 0. 事实所有权与边界

```text
固定岗位、动态岗位 Profile、知识加载、Interaction、通用授权、四条运行线、Formal Seal、执行保障模式
→ 本文件

机器可读字段、枚举和必填项
→ GOVERNANCE_EXECUTION_CONTRACTS.yaml

正式 C04 Finding / Severity / Decision Matrix
→ AI_ENGINEERING_RULES_V2.md 第 38.7 节

保障频率、正式 C04 和独立 Session 触发
→ PROJECT_ASSURANCE_CADENCE_POLICY.md

Session 创建、隔离、连续、交接和返回
→ AI_CONVERSATION_ORCHESTRATION_RULES.md

当前授权、路由、Profile 和执行保障模式
→ CURRENT_STATE.md

当前任务实例状态
→ ACTIVE_TASKS.md

当前 Baseline 身份与组成
→ BASELINE_INDEX.md
```

软件工具可以机械执行本政策，但不得成为本政策成立的前提，也不得反向创造新的治理 Owner。

---

## 1. 固定岗位与动态执行身份

C00～C06 保持框架固定标准岗位：

```text
C00  项目控制
C01  产品与系统需求
C02  架构与详细设计
C03  编码实现
C04  独立评审
C05  验证、CI 与发布
C06  问题与变更闭环
```

项目可以配置岗位的当前任务、输入、输出、工具、交互、授权、执行环境和保障节奏，但不得用配置改写 Role Brief 中的稳定职责、事实 Owner 或 Gate 身份。项目特有专家或子角色可以作为受控执行分工存在，但不能替代 C00～C06，也不能创建新的 Current Truth 来源。

稳定分离规则：

```text
Role != Model != Runtime != Harness != Session != Tool
```

- `Role`：决定职责、权限上限、事实 Owner 关系和 Gate 身份；
- `Model`：提供推理与生成能力；
- `Runtime`：执行 Agent Loop、权限拦截、状态与调用协议的运行时；
- `Harness`：承载任务编排、上下文、工具接入和用户交互的 Agent 环境；
- `Session`：绑定一次具体上下文、Role Assignment、任务和权限的执行实例；
- `Tool / CLI / API`：被调用的具体能力或动作入口。

一个维度的变化不能自动改变其他维度。技术上能够调用某个 Runtime、Harness、Model 或 Tool，不会自动取得它在其他项目中通常承担的 Role、Gate 或批准权。

---

## 2. Dynamic Role Profile

每个物理 Session 在承担受控工作前，必须拥有一个与当前项目、岗位、任务、Gate 和授权相匹配的 `DYNAMIC_ROLE_PROFILE`。工具可自动生成；无专用工具时由 C00 / Primary Executor 按合同声明并检查。

Profile 至少包含：

- Role ID 和稳定 Role Brief 引用；
- 当前 Task / Work Package；
- 当前或适用 Gate 绑定及其 Authority Source；
- 上游与下游岗位；
- 输入、输出和适用事实 Owner 绑定；
- 允许发起的流程；
- 允许使用的 Tool / Action；
- 禁止动作和禁止副作用；
- Authority Source、授权终点和有效期；
- 必须上报 Human Project Owner 的事项；
- Knowledge Manifest；
- 当前 Interaction Contract；
- Model / Runtime / Harness / Session 绑定；
- Enforcement Mode；
- 生成依据、版本和失效条件。

机械规则：

1. Role Assignment 或 Profile 缺失、不完整、过期或与 Current Truth 冲突时，必须标记 `ROLE_PROFILE_NOT_READY`，不得执行受控副作用动作；
2. Profile 是当前授权和权威文件的受控投影，不是新的 Current Truth Owner；
3. Task、Gate、授权、Role、执行环境或关键输入变化时必须重新验证，必要时重新生成；
4. Profile 只能缩小已获授权，不能扩大授权；
5. 静态 Role Brief 负责稳定职责，动态 Profile 负责本次执行边界，二者不得互相覆盖。
6. 当前或适用 Gate 绑定、适用事实 Owner 绑定必须分别对照 Current Truth、稳定 Role Brief、当前 Task / Work Package 和 Authorization 校验；任一绑定缺失、过期或冲突时不得声明 `ROLE_PROFILE_READY`。

---

## 3. 岗位知识加载与规则缺口

岗位接任默认采用：

```text
MINIMUM_NECESSARY_INITIAL_LOAD
→ ON_DEMAND_GOVERNANCE_SEARCH
→ RULE_GAP_REPORT_IF_UNRESOLVED
```

初始知识包只加载完成当前岗位、任务和 Interaction 所必需的治理内核、Current Truth、Role Brief、任务输入和适用 Gate。不得要求每个 AI 默认把整个治理仓库全部装入上下文。

AI 可以并且在需要时必须搜索整个受权治理仓库。知识范围不是权限边界；读取更多规则不会扩大 Role、Tool、Action、Gate 或副作用权限。

禁止猜测规则。完成必要检索后仍不能得到唯一适用答案时，必须发起 `RULE_GAP_REPORT`：

```text
RULE_NOT_FOUND
RULE_CONFLICT
VERSION_AMBIGUOUS
```

报告至少记录问题、检索范围、发现证据、受影响动作、当前阻断、建议选项和所需裁决者。依赖该规则的动作在正式补充或裁定前保持失败关闭。

---

## 4. 标准岗位交互

C00～C06 之间会影响任务、权限、Gate、受控产物或正式状态的协作，必须通过版本化 `INTERACTION_CONTRACT` 和实例化的 `INTERACTION_OPERATION` 执行。

自然语言可以作为说明或证据附件，但不能替代以下控制字段：

- Interaction ID、Contract Version；
- Sender Role、Receiver Role；
- Action Type、Intent 和 Scope；
- Authority Source；
- Input Evidence；
- Allowed Tools / Actions；
- Forbidden Actions / Side Effects；
- Expected Output；
- Receipt；
- Status 和 Terminal State；
- Error / Escalation；
- Audit Reference。

交互最小生命周期：

```text
PREPARED
→ AUTHORIZED
→ DISPATCHED
→ ACKNOWLEDGED
→ IN_PROGRESS
→ OUTPUT_READY
→ COMPLETED
```

允许的受控分支：

```text
BLOCKED
CANCELLED
FAILED
RESULT_UNKNOWN_RECONCILIATION_REQUIRED
```

未知交互类型、必填字段缺失、岗位不匹配、授权不适用、动作越权、合同版本不兼容或终态不可证明时，必须拒绝执行或进入对账，不得用自由文本推断成功。

普通对话和解释性交流不必为每句话创建 Interaction Operation；只有会改变受控执行、状态、权限、Gate 或正式产出的动作必须使用合同。

---

## 5. 通用授权合同与生命周期

任何会产生副作用或正式治理结果的动作，必须绑定一个精确授权合同。至少包含：

```text
AUTHORITY_OWNER
ACTION
SCOPE
TARGET
ALLOWED_SIDE_EFFECTS
FORBIDDEN_SIDE_EFFECTS
VALIDITY
CONSUMPTION_EVENT
TERMINAL_STATE
RETRY_POLICY
ESCALATION
```

授权状态：

```text
PROPOSED
ISSUED_UNUSED
CONSUMED
CANCELLED
EXPIRED
EXECUTION_COMPLETED
RESULT_UNKNOWN_RECONCILIATION_REQUIRED
```

规则：

1. “批准”只能应用于当前唯一、边界完整的待决 Package；存在多个候选、范围不完整或 Target 不明确时必须请求澄清；
2. 授权在 `CONSUMPTION_EVENT` 发生时原子消费，不以聊天回复、能力开关或技术可用性代替；
3. 文件修改、Commit、Tag、Baseline Adoption、独立 Session 创建、正式 C04 Dispatch、安装、真实 Model 调用、Push、PR、Release、远程修改和 Formal Seal 是相互独立的 Action Class，任何一个都不自动授权另一个；
4. 子 Agent、Model、Runtime、Harness、Session 和 Tool 不得扩大调用者权限；
5. 授权已消费后，失败、超时或结果未知不得自动重试，除非 Retry Policy 明确允许；
6. 外部副作用结果无法确认时进入 `RESULT_UNKNOWN_RECONCILIATION_REQUIRED`，先对账，不得重复执行来“试试看”；
7. 执行完成不等于结果被接受、Baseline 被采用或 Gate 通过。

### 5.1 Baseline Adoption 的有限预授权

正式 Baseline 默认由 Human Project Owner 或项目既有 Baseline Owner 明确采用。C00 只有在以下条件全部满足时，才可依据预授权执行 `CANDIDATE -> CURRENT`：

- 预授权由正确 Owner 签发；
- 精确绑定 Baseline Type、Candidate ID、Target、适用范围和有效期；
- 明确授权的 Action 是 `BASELINE_ADOPTION`；
- 必要 C04、验证和采用前 Gate 已完成；
- Open Finding 为 0；
- 不涉及产品目标、重大架构裁定、Acceptance Threshold、重大风险接受、Formal Seal 或 Release；
- 允许与禁止副作用、消费事件、终态和对账规则完整；
- 当前事实与签发授权时一致。

缺少任一条件、只出现模糊“批准”或当前事实已变化时，C00 不得采用 Baseline，必须请求 Human Determination。

---

## 6. 四条审核与裁决运行线

所有检查或裁决意图必须在执行前唯一分类为以下一条；普通实现、查询和机械操作不因名称相似自动进入评审线。

### 6.1 INDEPENDENT_REVIEW

必须使用与执行/整改上下文隔离的新 Session。

分为：

- `FORMAL_C04`：绑定冻结且精确的 Target、针对被审对象只读、形成正式 Review Record，可产生受控 Gate Decision；
- `INFORMAL_INDEPENDENT`：保持上下文隔离，但结果只能是 Advisory / Observation，不具有 C04 或 Gate 权威。

同一个 Model、Runtime 或 Harness 可以在不同的真正独立 Session 中承担 Reviewer；使用不同 Model 本身也不能证明独立。

### 6.2 CONTEXTUAL_REVIEW

在政策允许、Role/授权/任务绑定兼容时，可以 Continue / Resume 当前或受控历史 Session。必须记录继承的上下文、来源和限制。结果权威不得超过调用者与本次任务授权，不能冒充正式 C04。

### 6.3 SELF_REVIEW

由执行者对自身产物执行 Diff、检查表、构建、测试或一致性自检。只能证明自检完成，不能关闭必须由独立 Reviewer 关闭的 Finding，也不能产生正式 C04 `PASS`。

### 6.4 HUMAN_DETERMINATION

以下事项由 Human Project Owner 保留：

- 产品目标变化；
- 已批准需求或 Acceptance Threshold 变化；
- 新系统边界、公共接口、跨系统依赖、安全/数据完整性设计或重大不可逆架构取舍；
- 重大风险接受；
- Formal Seal；
- 未被精确预授权覆盖的正式 Baseline Adoption；
- Release；
- 模糊、冲突或可能扩大的授权；
- 其他正式规则明确保留给人的事项。

模型可以分析和推荐，但不得代替人类决定。Human Determination 必须使用 `AI_ENGINEERING_RULES_V2.md` 第 41.1 节定义的 Package 和明确回复格式。

---

## 7. Formal Seal

`FORMAL_SEAL` 是 Human Project Owner 对一个精确、不可变 Package 作出的正式封印，表示该 Package 已完成规定的审核并被接受用于声明目的。

Formal Seal：

- 必须绑定精确 Target、Purpose、Evidence、适用范围和日期；
- 只能由 Human Project Owner 明确签发；
- 不得由 C00、C04、Model、Runtime、Harness、Session 或 Tool 代签；
- 不由 Commit、C04 PASS、Baseline Adoption 或 Release 自动推导；
- 不自动授权 Push、Release、安装或其他远程副作用；
- 发现问题时通过新版本、新 Target 和新 Seal 处理，不改写旧 Seal。

---

## 8. Model / Runtime / Harness 切换与逻辑 C00 连续性

逻辑 C00 控制通道在 Model、Provider、Runtime 或 Harness 切换时保持连续；这不表示物理上下文无需校验。

切换时必须记录变更前后 Provider / Model / Runtime / Harness，并选择：

### KNOWLEDGE_CONTINUATION_CHECK

只有同时满足以下条件才可使用：

- Role、Task、Scope、Authority、Baseline 和 Gate 未变化；
- 当前正式文件和精确 Git Anchor 可读取；
- 平台能够证明当前上下文连续或已提供受控最小接续包；
- 没有独立性要求；
- 新执行单元能准确复述当前 Role、任务、权限和下一步。

### BASELINE_RELEARN

出现以下任一情况必须执行：

- Baseline、Owner、Gate、重大阶段或任务边界发生实质变化；
- 上下文连续性无法证明；
- 已出现事实遗忘、版本混淆或权限理解差异；
- Harness / Runtime 变化导致新的物理上下文边界；
- 高风险任务的当前政策要求；
- Human Project Owner 明确要求。

正式 C04 始终使用新的独立 Session，不得用 Knowledge Continuation 替代独立性。

---

## 9. 执行保障模式

框架支持：

```text
PROCEDURAL_FALLBACK
TOOL_ENFORCED
```

两种模式必须使用同一套 Role、Gate、Authorization、Interaction、Review Line、State、Finding、Commit、Baseline、Formal Seal 和 Release 语义。

### 9.1 PROCEDURAL_FALLBACK

无专用工具、工具未完成或不可用时，使用受控文件、Git、合同模板、检查表、明确回复、独立 Session 和证据完成闭环。

必须声明：

```text
ENFORCEMENT_MODE: PROCEDURAL_FALLBACK
MECHANICAL_ENFORCEMENT: NOT_AVAILABLE / PARTIAL
CONTROL_EVIDENCE: DECLARED / CHECKED / HUMAN_CONFIRMED
RESIDUAL_RISK: PROCEDURAL_ERROR
```

人工声明不得冒充软件机械拦截证据。

### 9.2 TOOL_ENFORCED

工具可以机械校验 Role Assignment、Knowledge Manifest、Interaction、Tool/Action Authority、Gate、Authorization Consumption、Review Line、Session Binding、状态转换、审计和失败恢复。

工具必须消费本政策和机器合同，不得自行重定义 Role、Owner、Gate、状态或授权语义。工具缺失某项能力时必须显式报告，不得模拟成功。

### 9.3 降级与恢复

从 `TOOL_ENFORCED` 降级到 `PROCEDURAL_FALLBACK` 时必须记录：

- 降级原因和 Owner；
- 失去的机械控制；
- 替代检查；
- Remaining Risk；
- 是否需要额外 Human Confirmation；
- 工具恢复后的对账和恢复条件。

以下事项不能通过程序性声明降级成立：

- 没有 Git 时宣称形成精确 Commit；
- 没有不可变 Target 时宣称完成正式 Target Review；
- 没有独立 Session 时宣称完成正式独立 C04；
- 没有人类明确决定时宣称完成 Human Determination 或 Formal Seal；
- 没有实际证据时把任务标记完成；
- 权限不明确时执行副作用动作。

---

## 10. 标准运行闭环

```text
岗位接任
→ 最小知识加载
→ Current Truth 检查
→ Dynamic Role Profile 就绪
→ 接收标准 Interaction / Task
→ Authorization / Gate 检查
→ 执行
→ Self Review 与验证
→ 标准完成报告
→ 适用的 Independent / Contextual Review 或 Human Determination
→ Commit / Baseline / 状态收口
```

任何一步失败或无法证明时进入显式 Blocked / Unknown 状态，不得跳步制造成功结论。
