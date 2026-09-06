# APLS 0.1 CNL 语义验证与规范化 Profile

- 设计 ID：`DES-APLS-CNL-SEMVAL-001`
- 状态：`ADOPTED_AS_IMPLEMENTATION_INPUT_BY_DEC-023`
- 日期：`2026-09-03`
- 输入：`DEC-004`、`DEC-014`～`DEC-021`、`DES-APLS-ZH-CNL-001`、`DES-APLS-CNL-FRAME-001`、`DES-APLS-CNL-IR-001`
- 关闭 Finding：`BF-04`、`BF-05`、`BF-08`、`BF-09`

> 本 Profile 是 `TASK-019` 形成的公共语义契约，已由 `DEC-023`（`HDP-APLS-018 Option A`）采用为 `TASK-018` 实现输入；尚未建立 Baseline。

## 1. 验证顺序与失败关闭

每个已经完成名称与类别绑定的候选按固定顺序验证：

```text
必需角色
  -> Property 声明类型/单位合法性
  -> Comparison 运算符、Literal 类型和单位兼容性
  -> Action/Target 与 State/Owner 一致性
  -> Acceptance Deadline 正值
  -> Condition 规范化
  -> Transition 文档级检查
  -> Rule 文档级冲突检查
```

候选违反任一规则时被确定性淘汰。单个候选的失败不自动成为公共 Error；最终 `0/1/>1` 类别判定与零候选诊断聚合按 CNL 诊断契约执行。实现不得使用隐式类型转换、常识单位换算、主机浮点数、Locale 或 LLM 补齐本 Profile 未列出的关系。

## 2. Property 声明闭包

`unit_ref` 指 Canonical IR 中的规范值。`显式单位` 指属性声明中 `，单位为“…”` 的 Source 片段。

| `value_type` | 允许的显式单位 | Canonical `unit_ref` | 说明 |
|---|---|---|---|
| `boolean` | 禁止 | `null` | 只表示 `真/假` |
| `integer` | 省略，或一个已声明的名义自定义单位 | `null` 或该名义 Unit ID | 不接受 `%`、时间或温度内建单位 |
| `decimal` | 省略、一个已声明的名义自定义单位，或 `摄氏度` | `null`、该名义 Unit ID 或 `unit:摄氏度` | 温度量只由 `decimal + 摄氏度` 表示 |
| `percentage` | 省略或 `%` | `unit:%` | `%` 是该类型的内建规范单位 |
| `text` | 禁止 | `null` | 文本不携带单位 |
| `duration` | 省略、`毫秒`、`秒` 或 `分钟` | `unit:毫秒` | 声明单位只影响允许的自然表达，不改变规范基准单位 |

补充约束：

- 内建单位无需用户声明，自定义单位必须由同一文档的 `UnitDeclaration` 建立；Declaration Bootstrap 收齐全部 Header 后绑定，因此该 Unit Declaration 可以位于引用它的 Property Declaration 之后，不以 Source 顺序决定合法性；
- `percentage` 与 `duration` 即使省略显式单位也具有上表中的非空 Canonical `unit_ref`；
- `boolean/text` 带单位、`integer` 使用内建量纲单位、`decimal` 使用 `%/毫秒/秒/分钟`、`percentage/duration` 使用表外单位，均为 `APLS-E1401`；
- 相同属性名、类型和 Canonical `unit_ref` 才具有相同声明语义；`duration` 声明写 `秒` 与写 `毫秒` 规范化后等价。

## 3. Comparison 封闭兼容矩阵

运算符集合缩写：`E={EQ,NE}`，`O={EQ,NE,LT,LE,GT,GE}`。未列为允许的组合全部使用 `APLS-E1401` 淘汰。

| Property 规范形态 | 允许运算符 | 唯一允许的 Source 右值 | Canonical `TypedLiteral` | 正例 | 反例 |
|---|---|---|---|---|---|
| `boolean + null` | `E` | `真/假` | `BooleanLiteral` | `安全检查等于真` | `安全检查高于假` |
| `integer + null` | `O` | `signed-integer` | `IntegerLiteral` | `重试次数不高于 3` | `重试次数等于 3.0` |
| `integer + nominal U` | `O` | 整数数值和精确相同的 `U` | `Quantity(integer,U)` | `脉冲数等于 3 脉冲` | `脉冲数等于 3.5 脉冲` |
| `decimal + null` | `O` | `signed-integer` 或 `signed-decimal` | `DecimalLiteral` | `增益等于 3` | `增益等于 真` |
| `decimal + nominal U` | `O` | 数值和精确相同的 `U` | `Quantity(decimal,U)` | `目标流量高于 2.5 升每分钟` | `目标流量高于 2.5 摄氏度` |
| `decimal + unit:摄氏度` | `O` | `temperature-value` | `Quantity(decimal,dimension:temperature,unit:摄氏度)` | `水温低于 35 摄氏度` | `水温低于 35` |
| `percentage + unit:%` | `O` | `percentage-value` | `Quantity(decimal,dimension:percentage,unit:%)` | `水箱液位低于 20%` | `水箱液位低于 20` |
| `text + null` | `E` | `text-value` | `TextLiteral` | `运行模式等于『自动』` | `运行模式高于『自动』` |
| `duration + unit:毫秒` | `O` | `duration` | `Quantity(decimal,dimension:time,unit:毫秒)` | `等待时长不高于 2 秒` | `等待时长不高于 2` |

精确规则：

- `integer` 不接受带小数点的数，即使小数值在数学上是整数；
- `decimal` 接受整数写法并规范化为 Decimal；
- 名义单位只与同一 Unit ID 兼容，不使用显示名相似、复数、缩写或领域常识换算；
- 无单位数值不得与有单位属性比较，有单位数值不得与无单位属性比较；
- `temperature-value` 只匹配 `decimal + unit:摄氏度`；Percentage 和 Duration 使用各自专用 Source 生产式；
- Comparison 不执行 Boolean/Text 与数值之间、Integer/Decimal 与 Percentage/Duration/Temperature 之间的跨类型隐式转换。

Typed-value 的 Grammar Production 名不是规范角色。若同一 Source 片段既可作为专用 `temperature-value/duration`，又可作为 `number-with-declared-unit` 候选，只要按本矩阵得到相同 Type、Dimension、Canonical Unit 和 Value，就必须收敛为同一 Typed Literal；不得因 Production 名不同制造第二个 Canonical Frame，也不得靠 Production 优先级选一个。

## 4. Canonical Value

所有数值以任意精度十进制字符运算，不经过二进制浮点数：

1. Integer 删除负零：`-0 -> 0`；Grammar 已禁止多余前导零。
2. Decimal 删除小数末尾的 `0` 和无用小数点，并将任意零规范化为 `0`；`3`、`3.0`、`3.000` 在允许 Decimal 的位置等价。
3. `percentage` 不做比例换算：`20%` 的规范数值为 `20`，单位为 `unit:%`。
4. `摄氏度` 不做偏移或比例换算，单位为 `unit:摄氏度`。
5. Duration 使用精确乘法换算为毫秒：`毫秒 × 1`、`秒 × 1000`、`分钟 × 60000`；结果再按 Decimal 规则规范化。
6. `integer + nominal` 的 Quantity `numeric_type=integer`；其他允许的 Decimal、Percentage、Temperature 和 Duration Quantity 均为 `numeric_type=decimal`。
7. Acceptance Deadline 必须在换算后严格大于 `0`，否则使用既有时限根因 `APLS-E1307`。

## 5. Condition 合取规范化

对一个表层 Condition：

1. 先逐个规范化 Atomic Condition；
2. 按不含 Provenance 的 Canonical Semantic Payload Byte 升序排序；
3. 对 Payload 完全相同的 Atomic Condition 去重，并把它们的 Role Span 合并为稳定 Provenance 并集；
4. 去重后恰好一项时，Condition 必须降为该 `AtomicCondition`，不得构造单项 `Conjunction`；
5. 去重后两项或以上时，构造 `Conjunction`；其 `items` 已排序且唯一；
6. 空合取不可能由 Grammar 产生，也不得由规范化制造。

因此 `A并且A` 与 `A` 具有相同规范 Condition，但前者的 Provenance 同时保留两个表层 Atom Span。该规则适用于 Rule、Transition、Invariant 与 Acceptance 的每个 Condition 位置。

## 6. Transition 0.1 有限语义

### 6.1 执行含义

`source_state` 是 Transition 的隐式前置条件。一个 Transition 只有在：

```text
entity 当前处于 source_state
AND
trigger 成立
```

时才被启用。Source 不需要在 trigger 中重复状态谓词；IR 继续将 `source_state_ref` 与 `trigger` 分开保存。

### 6.2 结构可达性

对每个 `state_model` 独立建立有向图：

- 节点是该模型声明的全部 State；
- 每个已通过局部验证的 Transition 添加一条 `source_state -> target_state` Edge；
- 图分析故意忽略 trigger 的可满足性，只检查显式状态拓扑；
- 从 `initial_state` 做确定性的图遍历；邻接 Edge 按 Transition ID 的 UTF-8 Byte 升序；
- 任何不可从初态到达的 State 产生一条 `APLS-E1402`。Primary Span 是该 State 的声明名 Span；该模型全部 Transition 的 Sentence Span 按稳定顺序作为 Related Span。

这是一项必要但非充分的可达性检查。0.1 不声称证明一般条件可满足性，也不因未执行 SAT/SMT 推理而猜测结果。

### 6.3 确定冲突

将每个 Transition 放入以下冲突键：

```text
(entity_ref, source_state_ref, canonical_trigger_payload_bytes)
```

- 同一键只有一个 `target_state_ref`：不冲突；完全相同的 Transition 按匿名节点规则合并并取 Provenance 并集；
- 同一键存在两个或以上不同 `target_state_ref`：产生一条 `APLS-E1403`；
- Primary Span 是冲突组中按 `(source_id, sentence_span.start_byte, sentence_span.end_byte)` 排序的第一条 Transition Sentence Span，其余为 Related Span；`missing_or_ambiguous_roles=["target_state"]`；
- 不同 Canonical Trigger 不在 0.1 中做“可能重叠”推理，即使人类认为它们可能同时为真也不报告 E1403；
- 0.1 不使用优先级、Source 顺序或运行时先到事件解决冲突。

任何扩大到一般条件重叠、互斥证明或优先级的规则都必须升级 Language/Semantic Profile，不得由实现私自加入。

### 6.4 Rule 直接冲突（REQUIRE×PROHIBIT）

将每个 Rule 放入以下冲突键：

```text
(canonical_condition_payload_bytes, behavior.actor_ref, behavior.action_ref, behavior.target_ref)
```

- 同一键只出现一种模态：不冲突；完全相同的 Rule 按匿名节点规则合并并取 Provenance 并集；
- 同一键同时出现 `require`（`必须`）与 `prohibit`（`禁止`/`不得`）：构成直接冲突，产生一条 `APLS-E1405`，编译拒绝；
- Primary Span 是冲突组中按 `(source_id, sentence_span.start_byte, sentence_span.end_byte)` 稳定排序的第一条 Rule Sentence Span，其余冲突句为 Related Span；`missing_or_ambiguous_roles=["modality"]`；
- Invariant 与 Rule 的冲突、Safety（`安全要求：`）与普通 Rule 的优先级、不同 Canonical Condition 之间的一般重叠或互斥证明，在 0.1 中均不定义；0.1 不使用优先级、Source 顺序或运行时先到事件解决冲突。

任何扩大到上述未定义冲突语义的规则都必须升级 Language/Semantic Profile，不得由实现私自加入。

## 7. Unit Materialization Closure

Canonical IR 的 `units[]` 恰好包含：

1. Source 中每个合法 `UnitDeclaration` 对应的名义 Unit，即使未被 Property 或 Literal 引用；
2. 规范化后被任何 `property.unit_ref`、`QuantityLiteral.canonical_unit_ref` 或 Acceptance Deadline 引用的内建 Canonical Unit。

内建规范单位只有：

| 规范 Unit ID | 何时进入 `units[]` |
|---|---|
| `unit:%` | 至少一个 Percentage Property 或 Quantity 引用它 |
| `unit:毫秒` | 至少一个 Duration Property、Duration Quantity 或 Deadline 引用它 |
| `unit:摄氏度` | 至少一个 Temperature Property 或 Quantity 引用它 |

`unit:秒` 与 `unit:分钟` 是 Terminology/换算阶段的内建 Source Unit ID；Quantity 规范化后只引用 `unit:毫秒`，因此二者不作为非规范中间单位写入 `units[]`。所有 `units[]` 仍按 ID 的 UTF-8 Byte 顺序排列。

`unit_kind=builtin` 的 Unit 是 Compiler 注入节点，没有 Source 声明，不得产生 `source_map` Entry。名义 Unit 和其他所有 Source 派生语义节点必须有且只有一条 Source Map Entry。

## 8. Source Map 与 Span 不变量

### 8.1 覆盖集合

`source_map` 必须一一覆盖：Entity、Source 声明的 Nominal Unit、Property、Action、Event、Surface Alias、State Model、每个嵌套 State，以及 Rule、Transition、Invariant、Acceptance、Informative Item。它不得为 `unit_kind=builtin` 创建伪造 Provenance。

重复匿名节点合并后仍只有一个 Source Map Entry，`provenance[]` 是所有等价 Source 句和所有收敛候选 Role Span 的稳定并集。

### 8.2 Byte Span

- 所有 Span 是对 Manifest 中原始 UTF-8 Byte 的零起点半开区间 `[start_byte,end_byte)`；
- 必须满足 `0 <= start_byte < end_byte <= source_byte_length`；
- 两个端点都必须位于 UTF-8 Scalar 边界；
- Sentence Span 从句子首个非结构空白 Byte 开始，包含末尾全角句号 `。` 的全部 UTF-8 Byte，不包含前后结构空白；
- 每个 Role Span 必须非空、完全包含于对应 Sentence Span，并使用同一 `source_id`；
- `source_id` 必须等于唯一 `source_manifest.files[0].id`；
- 不同 Role Span 可以重叠；同一 `(source_id,sentence_span,role,role_span)` 只能出现一次。

### 8.3 稳定 Role 名集合

Role 名只能取以下值：

```text
sentence, name, entity_kind, value_type, observable, writable, unit,
alias, target_kind, target, owner, state, initial_state,
condition, condition_left, condition_operator, condition_right,
condition_entity, condition_state, condition_receiver, condition_event,
modality, behavior, behavior_actor, behavior_action, behavior_target,
trigger, trigger_left, trigger_operator, trigger_right,
trigger_entity, trigger_state, trigger_receiver, trigger_event,
entity, source_state, target_state, scope,
required_state, required_state_entity, required_state_state,
expected, expected_left, expected_operator, expected_right,
expected_entity, expected_state, expected_receiver, expected_event,
deadline, text
```

Composite Condition 的 Role Span 覆盖整个 Condition；每个 Atomic 的叶角色使用对应前缀。合取中的同名 Role 可以因 Span 不同重复出现。

## 9. Cross-validator 必须拒绝

下列情况即使通过 JSON Schema 也必须拒绝 Verified IR：

- Property/Comparison 不满足第 2～4 节闭包；
- 单项或重复项 `Conjunction`；
- Transition 存在 E1402/E1403；
- `units[]` 少于或多于第 7 节闭包；
- Built-in Unit 带 Source Map，或 Source 派生节点缺少/重复 Source Map；
- Span、Role 名、IR Pointer、Semantic ID、Provenance 并集或排序违反第 8 节；
- `header.unicode_normalization` 或 `header.unicode_data_version` 与批准 Compiler Profile 不同。

这些检查全部完成前不得构造 `VerifiedArtifact`。

## 10. 批准边界

本候选没有改变 Grammar 的句式集合；它精确关闭 Grammar 之后的类型、单位、Condition、Transition 和 IR 复验行为。采用本候选属于公共语义与 IR Cross-validation 变更，必须由 `HDP-APLS-018` 明确批准。
