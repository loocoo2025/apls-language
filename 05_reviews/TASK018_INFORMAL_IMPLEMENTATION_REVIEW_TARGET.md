# TASK-018 实现增量非正式独立复审 Target

- Target ID：`IIR-TARGET-APLS-TASK018-IMPLEMENTATION-001`
- 冻结日期：`2026-09-04`
- Review Line：`INFORMAL_INDEPENDENT`
- Review Scope：`DELTA_ONLY`
- Target Access：`READ_ONLY`
- 文件数：`67`
- Target Set SHA-256：`b4e377fe844ffd55403bf8a74bb4954107fef4dfe3719e2dfa461e1bd3952658`
- 上一有效独立证据：`IIR-APLS-TASK019-REREVIEW-003 = READY_FOR_HUMAN_DETERMINATION`
- 当前实施证据：`TASK-018 = OUTPUT_READY`、`TASK018_C03_IMPLEMENTATION_VALIDATION = PASS / OUTPUT_READY_CANDIDATE`

本 Target 和 Reviewer 输出报告不属于 Target Set，避免自引用。仓库尚无 Git Commit，因此本轮使用受控文件集及 SHA-256 冻结当前实现版本；本轮不是正式 C04，不产生正式 Gate Decision。

## 1. 摘要算法

对第 4 节列出的每个相对项目根目录普通文件计算 SHA-256，形成：

```text
<64 lowercase hex><two ASCII spaces><relative path><LF>
```

记录按 C Locale 整行 Byte 升序排序，再对完整记录流计算 SHA-256。Reviewer 必须在开始、报告写入前和结束时分别复算；任一不匹配即停止并报告 `TARGET_CHANGED_REVIEW_INVALID`。

## 2. 增量范围

### 2.1 本轮语义复审 Delta

只复审 TASK-019 第三轮 Target 冻结后形成的 TASK-018 CNL 实现及其直接影响闭包：

- `07_src/` 中简体中文 CNL Lexer、候选 Lattice/Stream、LALRPOP Grammar、AST、绑定/类型/单位、Canonical Frame/IR、资源登记、诊断和验证实现；
- `apls-cli` 的 `parse/check/emit-ir/diagnose` 公共路径、退出码、stdout/stderr、发布前复验和旧 DSL 不可达性；
- TASK-018 引入的 Unicode NFC 依赖、Cargo/Lock、Build Script 和 Schema Byte 绑定；
- 与上述实现直接相关的最小测试和 C03 验证主张。

### 2.2 直接影响闭包

- `DEC-017`：表层分析允许多候选，规范语义必须唯一；不得用优先级、概率、LLM 或首个成功路径制造唯一性；
- `DEC-019/020/023`：Canonical IR、完整垂直切片、资源上限、Unicode 17.0.0 和依赖边界；
- `WP-APLS-CNL-C03-001` 的八组高价值行为证据；
- `apls-zh-CN-0.1` 公开 Source 接受/拒绝边界和稳定诊断机器契约；
- 旧 DSL 与当前 CNL 公共成功路径隔离。

### 2.3 继承项

`IIR-APLS-TASK019-REREVIEW-003` 已关闭且本轮实现没有改变其规范前提的 BF-02～BF-10、NF-01～NF-05，统一作为 `INHERITED_CLOSED`。不得重复审查 TASK-019 的设计整改过程；仅当当前实现与已关闭契约直接冲突时，才记录新的实现观察项。

文件出现在完整 Target Set 中，只表示其 Byte 受到完整性保护，不表示其全文进入语义复审范围。

## 3. Reviewer 必须回答的问题

1. 当前唯一公开成功路径是否完整覆盖 Profile、七类声明、Rule、Transition、Invariant、Acceptance 和 Informative Sentence，并拒绝旧 DSL；
2. 多 Tokenization/Parse 候选是否只在 Canonical Frame 层按规范语义等价性收敛，`0/1/>1` 分支是否失败关闭，且无排名、最长匹配、首个成功或 LLM 猜测；
3. 无有效候选时是否给出术语、角色、绑定、类型/单位等具体错误，而不是误报 `APLS-E1310`；两个以上不等价 Frame 是否稳定产生带分歧见证的 `APLS-E1310`；
4. Declaration Bootstrap、Owner/类别约束、状态比较、Transition 可达/冲突和名义单位语义是否与批准契约一致；
5. Canonical Frame、Semantic ID、匿名节点 ID、文档语义 Hash、重复节点 Provenance 并集、Canonical JSON、Schema 与跨引用复验是否确定且失败关闭；
6. 七类候选资源的计数对象、登记顺序、边界、Span、`APLS-T0007` Payload 和 Artifact 禁止是否与 Resource Profile 一致；私有 Ledger 是否只存在于测试配置且不形成公共后门；
7. NFC 是否固定 Unicode 17.0.0，Binary 绑定断言、IR Header、依赖 Feature/Lock/License/MSRV 是否与 `DEC-023` 一致；
8. CLI 的四个命令、退出码、stdout/stderr、诊断 Envelope、Source 竞态复读和发布边界是否符合已批准契约；
9. C03 验证记录是否能由代码和最小独立机械检查支持，有无遗漏、夸大或不可复现的关键主张；
10. 是否存在会阻止 TASK-018 进入 `READY_FOR_REVIEW` 或后续 C05 验证的实现缺陷。

## 4. 精确 Target 文件集

```text
AI_START_HERE.md
AI_ENGINEERING_RULES_V2.md
00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md
00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml
00_project/governance/PROJECT_ASSURANCE_CADENCE_POLICY.md
00_project/governance/AI_TESTING_GOVERNANCE_RULES.md
00_project/ai_context/ROLE_BRIEFS/C04_INDEPENDENT_REVIEW.md
00_project/ai_context/CURRENT_STATE.md
00_project/ai_context/BASELINE_INDEX.md
00_project/ai_context/ACTIVE_TASKS.md
00_project/ai_context/DECISION_INDEX.md
00_project/ai_context/EXECUTION_CONTEXT.md
00_project/ai_context/HUMAN_DETERMINATION_018.md
01_product_requirements/PRD.md
03_architecture/system_architecture.md
03_architecture/APLS_0.1_CONTROLLED_NATURAL_LANGUAGE_ARCHITECTURE.md
04_design/detailed_design.md
04_design/language/APLS_0.1_CNL_AMBIGUITY_BOUNDARY.md
04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md
04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf
04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md
04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md
04_design/diagnostics/APLS_0.1_DIAGNOSTICS.md
04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md
04_design/diagnostics/apls-cnl-diagnostic-0.1.schema.json
04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md
04_design/compiler/APLS_0.1_IMPLEMENTATION_FOUNDATION_DESIGN.md
04_design/compiler/APLS_0.1_CNL_FRONTEND_ADAPTATION_DESIGN.md
04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md
04_design/compiler/APLS_0.1_CNL_RESOURCE_AND_ORDERING_PROFILE.md
04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md
04_design/ir/APLS_0.1_CNL_TO_IR_MAPPING.md
04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md
04_design/ir/apls-cnl-ir-0.1.schema.json
05_reviews/INFORMAL_INDEPENDENT_REVIEW_TASK018_PRE_IMPLEMENTATION.md
05_reviews/TASK019_INFORMAL_REREVIEW_TARGET_R3.md
05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK019_R3.md
06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md
11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md
07_src/Cargo.toml
07_src/Cargo.lock
07_src/rust-toolchain.toml
07_src/crates/apls-compiler/Cargo.toml
07_src/crates/apls-compiler/build.rs
07_src/crates/apls-compiler/src/apls_grammar.lalrpop
07_src/crates/apls-compiler/src/ast.rs
07_src/crates/apls-compiler/src/canonical.rs
07_src/crates/apls-compiler/src/cnl_ast.rs
07_src/crates/apls-compiler/src/cnl_lexer.rs
07_src/crates/apls-compiler/src/cnl_pipeline.rs
07_src/crates/apls-compiler/src/diagnostic.rs
07_src/crates/apls-compiler/src/index.rs
07_src/crates/apls-compiler/src/ir.rs
07_src/crates/apls-compiler/src/lexer.rs
07_src/crates/apls-compiler/src/lib.rs
07_src/crates/apls-compiler/src/limits.rs
07_src/crates/apls-compiler/src/parser.rs
07_src/crates/apls-compiler/src/pipeline.rs
07_src/crates/apls-compiler/src/resolve.rs
07_src/crates/apls-compiler/src/resource.rs
07_src/crates/apls-compiler/src/semantic.rs
07_src/crates/apls-compiler/src/source.rs
07_src/crates/apls-compiler/src/types.rs
07_src/crates/apls-compiler/src/validate.rs
07_src/crates/apls-cli/Cargo.toml
07_src/crates/apls-cli/src/lib.rs
07_src/crates/apls-cli/src/main.rs
```

## 5. Reviewer 身份、权限与输出

- 使用未参与 TASK-018 实现和自检的全新隔离 Session；`private_context_inherited: false`；
- 角色：独立 Reviewer；Review Line：`INFORMAL_INDEPENDENT`；Gate Authority：`ADVISORY_ONLY`；
- 只读 Target；禁止修改设计、代码、测试、Cargo、Current Truth、任务状态、Git 或远程状态；
- 允许写入：`05_reviews/INFORMAL_INDEPENDENT_REVIEW_TASK018_IMPLEMENTATION.md`；允许测试在已排除的 `07_src/target/` 产生临时构建输出；
- 不安装/升级依赖、不访问网络、不执行 Commit/Push/Baseline/Formal C04/Release/Formal Seal；
- 先列出 Delta、直接影响闭包和 `INHERITED_CLOSED`，不得扩张为全项目扫描；
- 最小机械检查默认仅运行普通 `cargo check/test --locked`；不重复运行已通过的百万级资源和 Unicode 20,034 行昂贵用例，除非发现直接证据要求复现；
- 每个阻断观察项必须给出文件/行、违反的精确契约、影响和可验证关闭条件；非阻断事项标记 `ADVISORY`；不得使用正式 C04 S0～S3 或冒充 Gate Finding；
- 结论只允许：`READY_FOR_C00_DETERMINATION`、`CHANGES_RECOMMENDED`、`TARGET_CHANGED_REVIEW_INVALID`；
- 报告必须记录开始、写入前和结束三次 Target Digest；写完结论即停止，不参与整改。
