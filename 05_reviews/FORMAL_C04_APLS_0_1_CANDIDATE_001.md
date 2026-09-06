# FORMAL C04 — APLS 0.1 实现候选首次正式独立评审

```yaml
review_id: FORMAL_C04_APLS_0_1_CANDIDATE_001
review_line: FORMAL_C04
reason_code: FORMAL_C04_INITIAL
review_purpose: APLS_0_1_PRODUCT_BASELINE_READINESS
review_scope: FULL_SCOPE
physical_session_id: APLS-C04-Independent-Review-v02
review_target_commit: 3289037bee1aab64dfa2d58188379a68dcfa601e
review_target_tree: 51edf42da73237bcb3408e234d2cbb2095758fa1
review_target_file_count: 249
review_date: 2026-09-05
gate_decision: CHANGES_REQUESTED
```

## 1. Dispatch 凭据摘要

- `HDP-APLS-022`（status=APPROVED，selected_option=A，`DEC-028`）：授权以精确 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e` 为唯一只读 Review Target，由全新独立 Session 执行 `FULL_SCOPE` 首次正式 C04，仅写入本文件。
- 首次 Dispatch `NISR-APLS-C04-022-001` 已消费且被平台中止，无 Review Record、无 Gate Decision、Target 完好；`DEC-029` 授权由项目负责人手动新建干净会话执行，Target、`FULL_SCOPE`、只读与唯一写入边界不变。
- 本 Session 即 `DEC-029` 授权的手动干净会话。

## 2. 独立性证据块（formal_c04_independence_evidence）

```yaml
review_id: FORMAL_C04_APLS_0_1_CANDIDATE_001
reviewer_role: C04
new_session_id: APLS-C04-Independent-Review-v02
new_session_evidence: >-
  项目负责人按 DEC-029 手动新建的干净会话；本会话未读取任何实现/整改 Session 的
  HANDOFF 或私有推理，全部评审事实从精确 Commit 副本与主仓库正式治理文件重建。
execution_or_remediation_session_ids_excluded:
  - NISR-APLS-C04-022-001   # 首次 Dispatch，已中断，无产出
  - 全部 C03 实现 / C05 验证 / 非正式复审 Session（TASK-018/019/020/021 各轮）
private_context_inherited: false
context_package_manifest:
  - 主仓库：AI_START_HERE.md、00_project/ai_context/HUMAN_DETERMINATION_022.md、
    00_project/ai_context/DECISION_INDEX.md（DEC-028/DEC-029）
  - Target 副本（/private/tmp/apls-c04-review-v2，checkout 自精确 Commit）：
    PRD、DECISION_INDEX、03_architecture、04_design 全套语言/诊断/IR/编译器契约、
    07_src 全部源码与内嵌测试、06_test_design、09_quality、11_validation、05_reviews 既往非正式记录（仅作对照）
exact_immutable_target: 3289037bee1aab64dfa2d58188379a68dcfa601e
target_binding_evidence: 见第 9 节三个时点的 cat-file/rev-parse/ls-tree 核验输出
review_target_access: READ_ONLY
allowed_write_paths:
  - /Users/luchuang/Documents/ai编程语言/05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md  # 本文件
  - /private/tmp/apls-c04-review-v2  # Target 只读审查副本及构建输出目录
git_write_allowed: false
remote_mutation_allowed: false
pre_review_target_status: UNCHANGED（时点 1/2 核验一致，见第 9 节）
post_review_target_status: UNCHANGED（时点 3 核验结果见第 9 节，于本文件写入后立即执行并回填）
review_record: 05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md
```

注：中断残留物 `/private/tmp/apls-c04-review-3289037` 未读取、未使用，不作为任何证据。

## 3. Review Readiness（AI_ENGINEERING_RULES_V2.md §38.7.1 + HDP-APLS-022 §2）

```text
REVIEW_ID: FORMAL_C04_APLS_0_1_CANDIDATE_001
REVIEW_TARGET: APLS 0.1 实现候选（本地初始 Commit）
EXACT_GIT_COMMIT_OR_CONTROLLED_VERSION: 3289037bee1aab64dfa2d58188379a68dcfa601e
TARGET_FROZEN: YES（本地不可变 Commit；克隆副本 checkout 后 HEAD/Tree 复核一致）
INDEPENDENT_REVIEW_SESSION: APLS-C04-Independent-Review-v02（DEC-029 手动干净会话）
FORMAL_REVIEW_RECORD_LOCATION_DEFINED: YES
FORMAL_REVIEW_RECORD: 05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md
REVIEW_READINESS: READY
```

逐项核验：完整不可变 Commit 存在且可读取、可复现（克隆 + checkout 成功）；Tree `51edf42da73237bcb3408e234d2cbb2095758fa1` 与文件数 249 与 HDP-022 §2 一致；适用 Baseline 为 `APLS_BASELINE_NOT_YET_ESTABLISHED_CANDIDATE_PREPARATION`；独立 Session 已建立；Review Record 位置预定义；未发现权威 Current Truth 来源之间导致无法确定判定标准的实质冲突（文档头状态过期问题见 F-09，权威决定以 DECISION_INDEX 为准，不构成判定标准不明）。

## 4. 范围与证据清单（FULL_SCOPE）

按优先顺序独立审查：

1. 已批准需求：`01_product_requirements/PRD.md`（PRD-001～009、非功能要求、关键约束）；
2. 当前有效决定：`00_project/ai_context/DECISION_INDEX.md`（DEC-001～026，含 DEC-017 唯一性层级、DEC-019 IR 身份、DEC-020 资源边界、DEC-023/024 契约与依赖采用）；
3. 架构：`03_architecture/APLS_0.1_CONTROLLED_NATURAL_LANGUAGE_ARCHITECTURE.md`、`system_architecture.md`（含 §5.5/§11 需求覆盖）、ADR-APLS-001；
4. 设计与语言契约：`04_design/language/` 全部（ZH_CN Language Profile、ZH_CN_GRAMMAR.ebnf、SEMANTIC_FRAME_PROFILE、CNL_AMBIGUITY_BOUNDARY、CNL_SEMANTIC_VALIDATION_PROFILE 及旧 DSL 三份标记文件）、`04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md` 与 `apls-cnl-diagnostic-0.1.schema.json`、`04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md` 与 `apls-cnl-ir-0.1.schema.json`、`04_design/compiler/` 全部（WORK_PACKAGE、RESOURCE_AND_ORDERING、UNICODE_NFC、FRONTEND_ADAPTATION、COMPILER_MVP、IMPLEMENTATION_FOUNDATION、TASK019 整改记录）；
5. 实际代码：`07_src/crates/apls-compiler/`（cnl_lexer.rs 760 行、cnl_pipeline.rs 4214 行、cnl_ast.rs、diagnostic.rs、resource.rs、limits.rs、pipeline.rs、validate.rs、build.rs、apls_grammar.lalrpop 全部逐行阅读）与 `07_src/crates/apls-cli/`（lib.rs、main.rs）、Cargo.toml/Cargo.lock；
6. 实际测试：07_src 内 `#[cfg(test)]` 全部测试（Compiler 24 + 1 ignored、CLI 7 + 1 ignored）逐条阅读；`08_tests/` 确认仅为模板 README；
7. 测试设计与验证证据：`06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md`（CNL-C001～C016）、`APLS_0.1_CNL_IMPLEMENTATION_VERIFICATION_PLAN.md`、`11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md` 与 `TASK020_C05_CNL_VERIFICATION.md`；
8. 既往非正式复审记录（05_reviews/ 各 IIR）：仅在形成独立判断后对照，未继承其结论；IO-01～IO-07、BF-02～BF-10、NF-01～NF-05 的关闭状态与 CURRENT_STATE 一致。

## 5. 机械验证命令与结果

| 命令 | 执行位置 | 结果 |
|---|---|---|
| `python3 09_quality/traceability/validate_traceability.py` | Target 副本 | **FAIL（exit 1）**：Expected IDs=3（模板占位 SYS-001/NFR-001/IF-001），Covered=0，Missing=3，Matrix/Metadata Edges 均为 0 |
| `cargo test --locked --offline` | Target 副本 `07_src/` | **无法执行（证据受限项 E-01）**：本机无 Rust 工具链（`cargo`/`rustup` 不存在，`~/.cargo`、`~/.rustup` 均不存在）。按 Dispatch 规则记录为证据受限，未修改 Lock、未联网新增依赖、未安装工具链 |

证据受限项：

- E-01：Compiler 24 项 / CLI 7 项普通测试、T0 资源边界 ignored 用例均未由本评审独立重跑；`VAL-APLS-C05-020 = PASS` 作为既有证据审阅但不继承（C05 PASS ≠ C04 PASS）。
- E-02：LALRPOP 零冲突构建门禁、MSRV 1.86 锁定离线构建未独立复验（同因）。
- E-03：Unicode 17.0.0 官方 `NormalizationTest.txt` 一致性用例未重跑（评审禁止联网下载；代码级审查确认 `is_nfc` 全量判定、`UNICODE_VERSION == (17,0,0)` 运行时断言与 E1004 原始 Byte Span 逻辑存在）。

追溯门禁适用性判定（对 FAIL 结果的评审裁定）：`PROJECT_ASSURANCE_CADENCE_POLICY.md` 第 4 节将 Traceability Gate 绑定于“建立或实质修改正式需求 Baseline / 追溯关系时”；`09_quality/traceability/README.md` 明确模板初始 FAIL 不是启动阻断，该门在“准备建立或复审正式需求 Baseline”时必须通过。本次 C04 的授权边界（DEC-028）明确不建立 Baseline，故该门对本 Gate 不直接构成机械阻断；但评审目的即产品 Baseline 就绪度，该 FAIL 与 02_system_requirements 全模板状态作为实质就绪缺口记录为 F-08。

## 6. 评审中确认无误的关键机制（正面核验记录）

- 词法 Lattice 对每个 Byte 位置生成全部 FIXED/引用/数值候选 Edge，DFS 完整枚举全部 Complete Token Stream；未发现最长匹配、概率、LLM、候选排名或“第一个成功 Parse”路径；
- 收敛 Gate 按 `canonical_bytes(frame.value)` 去重，0/1/>1 分类正确，>1 时只输出一条 E1310 且取 Payload Byte 序前两类见证；等价候选 Provenance 取稳定并集（IO-03 已修复属实）；
- `process_candidate` 按 bind→type→normalize 阶段顺序确定性淘汰候选，阶段失败不构造下游 Artifact；
- Transition 文档级 E1402（结构可达）与 E1403（同实体/源状态/同 Canonical Trigger 多目标）均已实现并有定向测试；
- IR 发布前对实际输出 Byte 重新解析 + JSON Schema + 跨节点复验（IO-04 已修复属实），`units[]` 物化闭包、Source Map 覆盖、Span/Role/Pointer、匿名节点 ID 与 `semantic_hash` 均被复验；
- CLI 退出码 0/1/2/3、`--version` 固定行、emit 前 Source 复读、原子发布、符号链接/路径穿越拒绝与批准契约一致；
- 资源 Ledger 登记函数本身符合 DES-APLS-CNL-RESOURCE-001（首次超限 T0007、`observed=limit+1`、正确 Stage/Span/Payload），三类公开可达资源的 CLI 黑盒边界用例存在（ignored），四类受支配资源用私有 `#[cfg(test)]` 注入且未泄漏到公共 API；
- 公共诊断总排序键按 DES-APLS-CNL-DIAG-001 §1.2 实现，第 1001 条截断发生在总排序之后；
- 诊断 Envelope 与 E1310/T0007 Payload 通过嵌入 Schema 复验。

## 7. Finding 列表

> 任何 Open Finding 阻断 PASS。Severity 定义见 AI_ENGINEERING_RULES_V2.md §38.7.2。

### F-01（S1）合法 Source 可因文本内容含 `unit:%` 子串被误判为 Compiler Internal Failure

- 问题：`cnl_pipeline.rs` `build_ir` 用 `canonical_bytes(v).windows(6).any(|w| w == b"unit:%")` 扫描全部 Frame 序列化字节来判定是否注入内建 `unit:%`（`needs_percent`）。任意合法文本（如 `说明：『unit:% 是内建百分比单位』。` 或文本比较右值 `『…unit:%…』`）都会命中该子串，导致注入未被任何 Property/Quantity/Deadline 引用的 `unit:%`；随后 `cross_validate` 的 Unit Materialization 闭包检查（`materialized_units != unit_ids`）必然失败，整个编译事务以 `APLS-T0006` / Exit 3 结束。
- 重要性：S1 — 合法输入必须被接受并得到唯一规范化解释（PRD-001/PRD-003 及首要成功标准）；此处合法 Source 被失败关闭拒绝，且把 Source 诱发的结果误分类为 Internal Failure（T0006 语义漂移）。同一 Source `apls check` 通过（Exit 0）而 `apls emit-ir` 失败（Exit 3），行为自相矛盾。
- 影响：特定合法文档无法产出 Verified IR；Internal Failure 误报会错误指向 Compiler 缺陷而非可修复原因；`needs_time/needs_temp` 使用精确字段匹配，唯独 `%` 用子串扫描，属于实现捷径。
- 关闭条件：`needs_percent` 改为与其他两个内建单位一致的精确语义判定（Property `unit_ref` 与 Quantity `canonical_unit_ref` 精确匹配，不做字节子串扫描）；新增“文本/说明含 `unit:%` 子串的合法 Source 在 check 与 emit-ir 下均接受且 `units[]` 闭包精确”的定向测试；由新独立 C04 针对新精确 Target 复核。

### F-02（S1）PRD-005 规则冲突无检测：同条件下 `必须` 与 `禁止` 可共存进入 Verified IR

- 问题：PRD-005 要求“规则冲突均有稳定诊断”，且“尚未定义冲突解决规则时，必须拒绝而不是选择任意结果”；已批准架构 `system_architecture.md` §5.5 明确“多条 Rule 对同一目标产生冲突动作时必须显式组合，否则报错”“Safety 与普通 Rule 冲突时不能依赖声明顺序解决”。当前设计与实现只定义并检查 Transition 冲突（E1402/E1403）；对同一实体、相同 Canonical Condition 下针对同一动作的 `必须`（REQUIRE）与 `禁止/不得`（PROHIBIT）两条 Rule 既未定义冲突规则，也不拒绝——两句各自收敛、生成不同匿名 ID 并同时进入 `rules[]` 与 Verified IR。Invariant 与普通 Rule 的冲突同样无任何处理。
- 重要性：S1 — 已批准 P0 需求（PRD-005）验收标准未满足，且无 Descope 决定（DECISION_INDEX 中无相关记录）；矛盾规范事实被静默标记为 `verified` 交付 Agent 消费，直接违背“拒绝歧义/冲突而非任意继续”的产品核心立场。
- 影响：相互矛盾的规则进入唯一机器契约；下游 Agent 无法得知冲突存在；后续补定义冲突语义将改变接受集，构成兼容性问题。
- 关闭条件：由有权层级（C01/C02，必要时 HDP）明确定义 0.1 规则/Safety 冲突语义（至少覆盖相同 Canonical Condition + 相同 Action/Target 的 REQUIRE×PROHIBIT 直接冲突），或在当前语言/语义 Profile 中冻结“拒绝”行为与稳定诊断代码；实现、诊断目录与 Conformance 同步；新独立 C04 复核。

### F-03（S2）Bind/Type/Normalize 阶段的 APLS-T0007 被当作候选淘汰处理，可被兄弟候选掩盖

- 问题：`cnl_pipeline.rs` 行为句候选循环中 `Err(error) => failures.push(error)` 把 `process_candidate` 返回的所有错误——包括三个 Frame 候选资源计数器超限产生的 `APLS-T0007`——与普通候选确定性淘汰同等处理。若另一候选收敛成功，该 T0007 被静默丢弃并正常产出 Frame 与 Verified IR。这违反 `DES-APLS-CNL-RESOURCE-001` §7“资源耗尽立即终止当前编译事务，不能输出 E1310、Frame、部分 IR 或旧 Artifact 替代品”及 `WP-APLS-CNL-C03-001` §3“首次试图超过时返回 APLS-T0007 / Exit 2，且不得产生 Frame 或 IR”。
- 重要性：S2 — 当前 Grammar 下 `bound/typed/canonical_frame_candidates` 受 `candidate_token_occurrences` 支配（同上限 1,000,000 且 Token 登记先于 Parse/Bind/Type/Normalize），公开 Source 不可达，属防御路径合同违规而非现行可触发缺陷；但一旦支配不变量被未来 Grammar/实现改动破坏，该路径将直接使“无法完整处理全部候选”的 Source 被接受，破坏首要质量目标的保护栏。
- 影响：防御性资源检查的最后一层语义不正确；测试注入若经由 `process_candidate` 也会观察到错误行为。
- 关闭条件：候选循环区分 Source 候选淘汰与 Tool Failure（任一 T0006/T0007 立即终止整个编译事务，不进入 `failures` 聚合）；补充对应的模块级注入测试证明 T0007 不被掩盖；新独立 C04 复核。

### F-04（S2）零候选公共诊断未按封闭根因聚合算法实现，使用硬编码词表启发式

- 问题：`DES-APLS-CNL-DIAG-001` §1.1 冻结了零有效候选时的根因聚合（按候选路径首个失败 Stage 的 Terminal Finding 取并集、Canonical JSON 去重、封闭抑制表、唯一总排序）。实现中“无任何完整 Parse”的路径（`diagnose_unparsed`）不使用任何候选失败证据，而是对句子文本做词表子串扫描：`E1303` 仅对硬编码的 `["温度","速度"]` 两词触发（恰为设计样例用词），`E1301` 对 `高/低` 等子串触发（可命中已声明术语名内部，如 `“最高水位”`），`E1302/E1304/E1306/E1307` 同样为词表匹配。目录中的 `E1105/E1106/E1203/E1205/E1305/E1308/E1309` 在生产代码中从不产生；`suppress_derived_diagnostics` 的抑制规则与封闭表不一致（实现中任意非 E1101/E1308 的具体诊断抑制同 Span 的 E1308，而合同只允许 E1401 抑制 E1308；E1203 抑制链因 E1203 不存在而缺失）。
- 重要性：S2 — 对样例之外的输入，公共诊断的根因代码与最小 Span 不可依赖，PRD-006“可操作诊断”与诊断目录的根因身份合同只对已见样例成立；不产生错误接受，属于诊断正确性/可测试性实质缺陷。
- 影响：用户/Agent 依赖根因代码修复 Source 时会被误导；Conformance 对未覆盖类别无保护。
- 关闭条件：实现 §1.1 的封闭根因聚合（各候选路径的首个失败 Stage Terminal Finding 并集、去重、封闭抑制、总排序），或以有权决定修订诊断目录使其与实现能力一致；为 E1305/E1309 等目录根因补负向样例或删除/重定义代码；新独立 C04 复核。

### F-05（S2）声明歧义路径的 E1310 Payload 不符合冻结机器契约

- 问题：声明句多候选路径调用 `ambiguity()`：Witness 的 `semantic_fingerprint`/`value_fingerprint` 以 Rust `Debug` 格式串（`format!("{v:?}")`，含 Span、rustc 实现相关、非 Canonical Payload Byte）为 SHA-256 Preimage，违反 `DES-APLS-CNL-DIAG-001` §5 冻结的 Fingerprint Preimage（`APLS-CNL-FRAME-WITNESS-0.1` + Canonical Frame Payload Bytes）；`frame_kind` 对所有声明类别硬编码为 `entity_declaration`（Property/State 等声明歧义时报告错误 Kind）；`differing_roles` 硬编码为 `["meaning"]`，不是规范 Role 名集合中的角色；Primary Span 为整句而非“最小最终语义分歧范围”。
- 重要性：S2 — 该路径在观察到的 Grammar 下难以由公开 Source 触发（声明句 Tokenization 实际唯一），属防御路径；但一旦触发，公共机器诊断的内容不可跨实现复现、字段含义错误，违反已冻结诊断机器契约。
- 影响：E1310 机器 Payload 的可复现性与字段真实性仅在 Rule/Transition/Invariant/Acceptance 路径成立。
- 关闭条件：声明歧义 Witness 改按冻结 Preimage 对规范 Canonical Payload 计算指纹，`frame_kind` 取实际声明类别，`differing_roles` 取真实分歧规范角色，Primary Span 取最小分歧范围；或将该路径的根因与 Payload 合同显式冻结为另一受批准形态；新独立 C04 复核。

### F-06（S2）PRD-007 验收未满足：IR Schema 不能机械区分 unknown 与 open

- 问题：PRD-007 验收要求“IR Schema 能机械区分 normative、informative、unknown 和 open”。`apls-cnl-ir-0.1.schema.json` 仅有 `informative_items[].classification = "informative"` 与隐含的规范节点分区，不存在 `unknown`/`open` 类别或等价机械标记；未发现将该两类移出 0.1 IR 的 Descope 决定。
- 重要性：S2 — P0 需求验收标准字面未满足；失败关闭设计缓解了实际风险（未知/未决内容无法进入 Verified IR），但“机械区分四类”的机器合同不成立，Agent 消费边界文档化不足。
- 影响：面向 Agent 的输出合同不完整；Baseline Adoption 时该验收项无法通过。
- 关闭条件：由 C01/C02 裁定——在 Schema/契约中补充 unknown/open 的机械表达，或正式修订 PRD-007 验收措辞使其与“失败关闭下不可表示”的设计一致；新独立 C04 复核。

### F-07（S3）Transition 源状态等于目标状态复用 APLS-E1403，根因身份漂移

- 问题：`type_candidate`/`normalize_candidate` 对 `source_state == target_state` 发出 `APLS-E1403`；目录中 E1403 的根因身份是“相同实体、源状态与规范 Trigger 要求进入多个不同目标状态”（文档级冲突），而“两个状态必须彼此不同”是另一条 Frame 契约（`DES-APLS-CNL-FRAME-001` §5.3），目录中没有对应该根因的代码。诊断目录明确“Code 的根因身份在 0.1 内不得漂移”。
- 重要性：S3 — 失败关闭方向正确，仅根因代码归属错误，范围有限。
- 影响：机器消费者按 E1403 根因定位冲突组时会误判；目录覆盖完整性受损。
- 关闭条件：为该根因分配/使用目录中正确的稳定代码（或经有权决定扩展目录），并使复验路径（`cross_validate` 的 T0006 内部检查除外）与诊断目录一致；新独立 C04 复核。

### F-08（S2）正式需求追溯层缺失：追溯机械门禁 FAIL，02_system_requirements 全为模板占位

- 问题：`validate_traceability.py` 结果 FAIL（Missing：SYS-001/NFR-001/IF-001；Matrix/Metadata Edges 均为 0）。实质是项目尚无任何正式 SYS/NFR/IF 系统需求：`02_system_requirements/` 四个文件均为模板占位（含 `<SOURCE_ID>`），`01_product_requirements/acceptance_criteria.md` 仅有一行空 DRAFT（AC-001），PRD-001～009 没有任何正式下游追溯边。
- 重要性：S2 — 对追溯性与受控交付有实质影响。适用性判定见第 5 节：本 C04 不建立/修改需求 Baseline，该门对本 Gate 不构成独立机械阻断；但本评审目的即产品 Baseline 就绪度，该缺口在 Baseline Adoption 前必须关闭（届时该门不可关闭且无已批准 Exception）。
- 影响：需求→系统需求→架构/代码/测试的追溯链在当前 Target 不存在；Baseline 就绪主张缺少机械可追溯证据。
- 关闭条件：建立正式 SYS/NFR/IF 需求与 AC/CON 层并落盘 `Traces-From` 与 `requirements_traceability.md` 的 `FORMAL_TRACE` 边，`validate_traceability.py` PASS；或由有权 Owner 正式批准例外并记录范围与理由；新独立 C04 复核。

### F-09（S3）多份受控文档状态与实际 Current Truth 矛盾

- 问题：`07_src/README.md` 仍宣称整个 `07_src` 是 `PRE_DEC_014_LEGACY_DSL_PROTOTYPE`、“本目录当前输出…不是 CNL Compiler 符合性实现”、“已声明但未实现的命令统一以 APLS-T0006 / Exit 3 失败”，与 DEC-023/DEC-026 及实际 CNL 实现直接矛盾；`04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md`、`APLS_0.1_SEMANTIC_FRAME_PROFILE.md`、`APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md`、`04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md`、`04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md`、`04_design/compiler/APLS_0.1_CNL_RESOURCE_AND_ORDERING_PROFILE.md`、`APLS_0.1_CNL_UNICODE_NFC_PROFILE.md`、`APLS_0.1_CNL_FRONTEND_ADAPTATION_DESIGN.md` 头部状态仍为 `TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018`，而 DEC-023（HDP-APLS-018 Option A）已采用这些契约作为实现输入；`04_design/detailed_design.md` 状态仍称“原 CNL 实现包继续阻塞”。
- 重要性：S3 — 权威决定以 DECISION_INDEX 为准，不造成判定标准不明；但受控文档自相矛盾，违反当前事实落盘与文档货币性预期，易误导后续评审/实现。
- 影响：读者可能误认为当前语言契约未被采用、实现仍处阻塞；07_src 的身份被其自述文件错误否定。
- 关闭条件：按 DEC-023/024/026 更新上述文档状态字段（或按事实 Owner 规则改写为正确状态），07_src/README 重写为当前 CNL 实现描述并明确旧 DSL 文件的遗留参考身份；新独立 C04 复核。

## 8. Advisory / Observation（不阻断 PASS 的非阻断事项）

- A-01：`frame_ambiguity` 的等价类排序与 Witness 选择按含 `id` 字段（派生 SHA-256）的完整 Canonical Bytes，而设计措辞为“按不含 Provenance 的 Canonical Semantic Payload Byte 升序”；两者均确定，但跨实现 Byte 一致性依赖措辞解释，建议在诊断目录中明确 `id` 是否参与该排序。
- A-02：`07_src/crates/apls-compiler/src/` 下旧 DSL 文件（`ast.rs/lexer.rs/parser.rs/resolve.rs/source.rs/index.rs/canonical.rs/ir.rs/semantic.rs/types.rs`）未挂入模块树、不参与编译；保留作为迁移参考符合 `ARCH-APLS-CNL-001` §8，但建议按 F-09 一并标注，避免被误认为现行代码。
- A-03：`expression_depth`（上限 128）在当前 Grammar 下不可达（条件深度 ≤3），属遗留防御性上限；不构成缺陷，记录以备将来 Grammar 扩展时复核。
- A-04：CNL-C010 的“枚举顺序扰动下 Envelope Byte 一致”依赖实现本身全序确定性（BTreeMap/BTreeSet），现有测试以重复执行代替扰动注入；在当前实现下充分，记录为证据性质说明。

## 9. Target 身份核验（三个时点）

时点 1（评审开始，主仓库）：

```text
git cat-file -t 3289037bee1aab64dfa2d58188379a68dcfa601e  -> commit
git rev-parse 3289037...^{tree}                            -> 51edf42da73237bcb3408e234d2cbb2095758fa1
git ls-tree -r --name-only 3289037... | wc -l              -> 249
```

副本核验：`git clone --no-checkout` 至 `/private/tmp/apls-c04-review-v2` 并 checkout 后，`git rev-parse HEAD` = `3289037bee1aab64dfa2d58188379a68dcfa601e`，`git rev-parse HEAD^{tree}` = `51edf42da73237bcb3408e234d2cbb2095758fa1`，一致。

时点 2（写入本文件前，主仓库）：上述三条命令重复执行，输出与时点 1 完全一致（commit / 51edf42da73237bcb3408e234d2cbb2095758fa1 / 249）。

时点 3（本文件写入后，主仓库）：

```text
git cat-file -t 3289037bee1aab64dfa2d58188379a68dcfa601e  -> commit
git rev-parse 3289037...^{tree}                            -> 51edf42da73237bcb3408e234d2cbb2095758fa1
git ls-tree -r --name-only 3289037... | wc -l              -> 249
```

三个时点输出完全一致，评审期间 Target 未被改动。主仓库工作区在评审前已存在的未提交治理回执修改（`00_project/ai_context/` 下 6 个文件）与本评审写入的本 Review Record 均不属于 Target；本评审未对 Target、源码、测试、设计、Schema、Manifest 或 Lock 做任何修改，未执行任何 Git 写操作或远程操作。

## 10. Open Finding 数与 Gate Decision

- Open Findings：**9**（S0：0；S1：2 — F-01、F-02；S2：5 — F-03、F-04、F-05、F-06、F-08；S3：2 — F-07、F-09）
- Advisory：4（A-01～A-04，不阻断）
- 证据受限项：E-01～E-03（不构成 Finding；不影响下述 Decision 的充分性）

**唯一 Gate Decision：`CHANGES_REQUESTED`**

依据 §38.7.4 机械判定：`REVIEW_READINESS = READY` 且 `OPEN_FINDINGS > 0` → `CHANGES_REQUESTED`。任一 Open Finding 均阻断 PASS；F-01/F-02 为 S1，默认路由 Expert Escalation → 受控整改 → 新精确 Review Target → 全新独立 C04 复审。本评审不批准任何 Exception，不关闭任何 Finding，不参与整改。

本结论只适用于精确 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e`，不外推到任何后代 Commit 或当前 `HEAD`。Baseline Adoption 仍须另行裁决，不随本评审发生。
