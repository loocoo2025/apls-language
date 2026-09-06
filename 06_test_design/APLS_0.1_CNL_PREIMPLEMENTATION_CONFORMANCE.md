# APLS 0.1 CNL 实施前最小 Conformance 设计

- 设计 ID：`TESTDES-APLS-CNL-PREIMPL-001`
- 状态：`TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018`
- 日期：`2026-09-03`
- 输入：`DEC-004`、`DEC-017`、`DEC-019`～`DEC-021`、`DES-APLS-CNL-SEMVAL-001`、`DES-APLS-CNL-RESOURCE-001`、`DES-APLS-CNL-UNICODE-001`、`apls-cnl-diagnostic-0.1`
- 风险来源：`IIR-APLS-TASK018-PREIMPL-001 BF-02～BF-10`

> 本文件定义实现前的黑盒判定样例，不包含 Compiler 测试代码，不授权扩大产品行为。每个 Case 只验证已识别的 P0/P1 契约风险。

## 1. 判定通则

- 相同 Case 在候选遍历、HashMap 插入和线程调度扰动下，Exit、解析后按 `APLS-CNL-C14N-0.1` 规范化的 Envelope JSON、IR Canonical Bytes 必须一致；Envelope Object Member 的传输顺序本身不构成语义；
- Source Error 为 Exit `1`，Tool Failure 为 Exit `2`，成功为 Exit `0`；
- Error 时不得产生 Canonical Frame/IR，成功时 IR 必须通过 Schema 与 Cross-validation；
- 下文“同 Hash”指 `header.semantic_hash` 相同，不要求 Source Manifest 或 Provenance Byte 相同；
- 下文 JSON 片段省略与 Case 无关字段，不可当成第二份完整 Schema。

## 2. Case 集

### CNL-C001 — Grammar 支持的状态谓词

Source 包含：

```text
“灌溉水泵”的状态包括“停止”和“故障”，初始状态是“停止”。
安全要求：灌溉水泵处于“故障”状态时，灌溉水泵必须处于“停止”状态。
```

预期：使用 `StatePredicate` 成功；不得要求或接受旧错误示例 `水泵状态等于“故障”`，后者在没有 Text Property 声明时不能被当作状态比较猜测。

来源：BF-03；防止 Profile 示例扩张 Grammar。

### CNL-C002 — Property/Comparison 矩阵正向代表

一个文档声明并使用：Boolean EQ、Integer Ordered、Decimal 接受 Integer 写法、Percentage、Text EQ、Duration，以及 `decimal + 摄氏度` 和 Integer/Decimal 名义单位。

预期：全部通过；`3` 与 `3.0` 在 Decimal 位置形成相同 Decimal Payload；`2 秒` 与 `2000 毫秒` 形成相同 Time Quantity；`20%` 保留数值 `20` 而不是 `0.2`。

来源：BF-04；防止类型/单位实现分叉。

### CNL-C003 — Property/Comparison 矩阵反向代表

分别提交以下独立 Sentence：

```text
安全检查高于真
重试次数等于 3.0
运行模式高于『自动』
水箱液位低于 20
等待时长不高于 2
水温低于 35
目标流量高于 2.5 摄氏度
```

预期：每句零有效类。类型、运算符或显式单位不匹配（`安全检查高于真`、`重试次数等于 3.0`、`运行模式高于『自动』`、`目标流量高于 2.5 摄氏度`）的最具体根因为 `APLS-E1401`；属性声明要求单位而右值为无单位数值（`水箱液位低于 20`、`等待时长不高于 2`、`水温低于 35`）的根因为 `APLS-E1308`（`HDP-APLS-024` Q3=A / `DEC-031` 批准的根因归位）。不得以 `E1101`、隐式转换或默认单位接受。

来源：BF-04/BF-10；防止候选筛选改变最终唯一性。

### CNL-C004 — Condition 重复项降级

比较：

```text
当水箱液位低于 20% 时，系统必须启动灌溉水泵。
当水箱液位低于 20%并且水箱液位低于 20% 时，系统必须启动灌溉水泵。
```

预期：两句分别编译时产生相同 Rule Semantic Payload/匿名 ID；第二句 Condition 是 AtomicCondition，不是单项 Conjunction；其 Source Map 保留两个 Atom Span。两句在同文档时匿名 Rule 合并并取两句 Provenance 并集。

来源：BF-08/BF-09；防止 Schema `minItems=2` 冲突与 Provenance 丢失。

### CNL-C005 — Transition 隐式源状态与结构可达

状态 `待机/运行/急停`，初态 `待机`，Transition 为 `待机->运行` 与 `运行->急停`；Trigger 不重复 Source State。

预期：成功；每条 Transition 的启用条件隐含当前 Source State。删除 `运行->急停` 后，`急停` 的声明名 Span 产生一条 `APLS-E1402`。

来源：BF-05；防止实现自行解释可达性。

### CNL-C006 — Transition 确定冲突与能力边界

同一实体、同一 Source State、同一 Canonical Trigger 分别进入两个 Target State。

预期：一条 `APLS-E1403`，Primary/Related Span 按规范稳定；Source 顺序交换不改变冲突集合。若两个 Trigger Canonical Payload 不同，0.1 不做一般重叠推理，不产生 E1403。

来源：BF-05；防止优先级、SAT 猜测或漏检。

### CNL-C007 — Unit Materialization 五类闭包

分别构造：无任何 Unit、只有隐含 Percentage、Duration 使用 `秒`、显式 `摄氏度`、未使用名义 Unit 声明。

预期：

| Case | `units[]` ID 集合 |
|---|---|
| 无 Unit | `[]` |
| Percentage | `[unit:%]` |
| `2 秒` Duration | `[unit:毫秒]` |
| 摄氏度 | `[unit:摄氏度]` |
| 未使用名义 `升每分钟` | `[unit:升每分钟]` |

`unit:秒/unit:分钟` 不进入输出；Built-in Unit 无 Source Map，名义 Unit 有且只有一条 Source Map。

来源：BF-09；防止不同 IR Byte/Hash。

### CNL-C008 — Source Map 与 Unicode Byte Span

Source 同时含三 Byte 汉字、ASCII、非 BMP Scalar 的说明文本、CRLF 和重复匿名 Rule。

预期：所有 Span 为原始 Byte 的合法 UTF-8 边界；Sentence Span 含 `。` 不含周围空白；Role Span 位于 Sentence 内；所有 Source-derived ID 恰好一个 Entry；重复 Rule 的 Provenance 稳定合并；非法 `start>end`、越界、非 Scalar 边界、未知 Role、Built-in Unit Entry 均使 IR 复验失败。

来源：BF-09；防止不可追溯或跨实现 Span 漂移。

### CNL-C009 — Unicode 17.0.0 NFC

Fixture Family 覆盖：预组合/分解 Accent、Combining Class 重排、Hangul/Jamo、非 BMP、Quick Check Maybe。每对使用 Unicode 17.0.0 `NormalizationTest.txt` 的有效向量。

预期：已 NFC Source 进入句法；非 NFC Source 以 `APLS-E1004` 拒绝，Primary Span 指向原始 Byte 的首个完整差异序列；Header 固定 `NFC/17.0.0`。不存在独立 Compiler Manifest 或新增 `--version` 字段；依赖公开 Unicode 常量与 Binary 绑定常量不符时为 Internal Failure。

来源：BF-07；防止部分 Unicode 检查和数据版本漂移。

### CNL-C010 — 零候选根因聚合顺序无关

构造多个 Token Stream：一条在 Grammar 失败，一条完成 Parse 后在类别/类型失败；以正序、逆序和随机内部枚举执行。

预期：三个执行产生 Byte 完全相同的诊断 Envelope；聚合按 `DES-APLS-CNL-DIAG-001` §1.1 覆盖三级 Terminal Finding（无 Complete Token Stream 时的 Lattice/Sentence Validator 直接 Finding、各 Stream 的 Grammar Terminal Finding、各候选路径首个失败 Stage 的 Terminal Finding），Canonical JSON 去重后应用封闭抑制表——具体 `E1103/E1104/E1105/E1106/E1201..E1403` 抑制同 Span 派生 `E1101`，`E1201/E1204/E1206/E1207` 抑制同 Span `E1203`，`E1401` 抑制同 Span `E1308`，无其他规则。目录 §6 样例句 `温度高的时候适当降低一点速度。` 产生 E1301+E1303 根因组且不含 E1101；词级根因检测不得命中已声明术语名或固定 Token 内部（如 `“最高水位”` 中的“高”不产生 E1301）。不得提升候选局部概率或只保留第一个错误。

来源：BF-10 与 DEC-017；防止遍历顺序成为隐式消歧器。

### CNL-C011 — E1310 Witness 不暴露 IR

构造恰好两个不等价 Canonical Frame 类，并另有失败候选。

预期：该 Sentence 只输出一条 `APLS-E1310`；Payload 通过 Schema，恰好两个按 Payload Byte 选择的 Witness，包含 `differing_roles` 和 SHA-256 Fingerprint，不含 AST、Token Stream、完整 Frame 或 IR。候选枚举扰动不改变 Payload。

来源：BF-10；保护“最终语义唯一”和用户默认不接触内部结构。

### CNL-C012 — 候选资源边界与支配关系

本 Case 按 `DES-APLS-CNL-RESOURCE-001` 第 1.1 节分成三项，明确区分公开可达证据和防御性计数器证据。

#### CNL-C012A — 公开 Pipeline 可达资源

对 `candidate_lattice_edges/complete_token_streams/candidate_token_occurrences` 使用确定生成式 `.apls` Fixture：分别构造一个恰好等于上限、一个只多一个规范计数对象的 Source，并经正常 CLI Pipeline 执行。

预期：边界 Source 可继续；超一 Source 为 `APLS-T0007` / Exit `2`，Payload 的 `resource/limit/observed/scope/sentence_index/stage` 与 Profile 完全相同，Primary Span 是规范触发对象。失败 Parse 的完整 Token Stream 仍计全部 Token Occurrence，不允许测试参数绕过任何先行资源上限。

#### CNL-C012B — 防御性受支配计数器

对 `parse_candidate_syntax_nodes/bound_frame_candidates/typed_frame_candidates/canonical_frame_candidates` 使用 `#[cfg(test)]` Module-level Resource Ledger 注入。测试辅助器只能设定私有计数器初值并调用生产同一登记函数；不得进入 CLI、Release Binary、环境变量、Feature、公共 API 或 Source 语义路径。

每项分别验证：计数器从 `limit-1` 登记一个规范对象后达到上限且成功；从 `limit` 再登记一个对象时返回该资源的 `APLS-T0007`，`observed=limit+1`、Stage/Span/Payload 完全符合 Profile，且不构造下游 Artifact。

Syntax Node Span 另使用一个确定节点序列：Ledger 初值为 `999,995`，输入一条已成功 Parse 的双 Atom Comparison Rule，其节点顺序为 `Rule, Conjunction, Atomic-1, TypedValue-1, Atomic-2, TypedValue-2, ActionInvocation`。因此 `Atomic-2` 是第 `1,000,000` 个，`TypedValue-2` 是首次超限的第 `1,000,001` 个，Primary Span 必须精确等于第二个比较右值（含显式单位/内部空格）的 `TypedValue` Span，不能扩大为 Atom、Condition 或整句。该测试验证同一生产登记函数，不声称此 Ledger 前置状态可由公开 Source 达到。

#### CNL-C012C — 生产支配关系

用普通 Source 尽量提高成功 Parse/Syntax/Frame Candidate 计数，验证生产全过程始终满足：

```text
parse_candidate_syntax_nodes <= candidate_token_occurrences
bound_frame_candidates        <= candidate_token_occurrences
typed_frame_candidates        <= bound_frame_candidates
canonical_frame_candidates    <= typed_frame_candidates
```

预期：在相同 `1,000,000` 上限下，一旦需要继续处理导致后四类可能超限的 Source，公开结果只能先是 `candidate_token_occurrences` 的 T0007；Compiler 必须立即终止，不输出受支配资源的第二条诊断，也不得继续产生 Frame/IR。

来源：BF-06/BF-10、NF-03、IIR-APLS-TASK019-REREVIEW-002 NF-05；防止把防御性内部计数器冒充不可达的独立 CLI 接受边界。

### CNL-C013 — 公共诊断第 1001 条

构造稳定聚合后恰好 `1,001` 条 Source Diagnostic。

预期：保留排序前 `999` 条和一条 T0007；Payload 为 `public_diagnostics/1000/1001/document/null/diagnose`，Envelope 为 `tool_failure`，Exit `2`。诊断发现顺序扰动不改变结果。

来源：BF-10 与 DEC-011；防止截断时序漂移。

### CNL-C014 — Diagnostic/IR Schema 与 Cross-validation

对一个有效 Envelope/IR 逐一注入：未知字段、错误 Unicode Header、单项 Conjunction、缺少引用 Unit、伪造 Built-in Source Map、错误 Role、错误 Pointer、非法 Span。

预期：每个变体失败关闭；原对象成功。JSON Schema 负责局部封闭结构，Cross-validator 负责引用、Closure、Span 和算法不变量。

来源：BF-08～BF-10；防止“Schema 通过”被误认为 Verified。

### CNL-C015 — Declaration Bootstrap、前向引用与顺序扰动

构造同一声明图的两个 Source：Property 在 Unit 前声明、Action 在 Entity 前声明、Alias 在 Target 前声明、State Declaration 的 Owner Entity 在后声明；第二个 Source 反转这四组声明顺序。另加入一个行为句，其带引号名称在 Declaration Bootstrap 中能形成 Token Stream、但不能形成 Declaration Parse。

预期：两个 Source 都只用 `GRAM-APLS-ZH-CNL-001`；Bootstrap Edge 的带引号名称均为 `DECLARED_TERM` 且 `symbol_candidate_or_empty=""`，所有 Provisional Header 收集后才绑定，最终 Declaration Graph/Canonical Frame/领域语义 Hash 相同。行为句的 Bootstrap 失败保持内部，并在 Normative Pass 成功。两 Pass 的 Edge/Stream/Token/Node 按 `pass_rank` 和 Profile 累计计数。若替换为错误 Category/Owner 引用，则直接声明错误阻止 Graph 冻结且不产生行为派生错误；不得使用第二 Parser、未计数预扫描、声明顺序或“第一个成功”修复。

来源：IIR-APLS-TASK019-REREVIEW-001 NF-01 / BF-06；防止声明预索引循环改变接受集或资源边界。

### CNL-C016 — 公共诊断总排序碰撞

构造至少三条 Diagnostic：其中两条 `logical_path/start_byte` 相同但 `end_byte/code` 的词典关系相反，第三条与前一条直到 `related_source_spans` 都相同、只在 Payload 或其他完整字段不同；另加入一条无 Primary Span Tool Diagnostic。扰动发现顺序和 Line/Column 缓存顺序。

预期：所有执行都严格按 `(primary_span_group,logical_path_or_empty,start_byte_or_zero,end_byte_or_zero,code,related_source_spans_canonical_bytes,full_diagnostic_canonical_bytes)` 输出；有 Span 项全部先于无 Span项，Line/Column 不参与排序；CNL-C013 的前 `999` 条保留集合随之唯一且 Byte 完全一致。

来源：IIR-APLS-TASK019-REREVIEW-001 NF-02 / BF-10；防止两个公共排序键产生不同 Envelope 和截断集合。

### CNL-C017 — Transition 源状态等于目标状态（E1404）

Source 含 `当…时，“设备”从“运行”进入“运行”。`（源状态与目标状态为同一 State）。

预期：拒绝，唯一诊断码 `APLS-E1404`，Primary Span 指向该 Transition 句的状态引用短语；不复用 `APLS-E1403`（其根因身份仅为同实体/源状态/同 Canonical Trigger 的文档级多目标冲突）。

来源：FORMAL_C04_APLS_0_1_CANDIDATE_001 F-07；防止根因身份漂移。

### CNL-C018 — Rule 的 REQUIRE×PROHIBIT 直接冲突（E1405）

同一实体、相同 Canonical Condition 下针对同一 Action/Target 的一对 `必须` 与 `禁止`（或 `不得`）Rule。

预期：拒绝，恰好一条 `APLS-E1405`，Primary Span 为稳定排序后第一条冲突 Rule 句，另一句为 Related Span，`missing_or_ambiguous_roles=["modality"]`。防过杀：相同模态的重复 Rule 合并接受；不同 Canonical Condition 或不同行为三元组的 Rule 组合不产生 E1405。

来源：PRD-005、FORMAL_C04_APLS_0_1_CANDIDATE_001 F-02（`HDP-APLS-024` Q1=A）；防止矛盾规则进入 Verified IR。

### CNL-C019 — 无单位数值对需单位属性（E1308）

属性声明要求单位（百分比/时长/温度/名义单位）而比较右值为无单位数值，如 `水箱液位低于 20`。

预期：拒绝，根因码 `APLS-E1308`，Primary Span 为该数值；显式单位不匹配仍归 `APLS-E1401`（见 CNL-C003）。

来源：诊断目录 E1308 根因身份（`HDP-APLS-024` Q3=A 补实现）；防止该码成为空承诺。

### CNL-C020 — 条件外并列作用域（E1305）

行为之后出现固定并列连接 Token，如 `当液位低于20%时，系统必须启动水泵并且系统必须停止水泵。`

预期：拒绝，根因码 `APLS-E1305`，Primary Span 为该 `并且` Token；同 Span 的派生 `E1101` 被封闭抑制表抑制。

来源：诊断目录 E1305 根因身份（`HDP-APLS-024` Q3=A 补实现）；`E1309` 在 0.1 无机械识别来源，已从目录删除并注明版本边界，不立样例。

## 3. 实施时的最小证据边界

`TASK-018` 实施后不要求为矩阵做笛卡尔积。最低证据是：

- CNL-C001～C020 各至少一个黑盒或模块级测试；
- CNL-C002/C003 使用等价类代表覆盖矩阵每一行；
- CNL-C009 额外运行 Unicode 17.0.0 官方 Normalization Conformance 数据；
- CNL-C012A/C 使用确定生成器而不把百万级文本纳入版本库；CNL-C012B 只用私有 Module-level Ledger 注入，不增加产品测试接口；
- 不因本设计默认增加 Fuzz、性能、跨平台或完整发布测试。

## 4. 当前未执行

本轮只做文档与 Schema 机械自检。Compiler 代码尚未获恢复授权，因此这些 Case 当前是实施验收合同，不是已通过证据。
