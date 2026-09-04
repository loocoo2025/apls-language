# APLS 0.1 CNL 候选资源与稳定顺序 Profile

- 设计 ID：`DES-APLS-CNL-RESOURCE-001`
- 状态：`TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018`
- 日期：`2026-09-03`
- 输入：`DEC-011`、`DEC-017`、`DEC-020`、`DEC-021`、`WP-APLS-CNL-C03-001`
- 关闭 Finding：`BF-06`、`BF-10` 中 `APLS-T0007` 计数部分

> 本文不改变 `DEC-020` 已批准的数值，只冻结实现无关的计数对象、稳定顺序、超限位置与 Payload。采用本候选仍需 `HDP-APLS-018`。

## 1. 公共资源表

达到上限合法；首次试图产生第 `limit + 1` 个计数对象时返回 `APLS-T0007` / Exit `2`，不产生 Canonical Frame 或 IR。

| `resource` Payload 值 | 上限 | Scope | 0.1 验收可达性 |
|---|---:|---|---|
| `candidate_lattice_edges` | `1,000,000` | Document | `PUBLIC_SOURCE_REACHABLE` |
| `complete_token_streams` | `4,096` | Sentence | `PUBLIC_SOURCE_REACHABLE` |
| `candidate_token_occurrences` | `1,000,000` | Document | `PUBLIC_SOURCE_REACHABLE` |
| `parse_candidate_syntax_nodes` | `1,000,000` | Document | `DEFENSIVE_DOMINATED` |
| `bound_frame_candidates` | `1,000,000` | Document | `DEFENSIVE_DOMINATED` |
| `typed_frame_candidates` | `1,000,000` | Document | `DEFENSIVE_DOMINATED` |
| `canonical_frame_candidates` | `1,000,000` | Document | `DEFENSIVE_DOMINATED` |

既有 `single_source_bytes=1,048,576`、`source_graph_bytes=16,777,216`、`source_count=256`、`import_depth=32`、`expression_depth=128`、`public_diagnostics=1,000` 和 `canonical_ir_bytes=33,554,432` 继续适用；单 Entry CNL 0.1 不使用 Import，但不得复用这些 Resource 名表达其他含义。

### 1.1 可达资源与防御性受支配资源

上表“可达性”只决定如何形成 Conformance Evidence，不改变计数值、生产检查或 T0007 Payload：

- `PUBLIC_SOURCE_REACHABLE` 必须由普通 `.apls` Source 通过公开 CLI Pipeline 独立构造“恰好到达”和“首次超过”证据；
- `DEFENSIVE_DOMINATED` 是生产代码仍必须保留的防御性计数器，但 APLS 0.1 不承诺存在能让它先于上游计数器触发的公开 Source；其边界通过 `#[cfg(test)]` Module-level Resource Ledger 注入验证；
- Test-only 注入只能设置私有 Ledger 初值并调用与生产相同的登记函数，不得成为 CLI 参数、环境变量、Feature、配置文件、公共 API 或 Release Binary 路径，也不得改变任何 Source 的生产接受结果。

对当前 Grammar 和第 2～6 节顺序，下列支配关系是规范不变量：

```text
parse_candidate_syntax_nodes <= candidate_token_occurrences
bound_frame_candidates        <= candidate_token_occurrences
typed_frame_candidates        <= bound_frame_candidates
canonical_frame_candidates    <= typed_frame_candidates
```

每个成功 Parse Candidate 来自一个已在 Parser 前完整登记 Token Occurrence 的非空 Stream；当前固定 Syntax Node Taxonomy 对每个成功 Stream 产生的节点数不超过该 Stream Token 数。Bound/Typed/Canonical 每级最多为其直接上游的每个完整 Candidate 计一次。由于这些 Document 上限同为 `1,000,000`，正常公开 Pipeline 中 `candidate_token_occurrences` 必然先于或同时支配后四类超限；实现和测试不得伪造后四类可独立黑盒触发的承诺。

若生产执行实际请求登记某个防御性资源的第 `1,000,001` 项，其登记函数仍必须按本文返回对应 `APLS-T0007`；该防御检查不因当前 Grammar 下被支配而删除。上面的计数关系若被生产 Artifact 违反，则另属 Compiler Internal Invariant Failure，不得用测试注入扩大语言接受集。

## 2. 全局处理顺序

资源 Ledger 以单线程语义顺序定义，实现可以并行，但结果必须与下列顺序相同：

1. `pass_rank=0`：Profile Declaration；
2. `pass_rank=1`：对 Profile 后的每个 Sentence 按 `start_byte,end_byte` 升序执行 Declaration Bootstrap 尝试；
3. 所有成功的 Declaration Syntax Candidate 收集完毕后，建立 Provisional Header Set，按 Sentence/Stream/Candidate 顺序完成声明绑定与声明语义收敛；全部成功后冻结 Declaration Graph；
4. `pass_rank=2`：对没有成功 Declaration Candidate 的 Rule、Transition、Invariant、Acceptance、Informative Sentence，按 Sentence `start_byte,end_byte` 升序执行 Normative Pass；
5. 每个 Pass/每句内部依次执行 Edge、Stream、Token Occurrence、Parse Syntax Node；只有行为 Frame 继续执行 Bound、Typed、Canonical 计数，声明只执行声明绑定/图验证，不计 Frame Candidate 资源。

行为句可以前向引用，但两阶段 Pass 顺序仍固定。Source 文本顺序变化可以改变首次超限位置；同一 Source、Profile 与 Compiler Version 不得因集合遍历或线程时序改变。

### 2.1 Declaration Bootstrap 的封闭算法

`pass_rank=1` 是唯一声明启动路径，不存在不计数的预扫描：

1. 输入是 Unicode/Sentence Validator 已冻结的原始 Sentence Span；
2. Bootstrap Lexer 只产生固定字面量、数值和 `DECLARED_TERM`。每个满足语言 Profile 术语字符/长度规则的 `“term-name”` 整体产生一个 `DECLARED_TERM` Edge；它的 `symbol_candidate_or_empty=""`，不得附加类别或 Owner；
3. Enumerator 对每个完整 Bootstrap Stream 调用唯一正式 Grammar 的 `declaration-sentence` Entry。失败 Stream 仍完整计 Edge/Stream/Token Occurrence，但临时 Parser Node 计 `0`；
4. 成功 Candidate 的 Root Production 决定新声明 Header 的类别；声明内 `declaration-reference` 只保存名称及 Expected Category Role。所有成功 Candidate 均收集后，才解析 Entity Owner、Unit、Alias Target、State Owner 和前向引用；
5. 同一 Sentence 的等价声明 Candidate 合并；两个以上不等价 Declaration Semantic Candidate 使用 `APLS-E1310` 拒绝，零个声明 Candidate 的 Sentence 延后到 `pass_rank=2`，不得由 Bootstrap 失败直接拒绝；
6. 任一声明的重复、类别、Owner、别名或引用错误形成直接声明诊断并阻止 Graph 冻结；行为句不再运行，从而不产生派生未声明错误；
7. `pass_rank=2` 使用冻结 Graph 产生 category-labelled Symbol Edge。若同一 Sentence 在声明与行为 Entry 都成功，使用 `APLS-T0006` 终止，不能以 Pass 先后选择含义。

Bootstrap 与 Normative 都调用 `GRAM-APLS-ZH-CNL-001` 的 Entry，不得用 Regex、手写声明 Parser、LLM 或另一份 Grammar 判定声明。每个 Sentence 的 `complete_token_streams` 上限跨 `pass_rank=1/2` 累加；Document Scope 的 Edge、Token、Node 上限同样包含两个 Pass 的实际计数对象。

## 3. Candidate Lattice Edge

### 3.1 封闭 Token Kind Rank

| Rank | Token Kind |
|---:|---|
| 0 | `FIXED_LITERAL` |
| 1 | `DECLARED_TERM` |
| 2 | `INFORMATIVE_TEXT` |
| 3 | `TEXT_VALUE` |
| 4 | `SIGNED_INTEGER` |
| 5 | `SIGNED_DECIMAL` |
| 6 | `UNSIGNED_INTEGER` |
| 7 | `UNSIGNED_DECIMAL` |
| 8 | `EXACT_ENTITY_REF` |
| 9 | `EXACT_PROPERTY_REF` |
| 10 | `EXACT_ACTION_REF` |
| 11 | `EXACT_EVENT_REF` |
| 12 | `EXACT_STATE_REF` |
| 13 | `EXACT_UNIT_REF` |
| 14 | `BARE_ENTITY_REF` |
| 15 | `BARE_PROPERTY_REF` |
| 16 | `BARE_ACTION_REF` |
| 17 | `BARE_EVENT_REF` |
| 18 | `BARE_STATE_REF` |
| 19 | `BARE_UNIT_REF` |

固定关键词、标点、`%` 和 Profile 版本片段使用 `FIXED_LITERAL`；其精确 Source 字面量进入 `normalized_lexeme`。结构空白和 EOF 不形成 Edge。

### 3.2 Edge 身份与顺序

一个不同 Edge 的规范身份是：

```text
(pass_rank, sentence_index, start_byte, end_byte, token_kind_rank,
 symbol_candidate_or_empty, normalized_lexeme)
```

- `pass_rank` 固定为第 2 节的 `0/1/2`；同一 Source Token 在不同 Pass 形成两个实际 Edge 时分别计数；
- `sentence_index` 从 Profile Declaration 起按 Source Sentence Span 排序后零起点编号；
- Symbol Reference Edge 的 `symbol_candidate` 是完整 Semantic ID；State 必须包含 Owner-scoped State ID；
- 非 Symbol Edge 的 `symbol_candidate_or_empty` 固定为空字符串 `""`；合法 Semantic ID 不可能为空，因此不会碰撞；
- `normalized_lexeme` 是该 Token 的 Profile 规范值；引用使用绑定前的 NFC Source 词形，固定字面量使用固定字面量；
- 全部字符串排序均按 UTF-8 Byte 升序；Edge 按上述 Tuple 从左到右升序；
- 完全相同 Tuple 只计一个 Edge；任一字段不同计不同 Edge。

Document Edge Ledger 直接按完整 Edge Identity 排序。第 `1,000,001` 个 Edge 的 Span 是 T0007 Primary Span，`observed=1,000,001`。

Profile 与 Declaration Bootstrap 同样进入 Lattice 计数。若其固定语法只有一个合法切分，仍产生其全部固定 Token Edge 和一个 Complete Token Stream。Bootstrap 中的 `DECLARED_TERM` 属于非 Symbol Edge；只有冻结 Graph 后的 `EXACT_*/BARE_*` 才可携带 Semantic ID。

## 4. Complete Token Stream 与 Token Occurrence

Complete Token Stream 是从 Sentence 首个非空白 Byte 到末尾 `。` 全部 Token Byte 的无空洞、有序 Edge 序列；允许的 Token 边界空白由 Profile 单独验证，不形成 Edge。

- 两个 Stream 只有在同一 `pass_rank` 且 Edge Identity 序列完全相同时才相同；
- 每个 Sentence 的 Stream 顺序是 `(pass_rank, Edge Identity sequence)` 的词典序，并跨 Declaration Bootstrap/Normative Pass 累计 `4,096` 上限；
- 每发现一个完整 Stream 即计数，不因后续 Parse 失败而撤销；
- 试图确认第 `4,097` 个 Stream 时，T0007 Primary Span 为完整 Sentence Span，`observed=4,097`，该 Stream 不进入 Parser；
- 在调用 Parser 前，按 Stream Edge 顺序逐个登记 `candidate_token_occurrences`；失败 Parse 也因此按完整 Stream 长度计数，不按 Parser 实际读取前缀计数；
- 同一 Edge 在不同 Stream 中出现时，每次都计一个 Token Occurrence；
- 第 `1,000,001` 个 Token Occurrence 的 Edge Span 是 T0007 Primary Span。

## 5. Parse Candidate Syntax Node

资源名保留了 `AST` 的历史称呼，但规范计数对象不是 Rust Struct 数或内存分配次数，而是下列封闭的 `Syntax Node`。实现拆分或合并内部类型不得改变计数。

| Source 构造 | 固定节点数 |
|---|---:|
| Profile Declaration | `ProfileDeclaration=1` |
| Entity/Unit/Property/Action/Event/Alias Declaration | 对应 Declaration Root `=1` |
| State Declaration | `StateDeclaration=1`，每个列表 State 另加 `StateItem=1` |
| Rule | `Rule=1`，加 Condition 子树，加 `ActionInvocation=1` |
| Transition | `Transition=1`，加 Trigger Condition 子树 |
| Invariant | `Invariant=1`，加 Condition 子树，加 `RequiredStatePredicate=1` |
| Acceptance | `Acceptance=1`，加 Trigger Condition 子树、Expected Condition 子树、`Deadline=1` |
| Informative Sentence | `InformativeItem=1` |
| 单一 Atomic Condition | `AtomicCondition=1`；若为 Comparison，再加 `TypedValue=1` |
| 含 N 个表层 Atom 的 Condition，N>=2 | `Conjunction=1`，每个 Atom 按上一行计数 |

声明引用、行为引用、运算符、标点和 Literal Leaf 不另计节点；Comparison 的右值只由 `TypedValue` 计一次；Deadline 不再额外计 `TypedValue`。

只有一个完整 Stream 被 LALRPOP 成功解析后，才原子登记该 Candidate 的全部 Syntax Node。失败 Parse 即使内部构造过临时值，也计 `0`。成功 Candidate 内的节点顺序为：Root 前序；字段按 Grammar Source 顺序；Condition 先 Conjunction 后各 Atom；Comparison 先 Atomic 后 TypedValue；StateItem 按列表 Source 顺序。首次超过 Document 上限的 Node Span 是 T0007 Primary Span。

### 5.1 每类 Syntax Node 的唯一 Source Span

所有 Span 都是原始 Source 的零起点半开 UTF-8 Byte 区间；固定词和定界符按下表包含，周围结构空白不扩入 Span：

| Syntax Node | Primary Span |
|---|---|
| `ProfileDeclaration` | 从 `本规范采用` 首 Byte 到结尾 `。` 后一 Byte，即完整 Profile Sentence Span |
| 任一 Declaration Root | 从句首首个非空白 Byte 到结尾 `。` 后一 Byte，即完整 Declaration Sentence Span |
| `Rule/Transition/Invariant/Acceptance/InformativeItem` Root | 从句首首个非空白 Byte 到结尾 `。` 后一 Byte，即完整 Sentence Span |
| `StateItem` | 对应 `state-list` 项从左引号 `“` 首 Byte 到右引号 `”` 后一 Byte；初始状态引用不另建 `StateItem` |
| `Conjunction` | 从第一个表层 Atom 首 Byte 到最后一个表层 Atom 后一 Byte，包含中间全部 `并且` 与 Token 边界空白，不包含 `当/如果`、尾随 `时，/，` 或外层句式 |
| `AtomicCondition` | 该 Atom 第一个引用/Actor Byte 到该 Atom 最后一个 Value/固定词 `状态`/Event Reference 后一 Byte；不包含相邻 `并且` 或外层条件引导词 |
| `TypedValue` | 从值的首 Byte 到值构造最后一 Byte；包含 `『...』` 定界符、数值与单位之间允许的空白及显式 `%/单位`，不包含比较词 |
| `ActionInvocation` | 从 Action Reference 首 Byte 到 Target Entity Reference 后一 Byte，包含二者之间 Token 边界空白，不包含 `必须/不得/禁止` 或结尾 `。` |
| `RequiredStatePredicate` | 从不变量结论的 Entity Reference 首 Byte 到结论末尾固定词 `状态` 后一 Byte，包含 `必须处于`，不包含条件引导或结尾 `。` |
| `Deadline` | `duration` 的完整 Span：从无符号数值首 Byte 到 `毫秒/秒/分钟` 后一 Byte，包含数值与单位间可选空格，不包含 `必须在` 或 `内成立` |

若可选空白位于一个构造的首/末 Token 之间，它属于该构造 Span；首 Token 前和末 Token 后的空白不属于该节点。节点 Span 不因内部 Rust AST 拆分、错误恢复或 Canonical Condition 去重改变。

`parse_candidate_syntax_nodes` 的登记顺序、首次超限节点和本节 Span 均为生产合同；但按第 1.1 节，它在 0.1 公开 Source Pipeline 中受 Token Occurrence 支配。边界与首次超限 Span 使用私有 Module-level Ledger 注入验证，不宣称 CLI 可独立观察该资源先触发。

## 6. Frame 候选计数

- `bound_frame_candidates`：一个完整 Sentence Parse Candidate 的全部引用、类别和 Owner 均成功绑定后计 1；
- `typed_frame_candidates`：一个 Bound Candidate 的必需角色、Property/Comparison 类型单位、Action/State 和 Deadline 均通过后计 1；
- `canonical_frame_candidates`：一个 Typed Candidate 完成 Condition/Value/Modality/Provenance 规范化后、按等价类去重前计 1；
- 每一级都只计完整 Candidate，不计字段、中间对象或失败 Candidate；
- 各级顺序继承 Sentence、Stream 和唯一 Parser Candidate 顺序；
- 首次超限的 Primary Span 是该 Candidate 的完整 Sentence Span，`observed=1,000,001`。

Canonical Frame 等价类数量没有单独资源限额；它不可能大于已计数的 Canonical Candidate 数。

Bound/Typed/Canonical 三个计数器同样保留生产检查和上述 T0007 Sentence Span；按第 1.1 节，它们的独立边界/超限只做 Module-level Ledger 注入验证。普通 Source 若先触发 Token Occurrence 上限，只能公开该先行 `candidate_token_occurrences` T0007，不能继续运行以制造下游资源诊断。

## 7. `APLS-T0007` 精确 Payload

T0007 必须使用 `apls-cnl-diagnostic-0.1.schema.json` 的 `resource_limit` Payload：

```json
{
  "kind": "resource_limit",
  "limit": 4096,
  "observed": 4097,
  "resource": "complete_token_streams",
  "scope": "sentence",
  "sentence_index": 8,
  "stage": "enumerate"
}
```

- 单位步进计数器的 `observed` 固定为 `limit+1`；按 Byte 总量原子加入的 Source Graph 可记录触发时实际总量；
- `sentence_index` 只在 `scope=sentence` 或触发对象属于确定 Sentence 时为整数，否则为 `null`；
- Candidate 资源的 Stage 分别为 `lattice/enumerate/parse/bind/type/normalize`；
- Primary Span 依第 3～6 节确定；无法归属 Source（如公共诊断集合、IR Writer 或读取前 Source Byte 上限）时为 `null`；
- 资源耗尽立即终止当前编译事务，不能输出 E1310、Frame、部分 IR 或旧 Artifact 替代品。

## 8. 公共诊断上限

零候选根因聚合和全 Document 诊断按 `DES-APLS-CNL-DIAG-001` 的唯一 Byte-based 总排序键完成排序后：

- 不超过 `1,000` 条：全部保留；
- 需要第 `1,001` 条时：保留排序前 `999` 条，并以一条 `APLS-T0007` 作为第 `1,000` 条；
- Payload 为 `resource=public_diagnostics, limit=1000, observed=1001, scope=document, sentence_index=null, stage=diagnose`；
- T0007 Primary Span 为 `null`，Envelope `status=tool_failure`，Exit `2`；
- 不允许按发现时序截断。

## 9. 批准边界

本 Profile 只把 `DEC-020` 的数值变成跨实现可复现的计数合同。任何改变上限、Token Kind Rank、节点 Taxonomy、稳定排序或 T0007 Payload 的行为都需要新的 Compiler Version 与 Human Determination。
