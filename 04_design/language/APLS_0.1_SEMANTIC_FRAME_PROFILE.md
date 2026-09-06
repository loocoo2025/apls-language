# APLS 0.1 CNL Semantic Frame Profile

- 设计 ID：`DES-APLS-CNL-FRAME-001`
- 状态：`ADOPTED_AS_IMPLEMENTATION_INPUT_BY_DEC-023`
- 日期：`2026-09-03`
- 输入：`DEC-004`、`DEC-014`、`DEC-015`、`DEC-017`、`DEC-019`、`DEC-021`、`DES-APLS-ZH-CNL-001`、`GRAM-APLS-ZH-CNL-001`、`DES-APLS-CNL-SEMVAL-001`
- 适用范围：`apls-zh-CN-0.1` Source 到现有名称/类型/语义阶段的内部契约

> Semantic Frame 是 Compiler 内部有类型 Artifact。用户默认不查看、不编辑，也不能把 Frame 作为第二条规范性 Source 路径。

## 1. 总体不变量

每个规范句可以先产生多个 `AnalysisCandidate` 和 `BoundFrameCandidate`，但被接受前必须收敛到恰好一个 `CanonicalFrame` 等价类：

```text
surface_sentence_id -> one_or_more AnalysisCandidate
AnalysisCandidate   -> zero_or_one valid BoundFrameCandidate
valid candidates    -> exactly_one CanonicalFrame equivalence class when accepted
frame.required_roles -> all present
frame.symbol_refs    -> exactly_one declared symbol each
frame.value_refs     -> exactly_one type and unit each
```

- Frame 保留完整 Source Span 和各角色的最小 Span；
- Frame 中不保存“最可能”或“置信度”；候选不得因概率或顺序被放弃；
- 公共 Error 存在时不得输出 Canonical Frame；单个候选的确定性淘汰属于内部证据，不是公共 Error，也不得阻止其他候选合法收敛；
- 同一句的多个中间分析或多个批准表面句式可以映射到结构相同的 Canonical Frame；
- Frame 不是 Canonical IR。只有名称、类型、单位和语义检查全部通过后才能 Canonicalize。

## 2. 公共内部字段

候选 Frame 与最终 Canonical Frame 共享下列规范角色。`frame_id` 只在规范化收敛后赋给 Canonical Frame；候选使用不具规范身份的 `candidate_id`：

| 字段 | 类型 | 约束 |
|---|---|---|
| `frame_id` | Internal Stable ID | 仅 Canonical Frame 具有；同一 Source、Profile 和 Compiler Version 下稳定 |
| `candidate_id` | Internal Ephemeral ID | 仅用于完整候选遍历与诊断，不参与语义等价性 |
| `frame_kind` | Closed Enum | 必须属于本文列出的 Frame Kind |
| `source_sentence_id` | Source ID | 一句一主 Frame |
| `source_span` | Byte Span | 零起点、半开区间 |
| `profile_id` | String | 固定为 `apls-zh-CN-0.1` |
| `roles` | Typed Product | 不允许未知字段和缺失必需角色 |
| `provenance` | Source Role Spans | 每个规范角色可回到最小 Source Span |

说明句不产生规范 Frame，只产生独立 `InformativeItem`。

## 3. 声明 Frame

### 3.1 `EntityDeclaration`

| 角色 | 类型 | 必需 |
|---|---|---|
| `name` | DeclaredTerm | 是 |
| `entity_kind` | Device / Component / Sensor / Actuator | 是 |

### 3.2 `UnitDeclaration`

| 角色 | 类型 | 必需 |
|---|---|---|
| `name` | DeclaredTerm | 是 |

### 3.3 `PropertyDeclaration`

| 角色 | 类型 | 必需 |
|---|---|---|
| `name` | DeclaredTerm | 是 |
| `value_type` | Boolean / Integer / Decimal / Percentage / Text / Duration | 是 |
| `observable` | Boolean | 是 |
| `writable` | Boolean | 是 |
| `unit` | UnitRef | 当类型或声明要求单位时 |

`writable=true` 不会自动创建通用写入动作；当前 `set` 禁止决定保持有效。

Property 的合法 `value_type + unit` 组合、内建默认 Canonical Unit 和禁止组合由 `DES-APLS-CNL-SEMVAL-001` 第 2 节封闭定义；Frame 实现不得把表外组合保留为可定型候选。

### 3.4 `ActionDeclaration`

| 角色 | 类型 | 必需 |
|---|---|---|
| `name` | DeclaredTerm | 是 |
| `target` | EntityRef | 是 |

### 3.5 `EventDeclaration`

| 角色 | 类型 | 必需 |
|---|---|---|
| `name` | DeclaredTerm | 是 |

### 3.6 `AliasDeclaration`

| 角色 | 类型 | 必需 |
|---|---|---|
| `alias` | DeclaredTerm | 是 |
| `target_kind` | Entity / Property / Action / Event / Unit | 是 |
| `target` | ExactSymbolRef | 是 |

Alias 不产生新的语义对象，只增加一个唯一表面名称到冻结 Declaration Graph 中 Symbol ID 的映射。Target Declaration 可以位于 Alias Sentence 之后；“既有”指同一文档全部 Provisional Header 绑定完成后存在，不表示 Source 顺序优先。状态别名不在首版接受集中。

### 3.7 `StateDeclaration`

| 角色 | 类型 | 必需 |
|---|---|---|
| `owner` | EntityRef | 是 |
| `states` | Ordered NonEmpty Unique List<OwnerScopedStateTerm> | 是，至少 2 个 |
| `initial_state` | StateRef | 是，必须属于 `states` |

状态显示顺序保留用于诊断，不得被解释为优先级。

## 4. 条件模型

`Condition` 是封闭和类型：

```text
Condition = Comparison
          | StatePredicate
          | EventReceipt
          | Conjunction<AtomicCondition>
```

### 4.1 `Comparison`

| 角色 | 类型 | 必需 |
|---|---|---|
| `left` | PropertyRef | 是 |
| `operator` | EQ / NE / LT / LE / GT / GE | 是 |
| `right` | TypedLiteral | 是 |

比较词映射：`等于→EQ`、`不等于→NE`、`低于→LT`、`不高于→LE`、`高于→GT`、`不低于→GE`。

### 4.2 `StatePredicate`

| 角色 | 类型 | 必需 |
|---|---|---|
| `entity` | EntityRef | 是 |
| `state` | StateRef owned by entity | 是 |

### 4.3 `EventReceipt`

| 角色 | 类型 | 必需 |
|---|---|---|
| `receiver` | BuiltinSystem or EntityRef | 是 |
| `event` | EventRef | 是 |

### 4.4 `Conjunction`

- 表层 `items` 至少两个，全部为 Atomic Condition；
- 表面顺序保留在 Source Map，规范语义 Canonicalization 按后续 IR Profile 处理；
- 不允许 Conjunction 内嵌 Conjunction、Disjunction 或一般 Negation。

规范化必须先按 Atomic Semantic Payload 排序并去重、合并 Provenance。若只剩一项，Canonical Condition 的 Kind 降为该 `AtomicCondition`；只有剩余两项或以上时才形成 Canonical `Conjunction`。因此 Canonical Frame 与 IR 中不存在单项 Conjunction。

## 5. 行为 Frame

### 5.1 `ActionInvocation`

| 角色 | 类型 | 必需 |
|---|---|---|
| `actor` | BuiltinSystem or EntityRef | 是 |
| `action` | ActionRef | 是 |
| `target` | EntityRef | 是 |

`action.target` 必须与 `target` 是同一 Symbol。否则不是第二种解释，而是类别/配对错误。

### 5.2 `Rule`

| 角色 | 类型 | 必需 |
|---|---|---|
| `condition` | Condition | 是 |
| `modality` | REQUIRE / PROHIBIT | 是 |
| `behavior` | ActionInvocation | 是 |

表面 `必须` 映射 `REQUIRE`；`不得` 和 `禁止` 映射 `PROHIBIT`。

以下三种表面结构必须产生相同 Frame：

```text
当 C 时，A 必须 B。
如果 C，A 必须 B。
C 时，A 必须 B。
```

### 5.3 `Transition`

| 角色 | 类型 | 必需 |
|---|---|---|
| `trigger` | Condition | 是 |
| `entity` | EntityRef | 是 |
| `source_state` | StateRef | 是 |
| `target_state` | StateRef | 是 |

两个状态必须属于 `entity` 且彼此不同。

`source_state` 是隐式前置条件；Transition 启用条件是 `entity in source_state AND trigger`。文档级结构可达性与确定冲突严格采用 `DES-APLS-CNL-SEMVAL-001` 第 6 节算法；0.1 不进行一般 Trigger SAT、重叠或优先级推断。

### 5.4 `Invariant`

| 角色 | 类型 | 必需 |
|---|---|---|
| `scope` | Safety | 是，来自固定前缀 |
| `condition` | Condition | 是 |
| `required_state` | StatePredicate | 是 |

Invariant 不通过措辞强弱、领域常识或 AI 判断产生，只由批准的 `安全要求：` 句式产生。

### 5.5 `Acceptance`

| 角色 | 类型 | 必需 |
|---|---|---|
| `trigger` | Condition | 是 |
| `expected` | Condition | 是 |
| `deadline` | Positive Duration | 是 |

`deadline` 必须大于零并转换为 Canonical Duration；单位转换必须精确可表示。

## 6. Sentence-to-Frame 映射

| Grammar Production | Frame Kind | 规范化规则 |
|---|---|---|
| `entity-declaration` | EntityDeclaration | 实体类别映射 Closed Enum |
| `unit-declaration` | UnitDeclaration | 建立单位 Symbol |
| `property-declaration` | PropertyDeclaration | 可见性拆为两个布尔字段 |
| `action-declaration` | ActionDeclaration | 建立 Action/Target 配对 |
| `event-declaration` | EventDeclaration | 建立 Event Symbol |
| `alias-declaration` | AliasDeclaration | Alias 指向全量声明绑定后的 Symbol ID；允许前向引用 |
| `state-declaration` | StateDeclaration | 状态成为 Owner 子符号 |
| `rule-sentence` | Rule | 三种 Condition Lead 归一；两种禁止词归一 |
| `transition-sentence` | Transition | 状态 Owner 必须一致 |
| `invariant-sentence` | Invariant | 固定 Safety Scope |
| `acceptance-sentence` | Acceptance | 时限转 Canonical Duration |
| `informative-sentence` | InformativeItem | 不进入规范 Frame 集 |

## 7. Canonical Frame 收敛与歧义边界

两个有效候选只有同时满足下列条件，才属于同一个 Canonical Frame 等价类：

- Symbol Ref 最终指向相同 Symbol ID；
- Frame Kind 相同；
- 每个必需角色值相同；
- 模态、比较符、类型、单位和时限规范值相同。

Source Span、候选路径、Tokenization 和 Parse Tree 身份不参与语义等价比较；合并时必须保留所有候选 Provenance 的稳定并集，供 Explain 和诊断使用。

仅仅“人看起来差不多”不能去重。若两个候选的任一规范角色不同，它们属于不同等价类。最终等价类基数为 0 时 Source 非法，为 1 时接受，为 2 个及以上时必须以 `AMBIGUOUS` 失败，并给出至少两个分歧 Frame 见证。

任何实现都不得通过任意优先级、最高概率、LLM、声明/遍历顺序或第一个 Parse 将多个不等价类压成一个。

## 8. 与 Canonical IR 的关系

`HDP-APLS-015 Option A` / `DEC-019` 已采用独立 `apls-cnl-ir-0.1`；下列关系继续作为发布前不变量：

- 每个 Frame 角色都有唯一 IR 位置；
- 不需要从 Source 文本重新推断含义；
- InformativeItem 与规范节点机械隔离；
- Source Map 覆盖句子和角色 Span；
- IR Schema 不支持的 Frame 必须阻止发布，而非丢弃字段；
- Property/Comparison、Condition、Transition、Unit Materialization 与 Source Map 必须再次按 `DES-APLS-CNL-SEMVAL-001` 复验。

本次 TASK-019 修改仍是等待 `HDP-APLS-018` 的整改候选，不授权 IR 或 Compiler 实现修改。
