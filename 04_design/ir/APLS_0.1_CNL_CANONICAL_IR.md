# APLS 0.1 CNL Canonical IR 详细设计

- 设计 ID：`DES-APLS-CNL-IR-001`
- Schema ID：`apls-cnl-ir-0.1`
- 状态：`TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018`
- 日期：`2026-09-03`
- 输入：`DEC-014`～`DEC-019`、`DEC-021`、`DES-APLS-CNL-FRAME-001`、`DES-APLS-CNL-SEMVAL-001`、`DES-APLS-CNL-UNICODE-001`、`DES-APLS-CNL-IR-MAP-001`
- Schema：`apls-cnl-ir-0.1.schema.json`

> 本文原设计由 `HDP-APLS-015 Option A` / `DEC-019` 批准；标记为 TASK-019 的类型/单位、Unicode、Unit Closure、Condition 与 Source Map 整改是等待 `HDP-APLS-018` 的候选。它不修改旧 `apls-ir-0.1`，不建立 Baseline，也不单独授权 Compiler 实现。

## 1. 目标与边界

CNL Canonical IR 是全部规范句已完成候选分析、语义绑定、类型/单位检查和 Canonical Frame 收敛后的唯一机器契约。

```text
CNL Source
  -> zero or more intermediate analyses
  -> exactly one Canonical Frame class per accepted sentence
  -> checked document model
  -> apls-cnl-ir-0.1
  -> schema + cross-reference validation
  -> Verified Artifact
```

IR 不接受手写输入，不重新解释 Source，不保留概率，不允许下游根据字段缺失猜测含义。任何 Frame 角色无法无损映射时，Compiler 必须失败。

## 2. 顶层结构

```text
header
source_manifest
entities
units
properties
actions
events
surface_aliases
state_models
rules
transitions
invariants
acceptance
informative_items
source_map
```

- `entities` 到 `acceptance` 是类型化的规范/语言支持节点；
- `surface_aliases` 影响 Source 名称绑定，但不改变绑定完成后的领域行为语义；
- `informative_items` 不改变规范行为；
- `source_manifest/source_map` 只提供来源与解释证据；
- 未知顶层字段和未知节点字段全部失败。

## 3. Semantic ID

`source_manifest` 在 0.1 中恰好包含一个 Entry Source；当前 CNL Grammar 没有 Import 句式。Source ID 为 `source:<logical-path>`，Logical Path 使用 Compiler 已冻结的 `/` 分隔相对路径规则。文件路径和内容摘要用于溯源，不参与领域语义摘要。

### 3.1 声明型节点

用户只写自然中文声明，不填写机器 ID。Compiler 使用已验证 NFC 显示名机械构造：

```text
entity:<entity-name>
property:<property-name>
unit:<unit-name>
event:<event-name>
action:<target-entity-name>:<action-name>
state-model:<owner-entity-name>
state:<owner-entity-name>:<state-name>
alias:<target-kind>:<alias-name>
```

示例：

```text
“灌溉水泵”是设备。
-> entity:灌溉水泵

“启动”是“灌溉水泵”支持的动作。
-> action:灌溉水泵:启动
```

术语字符集不允许 `:`，因此 Segment 不需要转义。ID 使用 Source 已验证的 NFC Scalar 序列，区分大小写，不做拼音、翻译、繁简或视觉归一。重命名术语属于语义身份变更。

内建 Actor 固定为 `builtin:system`。内建单位使用 `unit:%`、`unit:毫秒`、`unit:秒`、`unit:分钟` 和 `unit:摄氏度`。

### 3.2 匿名规范节点

Rule、Transition、Invariant、Acceptance 和 InformativeItem 不要求用户为每句话命名。其 ID 为：

```text
<node-kind>:sha256:<64 lowercase hex>
```

摘要输入是该节点不含 `id` 和任何 Provenance 的 Canonical Semantic Payload，按 `APLS-CNL-C14N-0.1` 序列化后使用 SHA-256。精确 Preimage 为：

```text
UTF8("APLS-CNL-NODE-ID-0.1")
+ 0x00
+ UTF8(node_kind)
+ 0x00
+ canonical_payload_json_bytes
```

`node_kind` 只能是 `rule/transition/invariant/acceptance/info`。Payload 是对应 Schema 节点删除 `id` 后的完整 Object；Object Key 仍按 Canonicalization 规则排序。NUL 只作为 Hash Preimage 分隔 Byte，不进入 JSON。

- 相同语义 Payload 必须产生相同 ID；
- 两个 Source 句子形成同一 Payload 时，IR 合并为一个节点，并将来源加入稳定 Provenance 并集；
- 同一 ID 对应不同 Payload 时属于 Internal Failure，不得通过后缀或顺序号修补；
- Byte Offset、文件顺序、Tokenization、Parse Tree、Compiler 运行时间和声明遍历顺序不得进入摘要。

## 4. Canonicalization 与文档语义摘要

`APLS-CNL-C14N-0.1` 继承旧 IR 的基础 JSON 规则：UTF-8、无 BOM、无尾 LF、Object Key 按 Unicode Code Point 排序、无非必要空白、规范字符串转义、规范十进制字符串。

补充规则：

- 类型化节点集合按 `id` 的 UTF-8 Byte 顺序排列；
- `Conjunction.items` 是逻辑合取，按每个 Atomic Condition 的 Canonical Semantic Payload Byte 排序并去重；
- `state_models.states` 按 State ID 排序；初始状态由 `initial_state_ref` 单独表达；
- Provenance、Role Span 和 Source Map 使用稳定集合排序，不参与领域语义摘要；
- `surface_aliases`、`informative_items`、`source_manifest`、`source_map`、Compiler 身份和 `header.semantic_hash` 不参与文档领域语义摘要；
- Language Profile、IR Schema Version、Entity/Property/Unit/Action/Event/State、Rule/Transition/Invariant/Acceptance 的规范 Payload 参与摘要。

Header 中：

```text
unicode_normalization = "NFC"
unicode_data_version = "17.0.0"
semantic_hash_algorithm = "sha256"
semantic_hash = "sha256:<64 lowercase hex>"
```

Unicode 字段描述 Source 接受检查使用的数据版本与形式，不进入文档领域语义投影；其完整依赖和验证规则见 `DES-APLS-CNL-UNICODE-001`。

文档领域语义投影是下列精确 Object；各数组包含完整节点，包括节点 `id`：

```text
{
  language_profile,
  ir_schema_version,
  entities,
  units,
  properties,
  actions,
  events,
  state_models,
  rules,
  transitions,
  invariants,
  acceptance
}
```

精确摘要 Preimage 为：

```text
UTF8("APLS-CNL-DOCUMENT-SEMANTIC-0.1")
+ 0x00
+ canonical_semantic_projection_json_bytes
```

`semantic_hash` 是该 Preimage 的 SHA-256 小写 Hex，并加 `sha256:` 前缀。该规则同时关闭现有 `Q-007`。`source_manifest/surface_aliases/informative_items/source_map` 和其他 Header 字段不进入投影。

## 5. Unit 与 Typed Literal

### 5.1 内建单位

| Unit | Dimension | Canonical Unit | Scale | Offset |
|---|---|---|---:|---:|
| `%` | `dimension:percentage` | `%` | 1 | 0 |
| `毫秒` | `dimension:time` | `毫秒` | 1 | 0 |
| `秒` | `dimension:time` | `毫秒` | 1000 | 0 |
| `分钟` | `dimension:time` | `毫秒` | 60000 | 0 |
| `摄氏度` | `dimension:temperature` | `摄氏度` | 1 | 0 |

`TypedLiteral(kind=quantity)` 只保存换算后的 Canonical Value、Dimension 和 Canonical Unit Ref。原始写法从 Source Manifest/Source Map 追溯，不进入语义相等判断。

`units[]` 不无条件注入整张表。它恰好包含所有 Source 声明的名义 Unit，以及规范化后由 Property、Quantity 或 Deadline 引用的内建 Canonical Unit。`unit:%/unit:毫秒/unit:摄氏度` 可以进入 IR；`unit:秒/unit:分钟` 只用于 Source 绑定和精确换算，规范 Quantity 统一引用 `unit:毫秒`，因而不作为中间单位写入 IR。完整闭包见 `DES-APLS-CNL-SEMVAL-001` 第 7 节。

### 5.2 自定义单位

首版声明 `“升每分钟”是单位。` 没有提供换算关系，因此不得猜测它与任何常识单位等价。每个自定义单位成为一个名义维度的唯一基准单位：

```text
id                 = unit:升每分钟
unit_kind          = nominal
dimension_id       = dimension:nominal:升每分钟
canonical_unit_ref = unit:升每分钟
scale              = 1
offset             = 0
```

名义单位只与同一 Unit ID 兼容，不自动换算。将来支持显式维度/换算声明时必须升级语言与 Schema Version。

### 5.3 Literal 封闭和

```text
TypedLiteral = BooleanLiteral
             | IntegerLiteral
             | DecimalLiteral
             | TextLiteral
             | QuantityLiteral
```

Integer/Decimal 使用规范十进制字符串，避免 JSON Number 和主机浮点差异。Duration、Percentage、Temperature 与自定义工程量全部使用 `QuantityLiteral`，由 Dimension/Unit 决定语义。

Property 类型、Unit、Comparison Operator 和 Literal 的封闭兼容矩阵及 Canonical Value 规则只由 `DES-APLS-CNL-SEMVAL-001` 第 2～4 节定义；Schema 的结构合法不能替代该 Cross-validation。

## 6. Condition 与行为

```text
Condition = Comparison
          | StatePredicate
          | EventReceipt
          | Conjunction<AtomicCondition>
```

- `Comparison`：Property Ref、EQ/NE/LT/LE/GT/GE、Typed Literal；
- `StatePredicate`：Entity Ref 和 Owner-compatible State Ref；
- `EventReceipt`：`builtin:system` 或 Entity Ref 作为 Receiver，加 Event Ref；
- `Conjunction`：规范排序、去重后至少两个 Atomic Condition，且不嵌套；若表层合取去重后只剩一项，必须降为对应 Atomic Condition，并把重复项 Role Span 合并到 Provenance；
- `ActionInvocation`：Actor Ref、Action Ref、Target Entity Ref；Action 的声明 Target 必须等于 Invocation Target。

## 7. Frame-to-IR 无损映射

| Canonical Frame | CNL IR 节点 | 映射 |
|---|---|---|
| `EntityDeclaration` | `entities[]` | name→display_name，kind→entity_kind，机械生成 ID |
| `UnitDeclaration` | `units[]` | 建立 nominal unit；被规范引用的 built-in canonical unit 由 Compiler 注入；不猜测换算 |
| `PropertyDeclaration` | `properties[]` | type、observable、writable、unit_ref 完整保留 |
| `ActionDeclaration` | `actions[]` | name、target_ref 完整保留 |
| `EventDeclaration` | `events[]` | name 完整保留；首版无 Payload/Source |
| `AliasDeclaration` | `surface_aliases[]` | alias 类别、显示名和 target_ref；不创建新领域对象 |
| `StateDeclaration` | `state_models[]` | owner、states、initial_state 完整保留 |
| `Rule` | `rules[]` | condition、REQUIRE/PROHIBIT、ActionInvocation |
| `Transition` | `transitions[]` | trigger、entity、source/target state；不强制 event_ref |
| `Invariant` | `invariants[]` | scope=safety、condition、required_state |
| `Acceptance` | `acceptance[]` | trigger、expected、positive duration deadline |
| `InformativeItem` | `informative_items[]` | text、kind=note、可空 subject_ref |

该映射不借用旧 IR 中语义不匹配的 `transition.event_ref`、`acceptance.when actions` 或 `safety.prohibited_refs`。

## 8. Provenance 与候选收敛

每个 Source 派生最终节点具有一条 `source_map` Entry；Compiler 注入的 Built-in Unit 是明确例外：

```text
semantic_id
ir_pointer
provenance[]
  source_id
  sentence_span
  role_spans[] { role, span }
```

多个 Tokenization/Parse/Bound Candidate 收敛为同一 Canonical Frame 时：

- 合并相同 `source_id + sentence_span + role + role-span`；
- 按 Source ID、Sentence Span、Role、Role Span 稳定排序；
- 不保存概率、候选排名或“获胜 Parse”；
- Explain View 可以从内部非规范 Trace 展示分析过程，但 Trace 不进入 Verified IR 或语义 Hash。

`unit_kind=builtin` 的 Unit 是 Compiler 注入节点，没有 Source 声明，因此是唯一 Source Map 覆盖例外：不得为它伪造 Entry。Entity、名义 Unit、Property、Action、Event、Alias、State Model、每个 State、Rule、Transition、Invariant、Acceptance 和 Informative Item 必须各有且只有一条 Entry；重复匿名节点先合并，再取全部句子与候选 Role Span 的稳定并集。

Span、唯一 Source ID、IR Pointer、稳定 Role 名和覆盖集合必须满足 `DES-APLS-CNL-SEMVAL-001` 第 8 节。Schema 只检查局部形状；Cross-validator 必须检查端点顺序、UTF-8 Scalar 边界、Source Byte 范围、Role 包含关系、Pointer 指向与覆盖闭包。

## 9. Schema 与跨节点验证

JSON Schema 负责封闭结构、必需字段、基本 Pattern、枚举和互斥节点。IR Validator 还必须检查：

- 所有 ID 全局唯一且符合其节点类型的构造算法；
- 所有 Ref 唯一指向类别兼容节点；
- State 属于引用的 State Model/Entity；
- Property 类型、Unit 和 Comparison Literal 兼容；
- Action Target 与 Invocation Target 相同；
- Transition 两个状态属于同一 Entity 且不同；
- Transition 的 Source State 是隐式 Guard，并通过结构可达性和确定同 Trigger 冲突检查；
- Acceptance Deadline 是正的 Time Quantity；
- Built-in/Nominal Unit 满足第 5 节不变量；
- 匿名节点 ID 与 Payload SHA-256 一致；
- 文档 `semantic_hash` 与规范语义投影一致；
- `units[]` 精确等于名义声明与规范引用内建单位的 Materialization Closure；
- `source_map` 覆盖每个 Source 派生节点且 JSON Pointer 唯一正确，Built-in Unit 没有伪造 Entry；
- 所有 Span、Role 名与 Provenance 满足 `DES-APLS-CNL-SEMVAL-001`；
- Header 的 NFC/Unicode 版本与 Compiler 内嵌数据一致。

任一失败时不得标记 `verified` 或交给 Publisher。

## 10. 与旧 IR 的关系

- `apls-ir-0.1` 保留为旧 DSL 迁移参考，不被原地改写；
- `apls-cnl-ir-0.1` 是独立 Schema，不承诺与旧 Schema 字段级兼容；
- 两者不能同时成为 APLS 0.1 CNL 的规范输出；
- 本设计已获批；旧 IR 标记为 `LEGACY_DSL_ONLY`，C03 只能实现 CNL IR Publisher。

## 11. 当前批准与未授权

- `HDP-APLS-015 Option A` / `DEC-019` 已批准原设计与 Schema；本文件当前呈现的 TASK-019 整改差异尚待 `HDP-APLS-018` 采用；
- 本批准本身不授权修改 Compiler Source、测试、依赖或旧 IR Schema；
- 不授权建立 Baseline、Formal C04、Commit、Push、Release 或 Formal Seal；
- 实现范围与候选资源上限由 `WP-APLS-CNL-C03-001` / `HDP-APLS-016` 单独控制。
