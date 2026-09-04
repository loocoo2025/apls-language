# TASK-019 第二轮非正式独立再复审 Target

- Target ID：`IIR-TARGET-APLS-TASK019-002`
- 冻结日期：`2026-09-04`
- Target Access：`READ_ONLY`
- 文件数：`38`
- Target Set SHA-256：`bfada6b1248b6f1ed3ad3222392d1956d81462d18b1f5ab9eb99e45998d26c14`
- 直接输入：`IIR-APLS-TASK019-REREVIEW-001 = CHANGES_RECOMMENDED`

## 1. 摘要算法

对下列 38 个相对项目根目录的普通文件分别计算 SHA-256，形成精确的：

```text
<64 lowercase hex><two ASCII spaces><relative path><LF>
```

记录按 C Locale 整行 Byte 升序排序，再对完整记录流计算 SHA-256。本文和 Reviewer 输出报告不属于 Target，避免自引用。

## 2. 精确文件集

```text
AI_START_HERE.md
00_project/ai_context/CURRENT_STATE.md
00_project/ai_context/ACTIVE_TASKS.md
00_project/ai_context/OPEN_QUESTIONS.md
00_project/ai_context/DECISION_INDEX.md
00_project/ai_context/EXECUTION_CONTEXT.md
00_project/ai_context/HUMAN_DETERMINATION_015.md
00_project/ai_context/HUMAN_DETERMINATION_016.md
00_project/ai_context/HUMAN_DETERMINATION_017.md
03_architecture/system_architecture.md
03_architecture/APLS_0.1_CONTROLLED_NATURAL_LANGUAGE_ARCHITECTURE.md
05_reviews/INFORMAL_INDEPENDENT_REVIEW_TASK018_PRE_IMPLEMENTATION.md
05_reviews/TASK019_INFORMAL_REREVIEW_TARGET.md
05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK019.md
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
04_design/compiler/APLS_0.1_TASK019_REMEDIATION_RECORD.md
04_design/ir/APLS_0.1_CNL_TO_IR_MAPPING.md
04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md
04_design/ir/apls-cnl-ir-0.1.schema.json
06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md
07_src/Cargo.toml
07_src/Cargo.lock
07_src/crates/apls-compiler/Cargo.toml
07_src/crates/apls-cli/Cargo.toml
```

## 3. 本轮必须独立回答的问题

1. BF-06、BF-07、BF-10 的第一次 `PARTIAL` 剩余项是否全部达到 `CLOSED`；BF-02/03/04/05/08/09 是否保持关闭且无回归；
2. NF-01 是否由一个无循环、可实现且不引入第二 Parser 的 Declaration Bootstrap 唯一闭合：无类别 `DECLARED_TERM`、唯一正式 Grammar Entry、全部 Provisional Header 后绑定、前向引用、错误归类与两 Pass 资源计数是否一致；
3. NF-02 是否由一个跨 Compiler MVP/CNL/通用 Diagnostics/Foundation/Resource/Conformance 一致的 Byte-based 公共总排序键闭合，尤其是相同 Start、不同 End/Code、Null Span、Related 与最终 Tie-breaker；
4. NF-03 是否为固定 Syntax Node Taxonomy 的每个节点给出唯一原始 UTF-8 Byte Span，CNL-C012 的嵌套超限 Case 能否给出唯一黑盒期望；
5. NF-04 是否明确且一致地删除独立 Compiler Manifest 要求，并把 IR Header + Binary 常量/构建启动断言定义为唯一契约，同时不擅自改变 `apls --version`；
6. `DEC-017` 的最终语义唯一是否仍完整：不得由 Pass 顺序、声明顺序、Lexer 优先级、Parser Entry、资源耗尽、概率、LLM 或第一个成功候选制造唯一性；
7. 两个 JSON Schema 是否继续为合法 Draft 2020-12，全部本地 Ref、关键条件和 Cross-validator 边界与文档一致；
8. CNL-C001～C016 是否对原 Finding 与 NF-01～NF-04 形成最小充分、可执行的实施前判定合同；
9. 是否产生任何新矛盾、未定义公共行为或不可实现要求；
10. 是否有任何 Compiler Source、实现测试、Cargo Manifest/Lock、依赖或远程状态被越权修改。

## 4. Reviewer 输出要求

- 必须由未参与两轮整改设计的全新独立 Reviewer 执行；
- Review Line：`INFORMAL_INDEPENDENT`，不得冒充正式 C04；
- 开始先计算 Target Digest；不一致立即报告 `TARGET_CHANGED_REVIEW_INVALID`；
- 只读 Target，不得修改任何 Target、Compiler Source、测试、Cargo 或治理文件；
- 唯一允许新增/修改：`05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK019_R2.md`；
- 分别给出 BF-02～BF-10 与 NF-01～NF-04 的 `CLOSED / OPEN / PARTIAL`；
- 结论只允许 `READY_FOR_HUMAN_DETERMINATION / CHANGES_RECOMMENDED / TARGET_CHANGED_REVIEW_INVALID`；
- 报告写入前与结束时再次计算同一 Target Digest 并记录；
- 即使全部关闭，也不能批准公共契约、修改依赖、恢复 TASK-018、建立 Baseline、发起 Formal C04、Commit、Push、Release 或 Formal Seal。

## 5. 非 Target 实现完整性证据

本轮 Target 冻结前，`07_src` 下排除 `target/` 的普通文件集合摘要为：

```text
5f80929f91c106ff7c0f43ce57633c78eca890e89d6eb6272e4dede7e9c20230
```

四个受保护 Cargo 文件的 SHA-256：

```text
d3c90a730c55dbf47ca4ecc55fd68dd2503ae9a9f323fc907255216ce5ba5d95  07_src/Cargo.toml
138d064cb1af6721e313c9b5454b184b3cfeaebe8f15b970aa23216b94875b84  07_src/Cargo.lock
2c205e76502710f3f37b674b5b3ea047c0f660315cbb86f13cfb24bbe73dbb8c  07_src/crates/apls-compiler/Cargo.toml
29e2b4f937935caef9f5fc13de5be3eb880fecf1886ee9389bf4fb40c8dbe1f9  07_src/crates/apls-cli/Cargo.toml
```

这些值只帮助验证“未改 Compiler 实现/依赖”的范围事实，不替代第 1 节的 38 文件 Target Digest。
