# TASK-018 实施前非正式独立复审

## 1. 评审身份与结论

- Review ID：`IIR-APLS-TASK018-PREIMPL-001`
- Review Line：`INDEPENDENT_REVIEW / INFORMAL_INDEPENDENT`
- Reviewer：独立 Reviewer；未参与 TASK-018 实现，不是 C03，也不声明正式 C04 身份
- 评审目的：判断 TASK-018 的批准输入是否足够一致、完整且可机械实现
- 评审结论：`CHANGES_RECOMMENDED`
- Gate 权威：`ADVISORY_ONLY`；本报告不是正式 C04 Review Record，不产生 `PASS` 或 Gate Decision
- Target Access：`READ_ONLY`
- 唯一允许写入：本报告
- Git/Remote：未 Commit、未 Push、未建立 Baseline、未发起 Formal C04

当前批准输入尚不足以让 C03 在不发明公共语义的前提下完成 TASK-018。最先需要处理的是 Current Truth 授权冲突；随后需关闭语言接受集、类型/单位、转换语义、资源计数、Unicode NFC、Canonicalization、Provenance 和机器诊断契约中的实施缺口。

## 2. Target 完整性证据

目标集合由以下内容组成：`AI_START_HERE.md`、请求明确列出的 19 个治理/状态/设计文件，以及 `07_src/` 下 25 个非隐藏、与 Compiler/CLI 实施直接相关的普通文件；空目录占位 `.gitkeep` 不属于评审目标。

每次集合摘要的计算方式为：对 45 个文件分别计算 SHA-256，将完整的 `<sha256><两个空格><path>` 记录按 C Locale 排序，再对记录流计算 SHA-256。

| 时点 | 文件数 | Target Set SHA-256 |
|---|---:|---|
| 开始 | 45 | `343feebf2e27e9ef2e6feeaac7907a3c8098845b5b7e78d010927b7f6bdc0f61` |
| 报告写入前复核 | 45 | `343feebf2e27e9ef2e6feeaac7907a3c8098845b5b7e78d010927b7f6bdc0f61` |
| 结束 | 45 | `343feebf2e27e9ef2e6feeaac7907a3c8098845b5b7e78d010927b7f6bdc0f61` |

目标在评审期间未变化，未触发 `TARGET_CHANGED_REVIEW_INVALID`。

为解释 CNL 文档中显式继承的既有契约，另按需只读查看了 C04 Role Brief、旧 Canonical JSON 基础规则、Compiler MVP CLI/Diagnostic Envelope 和 Implementation Foundation 的相关段落；这些是上下文引用，不是本次冻结 Target，也未被修改。

## 3. Blocking Findings

### BF-01 — Current Truth 仍禁止已被其他权威记录为获批的 TASK-018 实施

- 精确位置：
  - `00_project/ai_context/CURRENT_STATE.md:12,18-20,28,33-37,57-67,76-87`
  - `00_project/ai_context/DECISION_INDEX.md:26,41`
  - `00_project/ai_context/ACTIVE_TASKS.md:22,199-201`
  - `00_project/ai_context/HUMAN_DETERMINATION_016.md:54-78`
  - `04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md:5,125-127`
- 冲突证据：`CURRENT_STATE.md` 自称项目阶段、Gate、授权和下一步的唯一权威，但仍写 `TASK018_READY_AWAITING_HUMAN_DETERMINATION`、`AUTHORIZED_UNTIL: HDP_APLS_016_DETERMINATION`、C00 Prep Profile、实施等待裁决，并明确禁止裁决前修改 Compiler Source。其他文件已记录 `HDP-APLS-016 Option A`、`DEC-020` 和 `TASK-018 IN_PROGRESS`。
- 实施影响：C03 无法从 Current Truth 得到有效的当前 Role Assignment、Authorization、Gate 和允许副作用；直接编码会违反治理合同，停下又会违反任务表的 `IN_PROGRESS` 路由。
- 建议关闭条件：由 C00/Human Project Owner 明确裁定哪一状态有效；若批准仍有效，更新 `CURRENT_STATE.md` 中项目状态、Gate、角色/Profile/Knowledge Manifest/Interaction/Authorization、允许项、阻塞项和下一步，使其精确投影 `DEC-020`；若批准无效，则回退 `DECISION_INDEX/ACTIVE_TASKS/HDP-016/WP` 的当前状态。关闭前不得开始 C03 副作用动作。

### BF-02 — Frame→IR 映射文件同时声明“当前阻塞”和“缺口已关闭”

- 精确位置：`04_design/ir/APLS_0.1_CNL_TO_IR_MAPPING.md:4,95-102`
- 冲突证据：文件状态是 `GAPS_CLOSED_BY_DEC_019_NOT_BASELINED`，第 102 行也称 GAP-CNL-IR-001～006 已闭合；但同一文件标题为“当前 Gate”的第 97～100 行仍声明 `Canonical Frame -> Verified IR: BLOCKED_BY_SCHEMA_GAPS`。
- 实施影响：工作包要求接入 CNL IR Publisher，同时禁止 C03 改变批准 IR 契约或在发现矛盾后继续。C03 无权自行把该“当前 Gate”解释成历史内容。
- 建议关闭条件：由 C02/当前事实 Owner 将该代码块明确标成历史 Gate，或更新为 DEC-019/DEC-020 后的当前 Gate；确保文件标题、状态、Gate 和工作包只有一个当前含义。

### BF-03 — Language Profile 宣称支持的状态比较句不在公开 Grammar 中

- 精确位置：
  - `04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md:200-218`，尤其第 208 行 `水泵状态等于“故障”`
  - `04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf:68-96`
  - `04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md:118-140`
- 冲突证据：Profile 将该句列为首版条件；Grammar 的 `comparison-condition` 只接受 `property-reference comparison typed-value`，而 `“故障”` 是术语/状态引用引号，不是使用 `『...』` 的 `text-value`。公开状态条件只有 `entity-reference 处于 state-reference 状态`。
- 实施影响：C03 必须在“按 Grammar 拒绝”与“按 Profile 接受并发明 State-as-Property 映射”之间选择，直接改变公开 Source 接受集和 Frame/IR。
- 建议关闭条件：由 C02/产品语义 Owner 明确选择并同步三份契约：若是示例错误，将其改为受支持的 `水泵处于“故障”状态`（或等价合法形式）；若确需状态比较句，则新增精确 Grammar、Frame、IR、诊断和 Conformance 规则并重新批准。

### BF-04 — Property/Comparison 的类型、单位和运算符兼容矩阵未冻结

- 精确位置：
  - `04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf:34-37,73-96,177-179`
  - `04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md:116-148,163-176,211-218,286-298`
  - `04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md:62-72,118-126`
  - `04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md:161-200,256-267`
  - `04_design/ir/apls-cnl-ir-0.1.schema.json:221-237,330-367`
- 缺口证据：Grammar 允许六种比较符与所有 `typed-value` 组合，Property 对所有 `value_type` 都语法上允许可选单位；Frame 和 IR 仅说“类型/Unit/Literal 兼容”，没有给出封闭矩阵。未定义的行为包括：Boolean/Text 是否仅允许 EQ/NE、Integer 是否接受 Decimal、Boolean/Text 是否禁止单位、Percentage/Duration 的 `unit_ref` 何时为 null 或内建单位、Integer/Decimal 与名义单位如何匹配、温度量如何与无显式默认单位的属性匹配。
- 实施影响：这些规则决定候选能否被淘汰，因而可改变最终 `0/1/>1` 类别数；任意实现选择都可能成为“偷偷选候选”的语义漏洞，并改变 Source 接受集、诊断和 Semantic Hash。
- 建议关闭条件：批准一张完整、封闭、可机械执行的 `Property.value_type × property.unit_ref × comparison.operator × TypedLiteral.kind/dimension/unit` 矩阵，包含允许的精确转换、禁止项、错误码和 Canonical Value 规则；为每一格提供至少一个正/反例。

### BF-05 — Transition 的“可达性与冲突”是公共必需语义，但没有算法和诊断闭环

- 精确位置：
  - `04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md:246-255`
  - `04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md:178-187`
  - `04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md:229,256-272`
  - `04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md:35-44`
- 缺口证据：Profile 明确要求 Semantic Checker 验证 Transition 可达性与冲突；Frame 只冻结 trigger/entity/source/target，IR Validator 只要求两个状态同属一个实体且不同。没有定义可达性图、触发条件重叠、同源多目标冲突、事件/状态/比较合取的可满足性边界、跨 Transition 检查顺序或适用 CNL 诊断 Code。
- 实施影响：C03 要么漏掉 Profile 明确要求的语义检查，要么自行发明约束求交/可达算法和公共错误行为；两者都不满足工作包“不改变公共契约”。
- 建议关闭条件：冻结 0.1 Transition reachability/conflict 的精确判定规则、算法能力边界、失败关闭规则、诊断 Code/Span/Related、资源计数和最小 Conformance Case；若 0.1 不做该检查，则正式修改 Profile 并由正确 Owner 批准。

### BF-06 — 候选资源数值已冻结，但部分计数对象仍依赖实现粒度

- 精确位置：
  - `04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md:63-77`
  - `04_design/compiler/APLS_0.1_CNL_FRONTEND_ADAPTATION_DESIGN.md:47-53,68-78`
  - `00_project/ai_context/HUMAN_DETERMINATION_016.md:9-10,20-21,29-32`
- 缺口证据：
  - `Parse Candidate AST Node` 以“每构造一个有类型 AST Node”计数，但 CNL Candidate AST 的节点种类和语法构造到节点的映射尚未冻结，C03 可通过 AST 粒度改变是否超限；
  - `Enumerated Candidate Token Occurrence` 说“每向一次 Parser 输入”，未明确失败 Parse 是按完整 Stream 长度计数还是按 Parser 实际拉取到的前缀计数；
  - Lattice Edge 的 `Symbol Candidate` 对固定关键词/标点/数值等非符号 Token 的规范值未定义；
  - 未明确声明句是否进入 Lattice/Stream/AST Document 计数，以及跨句和跨候选的精确枚举排序键。
- 实施影响：相同 Source 在两个合规实现中可一边成功、一边 `APLS-T0007`；“首次超限”的 Span/资源也可能变化。数值虽可测试，验收边界本身不唯一，且 C03 可无意中借实现结构扩大/缩小接受集。
- 建议关闭条件：冻结 CNL AST Node taxonomy 与逐 Production 计数表；规定所有 Stream 在调用 Parser 前或后的统一 Token 计数口径；定义非符号 Edge 的 sentinel/identity、声明句是否计数、句子/Edge/Stream 的完整稳定排序键，以及 T0007 的 `resource/limit/observed/span` 规则。边界与首次超限用黑盒 fixture 可复现后再实施。

### BF-07 — 完整 Unicode NFC 要求与当前依赖/版本闭包不成立

- 精确位置：
  - `04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md:30-35`
  - `04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf:155-165`
  - `04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md:35-36,51,55-60`
  - `00_project/ai_context/HUMAN_DETERMINATION_016.md:20-21,31-32,73-76`
  - `07_src/Cargo.toml:15-25`
- 缺口证据：Profile 要求对完整 Source 执行 NFC，并要求 Compiler Manifest 精确声明 Unicode 数据版本，但批准输入没有冻结具体 Unicode 版本，也没有定义 Manifest 字段/位置。Grammar 的说明/文本允许广泛 Unicode Scalar，因此不能只检查 ASCII/汉字子集。锁定闭包没有直接或传递的 `unicode-normalization` 包，Rust 标准库也不提供完整 Unicode NFC 分解/组合检查；工作包同时禁止新增依赖和改变 Cargo Lock。静态检查确认 `Cargo.lock` 与各 Cargo Manifest 中均无 `unicode-normalization`。
- 实施影响：C03 不能在现有闭包下实现可审计的完整 NFC；自行嵌入一套未批准 Unicode 表会发明语言接受版本和供应来源，自行缩小检查集会误接收非 NFC Source。
- 建议关闭条件：由正确 Owner 冻结 Unicode 数据版本和 Compiler Manifest 的规范字段；随后二选一并明确授权：增加精确锁定、MSRV/许可审查通过的 NFC 依赖，或提供受控、版本化、带来源与生成验证的 vendored Unicode normalization data/algorithm。补充组合字符、Hangul、非 BMP、Quick_Check 边界的 Conformance Case。

### BF-08 — 合取去重可产生 Schema 无法表示的单项 Conjunction

- 精确位置：
  - `04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf:68-71`
  - `04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md:142-146`
  - `04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md:118-123`
  - `04_design/ir/apls-cnl-ir-0.1.schema.json:395-413`
- 冲突证据：Grammar 接受 `A并且A`；Frame 只要求表面 `items` 至少两个，没有禁止重复；Canonicalization 要求 `Conjunction.items` 去重；Schema 又要求 Conjunction `minItems: 2` 且 `uniqueItems: true`。全部重复的合取去重后只剩一个 Atom，无法作为 Conjunction 通过 Schema。
- 实施影响：C03 必须自行选择“折叠为 AtomicCondition”“拒绝重复条件”或“保留重复”，每种选择都会改变 Frame Kind、匿名节点 ID、Semantic Hash、Provenance 或 Source 接受集。
- 建议关闭条件：由语义 Owner 明确并同步 Grammar/Frame/IR/Schema：推荐冻结 `dedup 后 1 项 -> AtomicCondition` 的机械归一和 Provenance 并集规则；若选择拒绝，则增加明确诊断和反例；不得由 C03 私下决定。

### BF-09 — Built-in Unit 的 IR 包含集合与 Source Map/Span 复验不闭合

- 精确位置：
  - `04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md:54-85,112-159,161-188,217-270`
  - `04_design/ir/apls-cnl-ir-0.1.schema.json:27-39,106-176,187-237,330-340`
- 缺口证据：IR 定义五个内建 Unit ID，语义 Hash 包含完整 `units` 数组，并要求每个最终节点有且只有一条 Source Map Entry；但未规定 `units` 是始终包含全部内建单位、只包含被引用单位，还是包含被声明/引用闭包。不同选择产生不同 IR Byte 和 Semantic Hash。若始终注入未在 Source 出现的内建单位，又无法满足 `provenance` 至少一项的来源要求。Schema 的 Span 只约束非负数，跨节点验证清单未要求 `start <= end`、UTF-8 边界、落在 Source Byte 内、Role Span 包含于 Sentence Span、`source_id` 与唯一 Manifest Entry 一致。
- 实施影响：两个实现可对同一 Source 产生不同 Semantic Hash；Malformed 或伪造 Provenance 仍可能被标记 `verified`，破坏重复节点 Provenance 并集和可审计性。
- 建议关闭条件：冻结内建单位 materialization closure 与排序规则、Property `unit_ref` 生成规则、内建单位的 Source Map 策略；补充完整 Span/Source Map 跨节点不变量及稳定角色名集合，并用“未使用/仅类型隐含/显式值引用/自定义单位/重复匿名节点”五类样例证明字节和 Provenance 唯一。

### BF-10 — 零候选诊断选择、E1310 Witness 和 T0007 Payload 尚无机器契约

- 精确位置：
  - `04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md:11-34,72-89,116-129`
  - `04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md:40-44,65-77,95-102`
  - `07_src/crates/apls-compiler/src/diagnostic.rs:1-17`
- 缺口证据：诊断目录要求“按稳定根因规则”从全部失败候选提升公共 Source Error，但没有给出该规则或说明是输出全部不可支配根因还是选一个；E1310 要求 `outcome=AMBIGUOUS`、至少两个 Canonical Frame 分歧见证和不同角色，却没有冻结字段名、类型、规范投影、排序与内部 Frame 信息暴露边界；T0007 没有冻结具体资源名、limit、observed、计数阶段和 Primary Span 的 Payload。当前公共 Rust `Diagnostic` 仅有 `code/severity/message`，因此没有可沿用的已实现结构能消除歧义。
- 实施影响：候选失败路径的遍历顺序可能改变公共诊断；C03 必须发明机器 JSON 字段，或者无法满足 E1310/T0007 的可测要求。错误分类的 Code/Exit 已分离，但 Payload 和根因集合不能做跨实现黑盒一致性验证。
- 建议关闭条件：冻结 CNL Diagnostic/Tool Diagnostic 的完整 JSON Schema 或等价封闭类型；定义零候选根因聚合与抑制算法、E1310 witness 的最小规范结构/排序/截断规则、T0007 资源 Payload 和无 Source Span 时的规则；再给出遍历顺序扰动不改变输出的 Conformance Case。

## 4. Non-blocking Observations

### NO-01 — 本次 Target 不是正式 C04 可用的不可变 Git Target

仓库尚无 Commit，全部项目文件处于未跟踪状态。开始/结束哈希足以保护本次 Advisory 评审的一致性，但不能替代正式 C04 所需的可检索、可复现精确 Commit。TASK-018 输出进入正式评审前仍需建立受权的不可变 Target。

### NO-02 — CNL Canonicalization 对旧 IR 基础规则的引用可解析，但自包含性较弱

`APLS_0.1_CNL_CANONICAL_IR.md:114` 说“继承旧 IR 的基础 JSON 规则”，未给出精确文件/章节 ID。本次按上下文查到旧设计第 23～35 行的转义和十进制规则，因而当前不是实现阻断；建议未来改为精确引用并明确只继承 byte-level rules，避免旧 DSL 的“禁止匿名节点”等不兼容语义被误继承。

### NO-03 — 现有代码是大型旧 DSL 原型，迁移量显著但边界诚实

`07_src/README.md`、`BASELINE_INDEX.md` 和工作包一致标记旧 DSL 为 Legacy；除 `--version` 外，CLI 当前均以 `APLS-T0006`/Exit 3 失败关闭。代码没有冒充 CNL 已实现。风险在于替换唯一 `.lalrpop` Grammar、隔离旧 Source Graph/Resolver 和扩展完整 Diagnostic/VerifiedArtifact 路径的改动面较大，需严格按工作包七步执行。

## 5. Pass 项

1. **DEC-017 收敛原则一致**：Language Profile、Grammar 注释、Frame、Ambiguity Boundary、Frontend Design、Diagnostics 和 Work Package 均一致规定中间多候选允许、最终按 Canonical Frame `0/1/>1` 判定，并明确禁止最长匹配、概率、LLM、排名、声明/遍历顺序和第一个成功路径。
2. **高层错误分类正确**：完整分析后 `>=2` 为 `APLS-E1310`/Source Error；无法完成枚举或验证为 `APLS-T0007`/Exit 2；两者在设计、工作包和现有 `CompileOutcome` Exit 映射中没有混用。
3. **Sentence→Frame→IR 种类覆盖完整**：Grammar 的七类声明、Rule、Transition、Invariant、Acceptance 和 Informative Sentence 均有 Frame Kind 和 IR 容器；未发现整个公开 Production 被直接丢弃。BF-03/BF-04/BF-05/BF-08 是接受/归一细节缺口，不是否认种类覆盖。
4. **Semantic ID 与 Hash 主干明确**：声明 ID 构造、匿名节点 kind-separated SHA-256 Preimage、文档语义投影、Provenance 排除、名义自定义单位、重复匿名节点合并和稳定 Provenance 并集的主规则可机械实现；BF-08/BF-09 是剩余闭合缺口。
5. **Schema 基础封闭性通过**：CNL Schema 是合法 JSON，全部本地 `#/$defs/...` 引用均能解析到已定义 `$defs`；顶层和各节点使用 `additionalProperties: false`，必需字段、封闭枚举和单 Entry `minItems=maxItems=1` 已表达。
6. **单 Entry 与 Legacy 隔离方向一致**：Grammar 无 Import，Schema Source Manifest 恰好一个文件，HDP-015/016、IR Design 和 Work Package 均禁止多文件 CNL、旧 IR 复用和旧 DSL 第二公开入口。
7. **CLI 成功产物边界一致**：既有批准契约与工作包都要求 `parse/check` 不输出 AST/Frame、`diagnose --through emit` 不发布 IR、只有重新解析 + Schema + 跨节点复验后的 `VerifiedArtifact` 可发布；文件失败保持旧目标，stdout 只有 Exit 0 且完整可复验时才构成交付。
8. **现有依赖对其余核心能力足够**：`serde/serde_json/jsonschema/sha2/num-bigint/num-traits/tempfile/LALRPOP` 可覆盖结构序列化、Draft 2020-12 Schema、SHA-256、精确整数/十进制辅助、原子文件交付和 LR Parser；唯一明确的闭包例外是 BF-07 的完整 Unicode NFC。

## 6. 建议的最小关闭顺序

1. 先裁定并修复 BF-01；在 Current Truth 未统一前不启动 C03。
2. 由 C02/语义 Owner 一次关闭 BF-02～BF-05 与 BF-08，形成唯一的新批准语言/Frame/IR Target。
3. 由架构/实现基础 Owner 关闭 BF-06、BF-07、BF-09、BF-10；若需要新增依赖或改变已批准资源/公共 Schema，必须重新走正确的 Human Determination/授权。
4. 用不改变公共契约的最小 Conformance fixtures 证明：支持句式覆盖、等价多候选收敛、最终多解 E1310、零候选稳定诊断、每项资源边界/T0007、NFC、IR Hash/Provenance、Legacy 隔离和 CLI 交付。
5. 形成新的精确 Target 后再做全新独立复审；本报告不能升级为正式 C04 `PASS`。

