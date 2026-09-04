# TASK-019 实施前非正式独立再复审

## 1. 评审身份与结论

- Review ID：`IIR-APLS-TASK019-REREVIEW-001`
- Review Line：`INFORMAL_INDEPENDENT`
- Reviewer：全新独立 Reviewer；未参与 TASK-019 整改设计，不是正式 C04
- Target ID：`IIR-TARGET-APLS-TASK019-001`
- Target Access：`READ_ONLY`
- 评审结论：`CHANGES_RECOMMENDED`
- Gate 权威：`ADVISORY_ONLY`
- 唯一写入：本报告

BF-02、BF-03、BF-04、BF-05、BF-08、BF-09 已达到原报告的建议关闭条件。BF-06、BF-07、BF-10 仍为 `PARTIAL`：剩余问题会使两个实现对声明预索引、资源超限 Span、Unicode Compiler Manifest 或诊断顺序作出不同选择，因此当前 Target 尚不足以直接进入 TASK-018 实施恢复裁决。

## 2. Target 完整性证据

Target 摘要严格按 Target 文件第 1 节算法计算：每个文件形成 `<sha256><两个 ASCII 空格><relative path><LF>`，按 C Locale 整行 Byte 升序排序后，对完整记录流计算 SHA-256。

| 时点 | 文件数 | Target Set SHA-256 | 结果 |
|---|---:|---|---|
| 开始 | 33 | `219e6ad30aa67d09a689efbad3c2fdf9c8ae7e9e01111736e7d069af7b8d4e6c` | MATCH |
| 报告写入前 | 33 | `219e6ad30aa67d09a689efbad3c2fdf9c8ae7e9e01111736e7d069af7b8d4e6c` | MATCH |
| 结束 | 33 | `219e6ad30aa67d09a689efbad3c2fdf9c8ae7e9e01111736e7d069af7b8d4e6c` | MATCH |

开始与写入前摘要均与冻结值一致，未触发 `TARGET_CHANGED_REVIEW_INVALID`。本报告不属于 Target，不产生自引用。

## 3. BF-02～BF-10 独立判定

| Finding | 判定 | 独立复审结论 |
|---|---|---|
| BF-02 | `CLOSED` | `APLS_0.1_CNL_TO_IR_MAPPING.md` 第 6 节已把 `BLOCKED_BY_SCHEMA_GAPS` 明确限定为 DEC-019 前历史 Gate，并单列当前 Gate；状态、Gate 与工作包不再同时表达相反当前事实。 |
| BF-03 | `CLOSED` | Language Profile 已改为 Grammar 支持的 `灌溉水泵处于“故障”状态`；Grammar、Frame、IR 与 CNL-C001 一致，没有扩张状态比较句式。 |
| BF-04 | `CLOSED` | `DES-APLS-CNL-SEMVAL-001` 第 2～4 节封闭列出六种 Property 类型的允许单位形态、E/O 运算符集合、唯一允许 Literal、Canonical TypedLiteral、表外统一 E1401 以及整数/十进制/百分比/温度/时长规范值。矩阵可机械执行且不依赖隐式转换、浮点、Locale 或常识。 |
| BF-05 | `CLOSED` | Transition 已冻结 Source State 隐式前置条件、忽略 Trigger SAT 的结构可达图、从初态遍历、同 `(entity,source_state,canonical_trigger)` 多目标冲突、E1402/E1403、Span/Related 顺序和一般重叠推理非目标；CNL-C005/C006 提供最小正反证据。 |
| BF-06 | `PARTIAL` | Edge 身份、空字符串 Sentinel、声明句计数、Stream 顺序、失败 Parse 的完整 Token 计数及固定 Syntax Node Taxonomy 已闭合；但 `DES-APLS-CNL-RESOURCE-001:119` 只说“Node Span”是首次超限 Primary Span，未把每种 Root/Conjunction/Atomic/TypedValue/ActionInvocation/StateItem/Deadline 节点映射到精确 Source Span。另见 NF-01 的声明预索引顺序缺口。 |
| BF-07 | `PARTIAL` | Unicode 17.0.0、UAX #15 rev57、完整 `is_nfc`、Quick Check Maybe 处理、IR Header、依赖版本/校验和/许可/MSRV 候选均准确，Cargo 仍未修改；但 Language Profile 要求 `Compiler Manifest/IR Header` 均声明版本，而 Unicode Profile 第 3 节只定义 IR Header，没有定义 Compiler Manifest 的文件、字段、序列化位置或复验接口。 |
| BF-08 | `CLOSED` | Grammar Well-formedness、Frame、Semval、IR 与 Schema 已一致冻结：按 Atomic Payload 排序去重；一项降为 AtomicCondition；两项以上才保留 Conjunction；Provenance 取稳定并集。CNL-C004 覆盖该行为。 |
| BF-09 | `CLOSED` | `units[]` 已冻结为全部名义声明加被规范引用的 `%/毫秒/摄氏度` 内建 Canonical Unit；秒/分钟不物化；Built-in Unit 无 Source Map，全部 Source 派生节点一一覆盖；Role Enum、非空半开 Byte Span、UTF-8 边界、Source 长度、Sentence/Role 包含和唯一 Source ID 均由 Cross-validator 封闭。 |
| BF-10 | `PARTIAL` | 零候选首失败 Stage 聚合、封闭抑制、E1310 两个指纹 Witness、T0007 Payload、Envelope 状态与诊断上限已形成机器结构；但 CNL Diagnostic 的最终排序键与已批准 Compiler MVP 排序键不一致，且 T0007 的 Parse Syntax Node Primary Span 仍受 BF-06 缺口影响。 |

## 4. 新矛盾与剩余缺口

### NF-01 — Declaration Pass 的启动顺序与“唯一 Grammar”未闭合

- `APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md:108-113` 要求 Declaration Pass 先解析固定句式和带引号声明名，建立完整术语索引；
- `APLS_0.1_CNL_FRONTEND_ADAPTATION_DESIGN.md:35-40` 与 Work Package `:15-22,36-40,82-85` 同样把 Declaration Pass 放在 Candidate Lattice/LALRPOP 前；
- 但 Grammar `:34-35,106-137` 的声明句本身可要求由 Terminology Indexer 产生的 category-labelled `EXACT_*_REF`，资源 Profile `:31-35,86` 又要求 Declaration Sentence 进入 Lattice、Stream 和 Parse 计数；Frontend `:49-52` 禁止第二 Parser/正则旁路。

当前合同没有说明在术语图尚未存在时，如何识别并验证全部声明句、前向引用和类别终端，同时仍由唯一正式 Grammar 解析并按同一资源合同计数。实现可能选择专用声明 Parser、未计数预扫描或多遍通用 Token，这会改变诊断、资源边界和可接受集。

关闭条件：冻结一个明确的 bootstrap 算法，例如“只扫描声明头的非语义 Token→建立 provisional symbol headers→用同一正式 Grammar 和全部 category candidates 解析声明→完成绑定后冻结图”，并明确预扫描接受边界、错误归类、Span、资源计数及它不是第二规范 Parser；或调整 Grammar/Token 合同消除循环。

### NF-02 — CNL 与已批准 Compiler MVP 的诊断排序键冲突

- `APLS_0.1_COMPILER_MVP_DESIGN.md:203`：`(logical_path,start_line,start_column,code,related_spans)`；
- `APLS_0.1_CNL_DIAGNOSTICS.md:35,47`：声称沿用既有排序，却定义 `(logical_path,start_byte,end_byte,code,related_source_spans Canonical Bytes)`。

两个键在相同起点、不同终点和不同 Code 时可产生不同顺序。该差异直接影响 Envelope Byte、公共诊断第 1001 条保留集合和 CNL-C010/C013 结果，不能由实现自行选择。

关闭条件：由公共诊断契约 Owner 明确一个唯一排序键，并同步 Compiler MVP、CNL Diagnostics、Resource Profile 与 Conformance；若 CNL 键替代旧键，需明确写成候选公共契约变更并纳入 HDP-APLS-018。

### NF-03 — Parse Syntax Node 的 T0007 Primary Span 映射不完整

`DES-APLS-CNL-RESOURCE-001:104-119` 冻结节点数量和顺序，但没有冻结各节点的精确 Span。特别是 Root、Conjunction、AtomicCondition、TypedValue 与 ActionInvocation 可分别选择整句、完整构造或最小角色范围。CNL-C012 只要求“规范触发对象”，无法给黑盒 Fixture 提供唯一期望 Byte 区间。

关闭条件：为 Syntax Node Taxonomy 增加逐节点 Span 表，并让 CNL-C012 至少覆盖一个嵌套 Condition 超限位置。

### NF-04 — Compiler Manifest 的 Unicode 版本声明没有机器落点

`APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md:36` 要求 Compiler Manifest 和 IR Header 精确声明 Unicode 版本；`DES-APLS-CNL-UNICODE-001:58-71` 只给出 IR Header 字段及 Binary 绑定陈述，没有定义 Compiler Manifest Artifact/字段或 `apls --version` 扩展。现有 CLI 版本格式仍是 `apls <compiler-version> language=0.1 ir=0.1`。

关闭条件：定义 Compiler Manifest 的唯一机器接口；如果项目不需要独立 Manifest，则从 Language Profile 删除该要求并明确 IR Header + Binary build assertion 是唯一契约。

## 5. 九项审查问题回答

1. **BF-02～BF-10 是否关闭**：六项 `CLOSED`，三项 `PARTIAL`；详见第 3 节。
2. **新矛盾/缺口/不可实现要求**：发现 NF-01～NF-04。没有发现要求通用 SAT/SMT、概率消歧或无法撤回 stdout 等新的不可实现承诺。
3. **DEC-017 最终语义唯一**：主语义路径仍完整保护 `0/1/>1`；禁止最长匹配、概率、LLM、排名、声明/遍历顺序和第一个成功路径；T0007 仍与 E1310 分离。NF-01 可能让声明预处理顺序成为未声明语义路径，需关闭后才能认为端到端唯一。
4. **Type/Unit/Transition/Condition/Unit Closure/Source Map**：这些语义主体已机械、封闭且跨实现一致；剩余不确定性位于资源节点 Span 和声明 bootstrap，不在矩阵或 IR Closure 本身。
5. **Unicode/NFC/依赖候选**：Unicode 17.0.0、UAX #15 rev57、`unicode-normalization 0.1.25` 的 `is_nfc(&str)`、`UNICODE_VERSION=(17,0,0)`、crate SHA-256、Rust 1.36、MIT OR Apache-2.0、`tinyvec 1.6.0 + alloc`、`tinyvec_macros 0.1.1` 的校验和与许可均复核一致。最终实际 Lock、Feature、MSRV 1.86 构建和许可证据仍按文档等待 HDP-APLS-018；当前 Cargo 未修改。Compiler Manifest 落点仍是 NF-04。
6. **两个 JSON Schema**：二者均为合法 JSON，声明 Draft 2020-12；本轮静态遍历 372 个 Schema Node，全部关键字形状与 ECMA Regex 可解析，108 个本地 `$ref` 全部解析。条件探针确认 E1310/T0007 Payload、普通诊断空 Payload、Envelope Status、Unicode Header、未知字段、单项 Conjunction 和封闭 Role 的预期行为。Span 顺序/边界等跨节点关系按设计由 Cross-validator 承担。
7. **零候选/E1310/T0007**：零候选聚合和 E1310 Witness 可直接实现、可黑盒验证；T0007 Payload 和大多数 Primary Span 已闭合，但 Parse Syntax Node Span 仍不唯一，诊断总排序仍冲突。
8. **14 个 Conformance Case**：对已闭合原 Finding 的覆盖总体最小充分；但在 NF-01、NF-02、NF-03 关闭前并不充分。至少应补充 Declaration bootstrap/前向引用顺序扰动、相同诊断起点但不同 End/Code 的排序，以及嵌套 Syntax Node 超限 Span Case。
9. **越权修改**：本 Reviewer 未修改任何 Target、Compiler Source、实现测试、Cargo Manifest/Lock、治理状态或远程状态。`07_src` 排除 `target/` 的 31 个普通文件路径排序 `shasum` 记录流摘要复算为 `5f80929f91c106ff7c0f43ce57633c78eca890e89d6eb6272e4dede7e9c20230`，与 Target 证据一致；四个 Target Cargo 文件也受 33 文件摘要保护。仓库无 Commit 且文件整体未跟踪，Target 未提供 `08_tests/` 独立起始摘要，因此对 Target 之外测试目录的“整改前后未变化”不能形成额外 Git 级证明；本轮未写入该目录。

## 6. 机械与外部核验

- 33 文件 Target：开始、写入前均与冻结摘要一致；
- 非 Target `07_src` 完整性：与 Target 提供值一致；
- JSON：两个 Schema 均可解析；本地 Ref 全部存在；Draft 2020-12 关键字结构、Regex 和条件探针通过；
- Unicode：Unicode 17.0.0 的 UAX #15 为 Revision 57，规范算法指向 Core Spec §3.11；
- crate：官方发布包流式 SHA-256 与候选值一致，包内 `UNICODE_VERSION`、API、Cargo MSRV/License、tinyvec Feature 闭包与候选记录一致；未下载到项目、未安装依赖、未生成 Lock；
- Conformance：仅审查设计充分性，未运行 Compiler 测试，未把候选 Case 冒充已通过证据。

## 7. 结论与权限边界

结论：`CHANGES_RECOMMENDED`。

建议只在 TASK-019 现有范围内关闭 NF-01～NF-04，形成新的精确 Target 后再启动全新 `INFORMAL_INDEPENDENT` 复审。本报告不得用于批准公共契约、修改依赖、恢复 TASK-018、提交、建立 Baseline、发起 Formal C04、Release 或 Formal Seal。
