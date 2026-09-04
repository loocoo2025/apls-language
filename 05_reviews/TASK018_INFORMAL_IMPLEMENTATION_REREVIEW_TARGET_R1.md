# TASK-018 Retry 1 实现增量独立再复审 Target

- Target ID：`IIR-TARGET-APLS-TASK018-IMPLEMENTATION-002`
- 冻结日期：`2026-09-04`
- Review Line：`INFORMAL_INDEPENDENT`
- Review Scope：`DELTA_ONLY`
- Target Access：`READ_ONLY`
- 文件数：`72`
- Target Set SHA-256：`c3ef44330dd560569149a065fe8623de4741e7c7161bf038e5553aa831b6ced9`
- 直接输入：`IIR-APLS-TASK018-IMPLEMENTATION-001 = CHANGES_RECOMMENDED`
- Retry 授权：`HDP-APLS-019 Option A / DEC-024`

本 Target 与 Reviewer 输出报告不属于 Target Set，避免自引用。仓库尚无 Git Commit，本轮继续使用受控文件集及 SHA-256 冻结当前版本。本轮不是正式 C04，不产生正式 Gate Decision。

## 1. 摘要算法

对第 4 节列出的每个相对项目根目录普通文件计算 SHA-256，形成：

```text
<64 lowercase hex><two ASCII spaces><relative path><LF>
```

记录按 C Locale 整行 Byte 升序排序，再对完整记录流计算 SHA-256。Reviewer 必须在开始、报告写入前和结束时分别复算；任一不匹配即停止并报告 `TARGET_CHANGED_REVIEW_INVALID`。

## 2. 本轮增量范围

### 2.1 实际 Retry Delta

只复核以下七个实际修改文件与 `IO-01～IO-07` 的对应关系：

```text
04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md
07_src/crates/apls-compiler/src/cnl_lexer.rs
07_src/crates/apls-compiler/src/cnl_pipeline.rs
07_src/crates/apls-compiler/src/resource.rs
07_src/crates/apls-cli/src/lib.rs
07_src/crates/apls-cli/src/main.rs
11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md
```

### 2.2 直接影响闭包

- `IO-01`：实际 `tinyvec/alloc + default(empty) + tinyvec_macros` 闭包与 `DEC-024`、Cargo/Lock 事实一致，`std` 未启用；
- `IO-02`：状态列表只接受 `和`；所有 `ws1/opt-space` 约束在 Parse Gate 内执行，四条公共命令共享同一边界；
- `IO-03`：同一 Canonical Frame 等价类的全部候选 Provenance 稳定去重并集，不保留“获胜 Parse”；
- `IO-04`：从待交付 IR Byte 重新执行批准的 Property/Comparison/Literal、Transition、Deadline、Unit Closure 与 Source Map 跨节点不变量；
- `IO-05`：只在完整 Bind/Type/Normalize 成功后登记对应资源，失败 Candidate 不计数；
- `IO-06`：E1402/E1403、E1307、E1310 的 Primary/Related Span 与稳定 Envelope 符合冻结契约；
- `IO-07`：stdout 写入或 flush 失败返回 Exit 2，stderr 尽力输出并 flush 稳定 `APLS-T0005`，已暴露前缀不构成成功 Artifact；
- C03 Retry 1 新增测试只覆盖上述关闭条件，Cargo Manifest/Lock、依赖版本、资源数值、Unicode 版本、公共命令集合和其他产品行为未改变。

### 2.3 继承项

- `IIR-APLS-TASK019-REREVIEW-003` 的 BF-02～BF-10、NF-01～NF-05 继续为 `INHERITED_CLOSED`；
- `IIR-APLS-TASK018-IMPLEMENTATION-001` 中与 `IO-01～IO-07` 无关且未被 Retry 触及的实现结论不重复复审；
- 完整 Target Set 只用于 Byte 完整性保护，不把未变化文件全文纳入语义复审。

若 Retry 变化直接破坏某一继承前提，可记录新的实现观察；不得借此扩张为全项目扫描。

## 3. Reviewer 必须回答的问题

1. `IO-01～IO-07` 是否逐项达到上一份报告定义的可验证关闭条件；
2. 七个实际修改文件是否只实现批准的关闭语义，是否产生新的公开接受集、IR、资源、诊断或 CLI 偏差；
3. 新增最小测试和普通锁定构建/测试是否足以支持每项关闭主张；
4. `Cargo.toml`、`Cargo.lock`、依赖版本/Checksum、资源数值、Unicode 版本和旧 DSL 隔离是否保持不变；
5. 是否仍存在阻止 TASK-018 回到 `READY_FOR_REVIEW` 或进入后续 C05 的实现缺陷。

每项 IO 必须给出 `CLOSED / PARTIAL / OPEN` 和直接证据。只有全部 `CLOSED` 且没有新阻断观察时，才可输出 `READY_FOR_C00_DETERMINATION`。

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
00_project/ai_context/OPEN_QUESTIONS.md
00_project/ai_context/EXECUTION_CONTEXT.md
00_project/ai_context/EXECUTION_CONTEXT_TASK018_RETRY1.md
00_project/ai_context/HUMAN_DETERMINATION_018.md
00_project/ai_context/HUMAN_DETERMINATION_019.md
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
05_reviews/TASK018_INFORMAL_IMPLEMENTATION_REVIEW_TARGET.md
05_reviews/INFORMAL_INDEPENDENT_REVIEW_TASK018_IMPLEMENTATION.md
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

- 必须使用未参与 TASK-018 实现、Retry 1 或 C03 自检的全新隔离 Session；`private_context_inherited: false`；
- 角色：独立 Reviewer；Review Line：`INFORMAL_INDEPENDENT`；Gate Authority：`ADVISORY_ONLY`；
- 只读 Target；禁止修改设计、代码、测试、Cargo、Current Truth、任务状态、Git 或远程状态；
- 唯一允许写入：`05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK018_IMPLEMENTATION_R1.md`；允许普通测试在已排除的 `07_src/target/` 产生构建输出；
- 不安装/升级依赖、不联网、不执行 Commit/Push/Baseline/Formal C04/C05/Release/Formal Seal；
- 只复核七个 Delta 文件、`IO-01～IO-07` 关闭和直接影响闭包；其他项目内容一律继承；
- 最小机械检查为普通 `fmt/check/test --locked --offline` 和直接关闭探针；不得重复百万级资源与 Unicode 20,034 行昂贵用例；
- 新阻断观察必须给出文件/行、违反的精确契约、影响和关闭条件；非阻断事项标记 `ADVISORY`；不得使用正式 C04 S0～S3；
- 结论只允许：`READY_FOR_C00_DETERMINATION`、`CHANGES_RECOMMENDED`、`TARGET_CHANGED_REVIEW_INVALID`；
- 报告必须记录开始、写入前和结束三次 Target Digest；完成后停止，不参与整改。
