# TASK-019 第三轮非正式独立再复审 Target

- Target ID：`IIR-TARGET-APLS-TASK019-003`
- 冻结日期：`2026-09-04`
- Target Access：`READ_ONLY`
- 文件数：`40`
- Target Set SHA-256：`05a9e0c556ef87f72ae5f2f21f9cbae9a75b3e59fa3c474da838a71e975fc801`
- 直接输入：`IIR-APLS-TASK019-REREVIEW-002 = CHANGES_RECOMMENDED`

## 1. 摘要算法

对下列 40 个相对项目根目录的普通文件分别计算 SHA-256，形成精确的：

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
05_reviews/TASK019_INFORMAL_REREVIEW_TARGET_R2.md
05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK019_R2.md
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

1. NF-05 是否在不改变任何资源上限或生产失败语义的前提下关闭；
2. `PUBLIC_SOURCE_REACHABLE` 的三类资源是否确实可以由普通 Source/CLI 独立形成边界和首次超限证据，且不会被其他先行上限必然支配；
3. `DEFENSIVE_DOMINATED` 的四类资源是否由明确、正确的计数不变量支配；生产仍保留检查，而 `#[cfg(test)]` 私有 Ledger 注入是否只验证生产同一登记函数且没有形成第二公共接口或改变 Source 接受集；
4. CNL-C012A/B/C 是否分别形成可执行的 CLI、Module 和生产支配证据，Syntax Node 第二个 `TypedValue` Span 预期是否仍唯一；
5. BF-06、BF-10、NF-03 是否因此达到 `CLOSED`；BF-02/03/04/05/07/08/09 与 NF-01/02/04 是否保持关闭且无回归；
6. NO-04 的“Manifest 字段”残留是否已清除，Unicode 仍只有 IR Header + Binary 绑定断言且不改变 `apls --version`；
7. `DEC-017` 最终语义唯一、Declaration Bootstrap、诊断总排序、Unicode、Schema、Frame/IR/Span 是否出现任何回归或新矛盾；
8. 两个 JSON Schema 是否继续为合法 Draft 2020-12，本地 Ref 和关键条件是否保持一致；
9. 是否产生任何新未定义公共行为、不可达验收、测试后门或不可实现要求；
10. 是否有任何 Compiler Source、实现测试、Cargo Manifest/Lock、依赖或远程状态被越权修改。

## 4. Reviewer 输出要求

- 必须由未参与三轮整改设计的全新独立 Reviewer 执行；
- Review Line：`INFORMAL_INDEPENDENT`，不得冒充正式 C04；
- 开始先计算 Target Digest；不一致立即报告 `TARGET_CHANGED_REVIEW_INVALID`；
- 只读 Target，不得修改任何 Target、Compiler Source、测试、Cargo、治理或远程状态；
- 唯一允许新增/修改：`05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK019_R3.md`；
- 分别给出 BF-02～BF-10 与 NF-01～NF-05 的 `CLOSED / OPEN / PARTIAL`；
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

这些值只帮助验证“未改 Compiler 实现/依赖”的范围事实，不替代第 1 节的 40 文件 Target Digest。
