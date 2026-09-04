# APLS 0.1 名称解析与符号类别 Profile

- 设计 ID：`DES-APLS-NAME-001`
- 状态：`SOURCE_FORM_REVIEW_REQUIRED_BY_DEC-014`
- 日期：`2026-09-03`
- 输入：`DEC-004`、`DEC-007`、`DEC-009`、`DES-APLS-LANG-001`、`DES-APLS-IR-001`、`DES-APLS-DIAG-001`、`DES-APLS-COMPILER-001`
- 对应裁决包：`HDP-APLS-009`
- 批准依据：`HDP-APLS-009 / OPTION A`、`DEC-013`

> 本文已获准作为 Declaration Indexer / Resolver 的受控设计输入，但不是产品 Baseline，也不代表 Compiler 已实现或通过验证。

> `DEC-014` 之后，符号类别和“恰好一个候选”原则仍是复用输入；`N/A.N/P.N/A.P.N` 不再是用户必写的公开 Source 形式，必须在 CNL 术语与引用设计中重新冻结。

## 1. 待关闭的规则缺口

已批准设计规定了 FQN、显式 Import Alias、禁止遮蔽和“恰好一个候选”，但没有唯一规定下列行为：

- `Execution.owner`、`Event.source`、`Channel.from/to` 可以引用哪些符号类别；
- 六类 Trigger、表达式裸名称、`safety prohibit` 的精确绑定目标；
- Import Alias、顶层声明、State、Transition 和局部 Binding 是否共享命名空间；
- 本地 State 和导入 State 的源码限定形式；
- `set target = expression` 的目标类别。当前 0.1 没有 Variable、Property 或其他可写数据声明，因此任何绑定都会发明未批准语义。

## 2. 已批准的命名空间模型

### 2.1 可导出符号

| Surface 声明 | Resolver 类别 | 稳定 ID |
|---|---|---|
| `type/enum/record` | `type` | `<spec>.<name>` |
| `dimension` | `dimension` | `<spec>.<name>` |
| `unit` | `unit` | `<spec>.<name>` |
| `domain` | `domain` | `<spec>.<name>` |
| `component` | `component` | `<spec>.<name>` |
| `operation` | `operation` | `<spec>.<name>` |
| `transport` | `transport` | `<spec>.<name>` |
| `event` | `event` | `<spec>.<name>` |
| `execution` | `execution` | `<spec>.<name>` |
| `channel` | `channel` | `<spec>.<name>` |
| `state_machine` | `state_machine` | `<spec>.<name>` |
| `state` | `state` | `<spec>.<machine>.<name>` |
| `rule/constraint/safety/acceptance` | 对应同名类别 | `<spec>.<name>` |

`decision/open/unknown` 是具名 Knowledge Item，参与全局 ID 唯一性检查，但不进入 Canonical IR `symbols` 的 normative 符号类别。`transition` 是具名嵌套 IR 节点，ID 为 `<spec>.<machine>.<name>`，不作为可导入顶层符号。

Enum Variant 是内部 `enum_value` 类别，语义身份为 `<spec>.<enum>.<variant>`；它不单独进入 IR `symbols`，而是规范化为具有该 Enum `result_type` 的 Literal Expression。

### 2.2 重复和遮蔽

- Source Graph 内的 `spec` 名必须唯一，否则其 FQN 不能作为全局稳定 ID。
- 同一 Spec 中所有具名顶层声明，包括 `decision/open/unknown`，共享一个名称空间。
- 每个 State Machine 内的 State 和 Transition 共享一个嵌套 ID 空间。
- Enum Variant、Record Field、Operation Parameter、Acceptance Given 分别在其各自 Owner 中唯一。
- 每个调用参数列表和 Schedule 参数列表中，Named Argument 名唯一。
- Import Alias 在所属 Source 中唯一，且不得与该 Source 的任一顶层声明同名。
- Acceptance Given 不得与当前 Spec 顶层声明或当前 Source Import Alias 同名；Resolver 不使用遮蔽选择候选。
- 任一违反以 `APLS-E2002` 失败；Primary 指向后一定义，Related 指向首一定义。

## 3. Source 中的限定名形式

| 形式 | 唯一解释 |
|---|---|
| `N` | 当前局部作用域的 Binding，或当前 Spec 顶层符号；禁止遮蔽保证两者不同时命中 |
| `A.N` | `A` 是当前 Source 的 Import Alias 时，引用目标 Spec 的顶层符号 |
| `P.N` | `P` 是当前 Spec 的 State Machine 时引用其 State；`P` 是 Enum 时引用其 Variant |
| `A.P.N` | `A` 是 Import Alias，`P` 是目标 Spec 的 State Machine 或 Enum，`N` 是其 State 或 Variant |

源码不使用被导入 Spec 自身名代替 Alias，不透传使用被导入 Source 的 Import，不接受超出上表的任意多段回退。State Action 中 `transition M to S` 的 `S` 以已绑定的 `M` 为唯一作用域；也可写成与 `M` 一致的 `M.S` / `A.M.S`，不一致时失败。

## 4. 引用位置与期望类别矩阵

| 引用位置 | 期望类别 |
|---|---|
| `dimension.base_unit` | 当前 Spec 的 `unit` |
| `unit : dimension` | `dimension` |
| Named TypeRef | `type` |
| Quantity TypeRef 的第一/第二参数 | `dimension` / `unit` |
| Numeric Literal 的 Unit | `unit` |
| `event.payload` | `type` |
| `event.source` | `execution` |
| `execution.domain` | `domain` |
| `execution.owner` | `component` |
| Trigger `call/event/message/interrupt/state_change` | `operation/event/channel/execution/state` |
| `channel.from/to` | `execution` |
| `channel.payload` | `type` |
| `channel.transport` | `transport` |
| State Machine `initial/from/to` | 同一 State Machine 内的 `state` |
| Transition `on` | `event` |
| Expression Call | `operation`；`pure/action` 位置兼容性由 `APLS-E3005` 检查 |
| Expression 裸 Name | 当前 Acceptance 的 `given` Binding；0.1 无其他裸值符号 |
| Expression 限定 Enum Variant | `enum_value`；必须写为 `Enum.Variant` 或 `Alias.Enum.Variant` |
| `emit` / `invoke` | `event` / `operation`；Operation Kind 由 `APLS-E3005` 检查 |
| `transition` 的 Machine / State | `state_machine` / 属于该 Machine 的 `state` |
| `safety prohibit` | `operation/event/channel/execution/state_machine/state/rule` |

Timer Trigger、Schedule 参数、Guard、Rule/Constraint/Safety/Acceptance 表达式递归使用同一表达式绑定规则。唯一名称存在但类别不在期望集合时以 `APLS-E2005` 失败。唯一目标为 `open/unknown` 且被 normative 位置依赖时，使用更具体的 `APLS-E4004`，不重复发出 `APLS-E2005`。

## 5. `set` 的闭合方案

APLS 0.1 Grammar、Surface AST 目标模型和 Canonical IR Action 集合不包含 `set`，但保留 `set` 为词法保留字，防止旧文本被误解释为标识符。

理由：

- 0.1 当前没有可写声明、可见性、所有权、初值、并发写或类型契约；
- 把 `set` 绑定到 Component、Record Field、State 或任意 FQN 都会产生不同语义；
- 暂时移除不影响 `emit/invoke/transition` 构成的闭合动作子集；
- 未来要恢复 `set`，必须先引入明确的 Mutable Property/State Data 声明及 IR 类别，不得只在 Resolver 中特判字符串。

## 6. 失败和确定性

- 零候选：`APLS-E2001`；不做近似匹配或隐式导入。
- 重复定义：`APLS-E2002`；不依赖遍历顺序选胜者。
- 多候选：`APLS-E2003`；列出全部候选 Related Span，不选“最近”。
- 类别不匹配：`APLS-E2005`；已找到的声明作为 Related Span。
- Indexer 任一 Error 时不构造 Declaration Index；Resolver 任一 Error 时不构造部分 `BoundProgram`。
- 符号与 Finding 按逻辑路径、Byte Span 和 Code 稳定排序，不使用 HashMap 迭代顺序决定输出。

## 7. 同步与实现边界

1. Language Design、正式 EBNF、Canonical IR 文档与 JSON Schema 已按 `DEC-013` 同步；
2. 现有 Parser/AST 尚未同步的实现偏差由 C03 在 `TASK-013` 一次修正，并实现 Declaration Indexer 和 Resolver；
3. `TASK-013` 只运行 Grammar 生成、Compiler 定向测试和 Workspace `fmt/check/test --locked --offline`；
4. 自检不得冒充 C04、Baseline Adoption 或 Compiler 目标完成。
