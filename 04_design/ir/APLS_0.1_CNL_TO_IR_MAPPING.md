# APLS 0.1 CNL Canonical Frame 到 Canonical IR 映射评估

- 设计 ID：`DES-APLS-CNL-IR-MAP-001`
- 状态：`GAPS_CLOSED_BY_DEC_019_NOT_BASELINED`
- 日期：`2026-09-03`
- 输入：`DEC-014`～`DEC-019`、`DES-APLS-CNL-FRAME-001`、`DES-APLS-IR-001`、`DES-APLS-CNL-IR-001`
- 原缺口评估目标：`apls-ir-0.1.schema.json`
- 当前闭合目标：`apls-cnl-ir-0.1.schema.json`

> 本文先评估旧 IR 能否无损承载 CNL Canonical Frame，再记录新 CNL IR 的缺口闭合。结论是：旧 Schema 不能无损承载全部 CNL 语义；新 Schema 已获准作为实现输入，但在 `TASK-018` 完成复验前仍禁止发布 CNL Verified IR。

## 1. 映射不变量

- 映射输入必须是 Convergence Gate 后唯一 Canonical Frame 等价类，不得重新解析 Source；
- 每个规范角色必须有唯一 IR 位置，不能丢字段、借说明文本补义或由下游猜测；
- Frame 到 IR 必须保持语义；若两个不等价 Frame 被映射成相同 IR，Schema 或映射即不合格；
- 中文显示名与机器稳定 ID 分离，显示名不能被无依据音译、哈希截断或顺序编号代替；
- Schema 不支持的 Frame 必须阻止 Verified IR 发布。

## 2. 逐 Frame 映射

| Canonical Frame | 现有 IR 候选位置 | 可复用部分 | 阻断性缺口 |
|---|---|---|---|
| `EntityDeclaration` | `components[]` + `symbols[]` | 可表示机器 ID 和 component Symbol | `namedNode` 只有 `id`，无法保存中文 `display_name` 和 Device/Component/Sensor/Actuator 的 `entity_kind` |
| `UnitDeclaration` | `dimensions[]` + `units[]` | 现有单位支持维度、符号、scale、offset | CNL 自定义单位声明只有名称，缺少 dimension/base/scale/offset；语言声明本身也不足以构造现有 Unit |
| `PropertyDeclaration` | `types[].record.fields[]` 候选 | 可复用 `type_ref` | 没有 Owner、顶层 Property Symbol、observable/writable 和可选 Unit 的无损结构 |
| `ActionDeclaration` | `operations[]` | `kind=action`、参数与 null return 可复用 | 没有 Action 所属/目标实体关系，无法验证 `action.target` |
| `EventDeclaration` | `events[]` | ID、payload/source 的空值可表示首版无 Payload Event | 缺少显示名；是否允许 `source_ref=null` 需明确为 CNL 语义 |
| `AliasDeclaration` | 无独立规范节点 | Alias 可在绑定后消失，引用直接指向目标 Symbol ID | 必须决定 Alias 仅保留在 Source Map/符号 Provenance，还是扩展 Schema；当前一般 Alias 与 `types.alias` 语义不同 |
| `StateDeclaration` | `state_machines[]` / `states[]` | 初态和 Owner-scoped State 结构大体可复用 | State Machine 机器 ID、实体 Owner、中文显示名、默认 `terminal` 语义未定义 |
| `Rule(REQUIRE)` | `rules[]` | 条件与 Action 数组可部分承载 | Rule 没有 `modality`；句子没有显式规范 ID，而现有 IR 禁止匿名规范节点 |
| `Rule(PROHIBIT)` | `rules[]` 或 `constraints[].safety` 均不精确 | 条件、动作可部分承载 | 普通禁止不必然是 Safety；现有两处都无法无损表达 `PROHIBIT` 模态 |
| `Transition` | `state_machines[].transitions[]` | from/to/guard/actions 可复用 | 现有 `event_ref` 必需，但 CNL trigger 是一般 Condition；优先级也未由 CNL 定义 |
| `Invariant` | `constraints[].safety` | Condition 和 Safety 类别可复用 | 现有结构表达 prohibited_refs/actions，不表达“required_state 必须成立” |
| `Acceptance` | `acceptance[]` | expect/within 可复用 | 现有 `when` 是 Action 数组，而 CNL `trigger` 是 Condition；`given` 映射不明确 |
| `InformativeItem` | `knowledge_items[]` | informative/text/subject_ref 可复用 | `kind` 只允许 intent/responsibility/rationale，没有一般说明类别；稳定 ID 同样缺失 |

## 3. 横切 Schema Gap

### GAP-CNL-IR-001 — 语义 ID 与显示名称分离

现有所有 `id` 只允许 ASCII FQN：

```text
^[A-Za-z_][A-Za-z0-9_]*(\.[A-Za-z_][A-Za-z0-9_]*)*$
```

CNL 声明名允许中文，但语言 Profile 没有规定如何给中文概念建立稳定 ASCII Semantic ID。需要新增显式、可审计且与声明顺序无关的 ID 规则，并在 IR 节点保留 `display_name`。禁止 Compiler 自行音译或选择任意编号。

### GAP-CNL-IR-002 — CNL 声明信息不完整

Entity Kind、Property Owner/Access/Unit、Action Target 和 Alias Provenance 没有完整 IR 位置；自定义 Unit 句式也缺少现有 IR 所要求的维度与换算信息。

### GAP-CNL-IR-003 — 模态和一般条件触发

现有 `rule` 没有 REQUIRE/PROHIBIT 模态；Transition 的 `event_ref` 和 Acceptance 的 Action `when` 不能承载一般 CNL Condition。

### GAP-CNL-IR-004 — Invariant 断言结构

当前 Safety Constraint 以 prohibited_refs/actions 为中心，不能无损表示 `condition -> required_state`。

### GAP-CNL-IR-005 — Source 派生规范节点的稳定身份

CNL Rule、Transition、Invariant、Acceptance 和 InformativeItem 没有显式名称，而现有 IR 禁止匿名规范节点。必须冻结稳定 ID 来源；Byte Offset 可用于 Provenance，但直接作为长期语义 ID 会使无语义编辑改变身份。

### GAP-CNL-IR-006 — 候选 Provenance 收敛

多个分析候选可收敛成一个 Canonical Frame。IR Source Map 需要保留该等价类全部 Source Role Span 的稳定并集，同时不让 Tokenization/Parse 身份进入规范语义哈希。

## 4. 可立即复用的 IR 原则

- UTF-8 Canonical JSON、稳定 Object Key 和集合排序；
- Header、Source Manifest、Compiler/Language/Schema Version；
- 失败关闭、Schema 复验、跨引用验证和 Verified Artifact 边界；
- 类型化 Expression、Canonical Quantity 的基本方向；
- normative/informative/open/unknown 的机械隔离；
- Source Map 和不把概率、时间、随机数写入规范语义。

这些原则的可复用不等于现有 Schema 已兼容 CNL。

## 5. 推荐闭合路线

推荐先把 Frontend 实现边界冻结在“Source → 唯一 Canonical Frame 等价类”，随后建立一个设计工作包修订 `DES-APLS-IR-001` 与 Schema：

1. 决定稳定 Semantic ID 与中文 Display Name 契约；
2. 补齐 Entity、Property、Action、Alias 和 Unit 模型；
3. 为 Rule 增加明确 Modality，为 Transition/Acceptance 统一 Condition Trigger；
4. 给 Invariant 增加一般 Required Assertion；
5. 冻结派生规范节点 ID 与 Candidate Provenance 并集；
6. 建立 Frame→IR 的双向可追溯 Conformance 表；
7. 通过 Schema 和映射评审后才允许 C03 接入 IR Publisher。

不推荐把 CNL Frame 硬塞入旧字段，也不推荐先实现有损映射再补 Schema；二者都会让两个不同含义生成同一个 Verified IR，直接违反 `DEC-017`。

## 6. 历史 Gate 与当前 Gate

### 6.1 `DEC-019` 之前的历史 Gate

```text
CNL Source -> Canonical Frame: 设计路线可进入批准
Canonical Frame -> Verified IR: BLOCKED_BY_SCHEMA_GAPS
```

该代码块只记录 `HDP-APLS-014` / `DEC-018` 时点的历史状态，不是当前 Gate。

### 6.2 当前 Gate

```text
CNL Source -> Canonical Frame: TASK-019_CONTRACT_REMEDIATION_IN_PROGRESS
Canonical Frame -> apls-cnl-ir-0.1: SCHEMA_GAPS_CLOSED_BY_DEC_019
Verified IR implementation: BLOCKED_PENDING_TASK019_REREVIEW_AND_HDP_APLS_018
```

`HDP-APLS-015 Option A` / `DEC-019` 已批准 `DES-APLS-CNL-IR-001` 和 `apls-cnl-ir-0.1.schema.json`，GAP-CNL-IR-001～006 均已闭合为实现输入。`TASK-019` 只整改实施前复审发现的类型/单位、Unicode、Unit Closure、Source Map 与诊断等横切契约；它不把已关闭的旧 Schema Gap 重新标记为 Open。

## 7. Gap 闭合候选追溯

| Gap | 候选闭合位置 | 状态 |
|---|---|---|
| GAP-CNL-IR-001 | Unicode Name-derived Semantic ID + `display_name` | CLOSED — DEC-019 |
| GAP-CNL-IR-002 | `entities/properties/actions/surface_aliases/units` 封闭节点 | CLOSED — DEC-019 |
| GAP-CNL-IR-003 | Rule `modality` + Condition-based Transition/Acceptance | CLOSED — DEC-019 |
| GAP-CNL-IR-004 | Invariant `condition + required_state` | CLOSED — DEC-019 |
| GAP-CNL-IR-005 | Kind-separated SHA-256 Semantic Payload ID | CLOSED — DEC-019 / Q-007 |
| GAP-CNL-IR-006 | `source_map[].provenance[]` 稳定并集，排除候选路径身份 | CLOSED — DEC-019 |

设计缺口闭合不等于 Compiler 已实现或 Schema 已成为产品 Baseline；`TASK-018` 完成并通过复验前仍禁止发布 CNL Verified IR。
