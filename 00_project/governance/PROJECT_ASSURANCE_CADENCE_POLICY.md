# 项目保障与评审节奏策略

> 本文件是项目保障节奏、强制触发事件、不可关闭控制项和独立 Session 触发条件的唯一权威来源。
>
> 本文件决定“何时必须执行某类保障活动”，不重新定义 C00～C06 的职责，不成为 Current Truth、Baseline、任务、Review Record 或运行路由的新 Owner。

---

## 1. 核心原则

```text
Role responsibility
!=
Assurance cadence
```

- 角色职责保持稳定；
- 项目可以配置各固定角色、阶段、质询、Expert、C04、测试和人工 Gate 的调用节奏；
- 节奏配置不能通过改名、合并称谓或更换 Model/Runtime/Harness/Session 绕过强制触发条件；
- “频率降低”不等于“适用要求失效”；
- Gate 可以预授权或自动执行，但不可关闭控制项不能被配置成不存在。

当前采用的 Profile 只在 `CURRENT_STATE.md` 记录；Profile 定义和不可关闭边界只在本文件维护。

固定岗位、动态 Profile、Interaction、通用授权、四条审核/裁决运行线和执行保障模式由 `ROLE_INTERACTION_EXECUTION_POLICY.md` 定义。本文件只决定何时必须触发保障活动。

---

## 2. 推荐 Profile

```text
LEAN
STANDARD
HIGH_ASSURANCE
CUSTOM
```

### LEAN

- 适用于低风险、小范围、短周期项目；
- 合并普通阶段检查，优先事件触发；
- 不降低正式 C04 独立性、Open Finding 关闭、Traceability、T0、Release 和权限边界。

### STANDARD

- 模板默认；
- 里程碑与事件共同触发保障活动；
- 普通实现增量由 Primary 连续执行，重大变化、Baseline、Finding 关闭和 Release 进入正式保障链。

### HIGH_ASSURANCE

- 适用于安全、合规、数据完整性或高风险控制项目；
- 可增加中间 C04、验证、证据和人工 Gate；
- 增加项必须有风险来源，不得为了形式无限增加文档或测试。

### CUSTOM

- 项目可以按风险配置；
- 每个 Override 必须记录适用范围、理由、Owner 和失效条件；
- 不得覆盖第 4 节不可关闭控制项。

---

## 3. 可配置节奏

以下项目允许配置为 `PER_TASK / PER_WORK_PACKAGE / PER_MILESTONE / ON_EVENT / MANUAL / NOT_APPLICABLE`：

- C00 状态汇总频率；
- C01 需求质询批次；
- C02 架构/设计复核批次；
- C03 实现工作包大小；
- C04 普通中间评审频率；
- C05 普通验证批次；
- C06 问题/变更汇总频率；
- PRD、SRS、架构、设计和测试设计的审批包粒度；
- Expert Escalation 的非强制咨询频率；
- 普通人工状态汇报频率。

配置 `NOT_APPLICABLE` 时必须说明该活动为何对当前范围不适用。不得把应执行的 C04 改名为“专家复核”，或把应执行的正式验证改名为“快速检查”来绕过本文件。

检查或裁决活动必须在执行前明确为 `INDEPENDENT_REVIEW / CONTEXTUAL_REVIEW / SELF_REVIEW / HUMAN_DETERMINATION`。选择 Contextual Review 或 Self Review 不会关闭本文件规定的正式 C04 触发事件；普通任务也不会因为完成 Self Review 就自动触发正式 C04。

### 3.1 默认增量复审范围

在当前 `LEAN` Profile 下，普通评审和再复审默认采用：

```text
DEFAULT_REVIEW_SCOPE: DELTA_ONLY
```

`DELTA_ONLY` 的规范范围只包括：

1. 自上一份有效评审证据以来实际发生 Byte 或规范语义变化的对象；
2. 这些变化直接影响的接口、约束、不变量、追溯项、Finding 和验收证据闭包；
3. 证明 Target 未被越权改动、继承结论仍绑定正确对象所需的最小完整性证据。

已经关闭、内容未变化且其成立前提未被本轮改动影响的事项，必须标记为 `INHERITED_CLOSED`，不得重复阅读、重复测试或重新论证。文件被纳入完整 Target Digest 只表示其 Byte 受到完整性保护，不自动把该文件全部内容纳入语义复审范围。

Reviewer 必须在开始时列明 Delta、直接影响闭包和继承项。若审查中发现本轮变化直接破坏某个继承项的成立前提，可以重新打开该项或记录新 Finding；不得借此扩张为与本轮改动无关的全项目扫描。

只有满足以下任一条件时，才升级为 `FULL_SCOPE`：

- 项目负责人明确要求全量复审；
- 第 5 节要求的首次正式 C04、正式 Baseline Adoption 或 Public / Production Release Candidate 评审需要完整范围；
- 变化跨越公共接口、架构、安全、合规、数据完整性或治理语义，且直接影响闭包无法可靠界定；
- 先前证据、精确 Target 或继承前提缺失、失效或不可复现；
- 适用的正式 C04 程序明确要求全量范围。

正式 C04 的独立性、精确不可变 Target、Open Finding 关闭和其他不可关闭控制项不因 `DELTA_ONLY` 降低。正式 Finding 关闭复审只有在适用 C04 程序允许、精确新 Target 可复现且影响闭包可证明时，才可以采用增量范围。

---

## 4. 不可关闭控制项

以下控制在所有 Profile 下均不可配置为 `OFF / DISABLED / NOT_APPLICABLE`：

1. Current Truth 冲突停止，以及保留决策由正确 Owner 批准；
2. `SUBAGENT_PERMISSION <= CALLER_PERMISSION`，以及远程、破坏性、Release 等重大副作用的明确授权；
3. 正式 C04 被发起时的 Review Readiness、精确不可变 Target、独立上下文和正式 Review Record；
4. 任一 Open S0～S3 Finding 阻断 `PASS`，关闭后由新独立 C04 针对新精确 Target 验证；
5. 建立或实质修改正式需求 Baseline / 追溯关系时的 Traceability Gate；
6. 适用的 T0、Acceptance Criteria / Threshold、安全、合规、数据完整性验证，且不得为了通过而降低阈值；
7. Public / Production Release 的授权和证据 Gate；
8. 治理升级的 Readiness、精确源/目标、回滚 Anchor、产品事实保护和明确 Baseline Adoption 结果；
9. 正式 Baseline、Review、Release、治理迁移的不可变审计 Anchor，禁止改写历史制造成功状态。
10. 受控副作用动作的精确 Authorization Contract、原子消费、未知结果对账和 Action Class 不互推；
11. Dynamic Role Profile、Interaction Contract 和 `SUBAGENT_PERMISSION <= CALLER_PERMISSION` 共同限制执行范围；
12. `PROCEDURAL_FALLBACK` 不得冒充 `TOOL_ENFORCED`，降级时必须记录失去的机械控制和 Remaining Risk。

“不可关闭”不等于“每次都要人工执行”。在 Current Truth、Autonomy Mode 和预授权范围内，可以自动检查、自动整改、自动复审或自动推进；命中负责人保留决策时才转人工。

---

## 5. 正式 C04 的不可关闭触发事件

以下事件必须形成正式 C04：

1. 接受或替换包含实质性需求、系统边界、公共接口、架构、安全、合规或数据完整性变化的正式 Baseline；
2. 验证任一正式 C04 Finding 的关闭；
3. 接受 Public / Production Release Candidate；
4. 采用会改变启动、Owner、权限、评审、测试、追溯或 Baseline 语义的治理框架版本。

以下事项不自动触发正式 C04：

- 编辑性文档修改；
- 不阻断接受的 Advisory / Observation；
- 普通实现增量；
- 不改变公共行为和 Acceptance Threshold 的局部重构；
- 纯术语、展示、Release Metadata 修改；
- 经影响分析证明不受上游变化影响的内容。

`INFORMAL_INDEPENDENT`、`CONTEXTUAL_REVIEW` 和 `SELF_REVIEW` 可以按风险或工作需要执行，但它们不替代上述正式 C04 触发，也不产生正式 C04 Gate Decision。

如果项目配置要求更高频率，可以增加 C04，但不得降低 C04 独立性或把辅助模型调用伪装成正式 C04。

---

## 6. Session 连续性与独立触发

默认：

```text
DEFAULT_SESSION_ACTION: CONTINUE_CURRENT_SESSION
```

只要当前 Role、授权、Review Target、独立性和上下文健康兼容，继续当前 Session。阶段变化本身不强制关闭用户面对的 C00 逻辑控制通道。

以下事件无条件要求新建独立 Session：

- 初次正式 C04；
- Finding 关闭或 Review Target 改变后的正式 C04 复审；
- 参与实现/整改的 Session 转入 Independent Reviewer Role；
- 其他现有权威规则明确要求角色或上下文独立的事件。

当某项工作已经由权威规则或预授权决定为“必须独立执行”时，自动创建请求只允许使用以下封闭 Reason Code：

```text
FORMAL_C04_INITIAL
FORMAL_C04_REREVIEW
EXPERT_ESCALATION_WITH_SELF_CONTAINED_PACKAGE
INDEPENDENT_ADVISORY_ANALYSIS
OTHER_PREAUTHORIZED_INDEPENDENT_SESSION
```

`OTHER_PREAUTHORIZED_INDEPENDENT_SESSION` 必须引用精确批准和触发规则，不得作为任意创建外部会话的逃生口。

`EXPERT_ESCALATION_WITH_SELF_CONTAINED_PACKAGE` 和 `INDEPENDENT_ADVISORY_ANALYSIS` 不是所有 Expert / Advisory 调用的默认模式。只有当前工作已明确要求独立性、最小输入包自足且处于授权范围时，才能使用这些 Reason Code；普通 Expert 咨询或辅助推理可以继续采用当前 Session 内 Tool Call。

普通上下文交接、阶段切换、工具调用、当前 Session 内 Auxiliary/Advisory 调用、Knowledge Continuation、Baseline Relearn 或普通 Model/Runtime/Harness 替换，不因名称本身自动获得“外部独立 Session”资格。

---

## 7. 持续 C00 控制通道

C00 是项目负责人默认面对的逻辑控制通道：

```text
PROJECT_OWNER
<-> LOGICAL_C00_CONTROL_CHANNEL
    -> Primary work
    -> Expert / independent child Session
    -> formal C04 child Session
    <- controlled result returns to C00
```

物理 C00 Session 仍在以下情况切换为 `C00-vNext`：

- 达到强制上下文阈值；
- 出现事实遗忘、版本混淆或上下文完整性失效；
- 需要 Clean Context Reset / Baseline Relearn；
- 当前物理 Session 无法安全继续。

继续保留：

```text
60% -> 准备 HANDOFF
70% -> 不再接新的大型任务
80% -> 必须切换物理 Session
```

Harness 支持且权限允许时，C00 可以自动建立本地 `C00-vNext`、传递受控 HANDOFF/Baseline Anchor、校验接管并继续。逻辑控制通道不因此改变 Current Truth Owner。

---

## 8. 历史精确 Commit 作为 C04 Target

当前 Commit 或历史 Commit 均可成为正式 C04 Target，但必须同时满足：

```text
FULL_IMMUTABLE_COMMIT: PRESENT
TARGET_RETRIEVABLE: YES
TARGET_REPRODUCIBLE: YES
APPLICABLE_BASELINE: IDENTIFIED
REVIEW_PURPOSE: IDENTIFIED
INDEPENDENT_SESSION: YES
REVIEW_RECORD_LOCATION: DEFINED
```

`PASS` 只适用于被评审的精确 Commit，不自动适用于后代、分支尖端或当前 `HEAD`。只记录可移动 `HEAD` 仍然不满足正式评审条件。

---

## 9. Profile 变更

更换 Profile 或 Override 前必须说明：

- 为什么改变；
- 哪些活动频率变化；
- 哪些不可关闭项保持不变；
- 对当前 Gate、任务和证据的影响；
- 生效和失效边界。

Profile 变更是治理运行配置，不自动改变产品 Current Truth。当前选择只更新 `CURRENT_STATE.md`，历史变化进入对应治理变更/审计记录。
