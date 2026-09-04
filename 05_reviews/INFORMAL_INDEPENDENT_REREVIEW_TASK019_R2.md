# TASK-019 第二轮实施前非正式独立再复审

## 1. 评审身份与结论

- Review ID：`IIR-APLS-TASK019-REREVIEW-002`
- Review Line：`INFORMAL_INDEPENDENT`
- Reviewer：全新独立 Reviewer；未参与两轮 TASK-019 整改设计，不是正式 C04
- Target ID：`IIR-TARGET-APLS-TASK019-002`
- Target Access：`READ_ONLY`
- 评审结论：`CHANGES_RECOMMENDED`
- Gate 权威：`ADVISORY_ONLY`
- 唯一写入：本报告

第二轮整改已经关闭 NF-01、NF-02、NF-04，并补齐 NF-03 所要求的逐 Syntax Node Span 表；BF-02/03/04/05/07/08/09 保持关闭且未见回归。但 CNL-C012 的 `parse_candidate_syntax_nodes` 超限路径在当前资源上限和计数顺序下不可达，且同一支配关系还影响 `bound_frame_candidates/typed_frame_candidates/canonical_frame_candidates` 的独立超限 Case。因此 BF-06、BF-10 与 NF-03 仍为 `PARTIAL`，16 个 Conformance Case 尚不能构成全部资源合同的可执行实施前判定。

## 2. Target 完整性证据

Target 摘要严格按 Target 文件第 1 节算法计算：每个文件形成 `<sha256><两个 ASCII 空格><relative path><LF>`，按 C Locale 整行 Byte 升序排序后，对完整记录流计算 SHA-256。

| 时点 | 文件数 | Target Set SHA-256 | 结果 |
|---|---:|---|---|
| 开始 | 38 | `bfada6b1248b6f1ed3ad3222392d1956d81462d18b1f5ab9eb99e45998d26c14` | MATCH |
| 报告写入前 | 38 | `bfada6b1248b6f1ed3ad3222392d1956d81462d18b1f5ab9eb99e45998d26c14` | MATCH |
| 结束 | 38 | `bfada6b1248b6f1ed3ad3222392d1956d81462d18b1f5ab9eb99e45998d26c14` | MATCH |

本报告不属于 Target，不产生自引用。

## 3. BF-02～BF-10 独立判定

| Finding | 判定 | 独立复审结论 |
|---|---|---|
| BF-02 | `CLOSED` | Frame→IR 文件已把 `BLOCKED_BY_SCHEMA_GAPS` 限定为 DEC-019 前历史 Gate，并单列当前 Gate；没有回归为相反的当前状态。 |
| BF-03 | `CLOSED` | Profile、Grammar、Frame、IR 与 CNL-C001 继续统一使用 Grammar 支持的 State Predicate，没有恢复旧的状态比较示例或扩张接受集。 |
| BF-04 | `CLOSED` | 六种 Property 类型、单位形态、运算符、唯一 Literal、Canonical Value 与 E1401 表外拒绝矩阵保持封闭；Schema 与 Cross-validator 边界清楚。 |
| BF-05 | `CLOSED` | Transition 的隐式 Source State、结构可达、同 Canonical Trigger 多目标冲突、E1402/E1403、Span/Related 及不做一般 SAT/重叠推理的边界保持唯一。 |
| BF-06 | `PARTIAL` | Edge Identity/Sentinel、两 Pass 计数、完整 Stream Token 计数、Syntax Node Taxonomy/顺序及逐节点 Span 已闭合；但 CNL-C012 指定的 Syntax Node 和后三类 Frame Candidate 超限 Fixture 被更早、同上限的 Token Occurrence 计数支配，无法按公共 Pipeline 独立到达。 |
| BF-07 | `CLOSED` | Unicode 17.0.0/NFC、完整判定、Quick Check Maybe、IR Header、Binary 常量与构建/启动断言已形成唯一合同；明确不设独立 Compiler Manifest，也未改变 `apls --version`。Cargo 仍未加入候选依赖。 |
| BF-08 | `CLOSED` | Atomic Payload 排序去重、一项降 Atomic、两项以上保留 Conjunction、Provenance 稳定并集与 Schema `minItems=2` 保持一致。 |
| BF-09 | `CLOSED` | Unit Materialization Closure、Built-in Source Map 豁免、Source-derived 覆盖、封闭 Role、非空半开 UTF-8 Byte Span 与 Cross-validator 责任保持闭合。 |
| BF-10 | `PARTIAL` | 零候选首失败 Stage 聚合、封闭抑制、E1310 两 Witness、T0007 Payload、公共诊断上限及唯一总排序键已闭合；但 CNL-C012 无法对四个受支配资源产生其声明的独立 T0007 黑盒结果，相关 Payload/Span 验收不可执行。 |

## 4. NF-01～NF-04 独立判定

| Finding | 判定 | 独立复审结论 |
|---|---|---|
| NF-01 | `CLOSED` | Declaration Bootstrap 已唯一冻结为无类别 `DECLARED_TERM`、同一正式 Grammar 的 `declaration-sentence` Entry、全量 Provisional Header 后绑定、前向引用、直接声明错误阻断 Graph、Bootstrap 失败证据延后聚合及两 Pass 资源累计；没有第二 Parser 或未计数预扫描。CNL-C015 覆盖顺序扰动和行为句 Bootstrap 失败后进入 Normative Pass。 |
| NF-02 | `CLOSED` | Compiler MVP、CNL Diagnostics、通用 Diagnostics、Foundation、Resource 与 CNL-C016 统一采用 `(primary_span_group,logical_path_or_empty,start_byte_or_zero,end_byte_or_zero,code,related_source_spans_canonical_bytes,full_diagnostic_canonical_bytes)`；Null Span、相同 Start 的 End/Code、Related 和最终 Tie-breaker 均唯一，第 1001 条在总排序后确定。 |
| NF-03 | `PARTIAL` | Resource Profile 第 5.1 节已经为全部固定 Syntax Node 给出唯一原始 UTF-8 Byte Span，CNL-C012 也唯一指定第二个 `TypedValue` Span；但该第 1,000,001 个 Node 在公共 Pipeline 中会被先到达的 Token Occurrence 上限截断，因此黑盒期望虽然文字唯一，却不可观测、不可执行。 |
| NF-04 | `CLOSED` | Language Profile、Unicode Profile 与 CNL-C009 一致删除独立 Compiler Manifest，唯一公开字段位于 IR Header；非 IR 命令通过 Binary 常量与构建/启动断言绑定，`apls --version` 精确保持原格式。整改记录 BF-07 行的“Manifest 字段”是残留措辞，但同文件 NF-04 行和全部规范性合同均明确“不设独立 Compiler Manifest”，未形成第二机器接口。 |

## 5. 新 Finding

### NF-05 — 相同上限和先行计数使四类资源超限 Conformance 不可达

- 精确位置：
  - `04_design/compiler/APLS_0.1_CNL_RESOURCE_AND_ORDERING_PROFILE.md:19-23,109-113,134,155-162`
  - `06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md:139-145,183-188`
- 证据：`candidate_token_occurrences`、`parse_candidate_syntax_nodes`、`bound_frame_candidates`、`typed_frame_candidates`、`canonical_frame_candidates` 的 Document 上限都为 `1,000,000`。每条完整 Stream 在 Parser 前先登记其全部 Token Occurrence；Syntax Node 只在该 Stream 成功 Parse 后登记；每个 Bound/Typed/Canonical Frame Candidate 又只能来自一个已成功处理的完整 Sentence Candidate。故后三类 Frame Candidate 数不可能大于已登记的 Token Occurrence 数。对 CNL-C012 指定的双 Atom Comparison Rule，7 个 Syntax Node 也需要不少于 13 个 Token Occurrence；在累计到第 1,000,001 个 Syntax Node 之前，Token Occurrence 必已先尝试超过 1,000,000 并以 T0007 终止。目标文档没有测试专用计数器注入或禁用先行上限的合同。
- 影响：CNL-C012 宣称“每项候选资源边界”均有边界/超一 Fixture，但 `parse_candidate_syntax_nodes` 与三个 Frame Candidate 的独立公共 T0007 Code Path、Payload、Stage 和 Primary Span 无法由同一公开 Pipeline 黑盒观察。Foundation 的“每个资源边界值成功、首个超限稳定失败”也无法按当前数值同时满足。
- 建议关闭条件：由正确 Owner 二选一并同步 Resource/Work Package/Foundation/Conformance：
  1. 调整受支配资源的上限或计数关系，使每个公共资源确有可构造的独立首个超限 Source；该方案会改变 Compiler Version 的能力边界，必须重新 Human Determination；或
  2. 明确这些上限是防御性不变量而非可独立触发的公共黑盒结果，将 CNL-C012 拆分为可达资源的 CLI 黑盒 Case 与受支配计数器的模块级注入 Case，并删除“每项均可黑盒产生 T0007”的承诺。

### NO-04 — 整改记录保留“Manifest 字段”历史措辞

`04_design/compiler/APLS_0.1_TASK019_REMEDIATION_RECORD.md:20` 仍写“Manifest 字段”，而同文件第 32 行及 Language/Unicode/Conformance 明确“不设独立 Compiler Manifest”。它不是规范性第二接口，当前不单独阻断 NF-04；建议下次整改改成“IR Header 字段 + Binary 绑定断言”，避免复审者误读为旧要求仍有效。

## 6. Target 第 3 节十问回答

1. **原 Finding 状态**：BF-02/03/04/05/07/08/09 为 `CLOSED`；BF-06/10 为 `PARTIAL`。第一次 `PARTIAL` 中，BF-07 已关闭，BF-06/10 因 NF-05 未全部关闭。
2. **NF-01 Declaration Bootstrap**：已由无循环、单 Grammar、无类别 Header、全量后绑定、允许前向引用、封闭错误归类与两 Pass 实际资源计数唯一闭合。
3. **NF-02 公共诊断总排序**：已在五类合同中统一为 Byte-based Tuple；相同 Start、不同 End/Code、Null Span、Related Canonical Bytes 和完整对象 Tie-breaker 均有唯一结果。
4. **NF-03 Syntax Node Span/CNL-C012**：逐节点 Span 唯一；指定嵌套节点也是唯一 `TypedValue` Span，但当前上限支配关系使该黑盒超限事件不可达，因此整体为 `PARTIAL`。
5. **NF-04 Unicode 机器接口**：已明确不设独立 Compiler Manifest；IR Header + Binary 常量/构建启动断言是唯一合同，`apls --version` 未改变。仅整改记录有一处非规范性残留措辞。
6. **DEC-017**：最终语义唯一主路径仍完整；没有发现 Pass 顺序、声明顺序、Lexer 优先级、Parser Entry、资源耗尽、概率、LLM 或第一个成功候选被允许制造最终唯一性。NF-05 会导致更早 Tool Failure，但不会把未完成证明误报为 `AMBIGUOUS` 或成功。
7. **两个 JSON Schema**：二者均为合法 JSON，声明 Draft 2020-12；静态遍历共 372 个 Schema Node、108 个本地 `$ref`，全部可解析，无外部 Ref，关键 Regex 与关键字形状通过检查。定向条件探针确认 Envelope 的 accepted/rejected/tool/internal 状态、E1310/T0007 专用 Payload、普通诊断空 Payload、必需 Unicode Header及单项 Conjunction 拒绝符合文档。跨引用、Closure、排序、Span、Pointer 与 Hash 等关系明确由 Cross-validator 承担。
8. **CNL-C001～C016**：C001～C011、C013～C016 对对应 Finding 构成最小充分判定；C012 的 Span 期望精确，但无法独立触发四个受支配资源，因此 16 Case 整体尚非完全可执行合同。
9. **新矛盾/未定义行为/不可实现要求**：发现 NF-05 的资源上限支配与 C012 不可达；另有 NO-04 残留措辞。未发现新的公共语义多解、一般 SAT 承诺、第二 Parser、第二 Manifest 或 Schema/Cross-validator 责任空洞。
10. **越权修改**：四个受保护 Cargo 文件 SHA-256 与 Target 证据逐项一致；`07_src` 排除 `target/` 的 31 个普通文件按路径排序的 `shasum` 记录流摘要为 `5f80929f91c106ff7c0f43ce57633c78eca890e89d6eb6272e4dede7e9c20230`，与冻结证据一致；Cargo/Lock 中没有 Unicode 新依赖。仓库仍无 Commit，且未配置 Git Remote。本 Reviewer 未修改任何 Target、Compiler Source、实现测试、Cargo、治理或远程状态。

## 7. 机械核验

- 38 文件 Target：开始、报告写入前、结束均与冻结摘要一致；
- 受保护 Cargo：四个 SHA-256 全部与 Target 第 5 节一致；
- 非 Target `07_src` 完整性：31 文件摘要与冻结证据一致；
- Schema：JSON 解析、Draft URI、Schema 关键字形状、Regex、本地 Ref 与关键条件探针通过；未安装任何 Schema 工具或项目依赖；
- Conformance：只审查合同的可执行性，未运行 Compiler 测试，未把候选 Case 冒充已通过证据；
- Git/Remote：仓库无 Commit、无 Remote；未 Commit、Push、建 Baseline 或改变远程状态。

## 8. 结论与权限边界

结论：`CHANGES_RECOMMENDED`。

建议仅在 TASK-019 现有 Retry Scope 内关闭 NF-05，并清理 NO-04 措辞，然后冻结新的精确 Target 交由另一名全新 `INFORMAL_INDEPENDENT` Reviewer 复审。本报告不得用于批准公共契约、恢复 TASK-018、修改依赖、提交、推送、建立 Baseline、发起正式 C04、Release 或 Formal Seal。
