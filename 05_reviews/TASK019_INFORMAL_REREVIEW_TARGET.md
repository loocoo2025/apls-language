# TASK-019 非正式独立再复审 Target

- Target ID：`IIR-TARGET-APLS-TASK019-001`
- 冻结日期：`2026-09-04`
- Target Access：`READ_ONLY`
- 文件数：`33`
- Target Set SHA-256：`219e6ad30aa67d09a689efbad3c2fdf9c8ae7e9e01111736e7d069af7b8d4e6c`

## 1. 摘要算法

对下列 33 个相对项目根目录的普通文件分别计算 SHA-256，形成精确的：

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
05_reviews/INFORMAL_INDEPENDENT_REVIEW_TASK018_PRE_IMPLEMENTATION.md
04_design/detailed_design.md
04_design/language/APLS_0.1_CNL_AMBIGUITY_BOUNDARY.md
04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md
04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf
04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md
04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md
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

## 3. 独立再复审问题

Reviewer 必须独立判断：

1. BF-02～BF-10 是否分别达到原报告的建议关闭条件；
2. 整改文件之间是否形成新的矛盾、缺口或不可实现要求；
3. `DEC-017` 的最终语义唯一是否仍被完整保护，是否存在顺序、概率、默认 Parse 或资源误分类路径；
4. Type/Unit/Transition/Condition/Unit Closure/Source Map 是否足够机械、封闭且跨实现一致；
5. Unicode 17.0.0、NFC API、Crate/传递闭包、许可与 MSRV 候选是否准确；注意当前未修改 Cargo，依赖采用必须等待 HDP-APLS-018；
6. 两个 JSON Schema 是否为合法 Draft 2020-12、所有本地 Ref 可解析、条件结构与文档一致；
7. 零候选聚合、E1310 Witness、T0007 Payload/Span/排序是否可直接实现和黑盒验证；
8. 14 个 Conformance Case 是否以最小充分方式覆盖原九项 Finding；
9. 是否有任何 Compiler Source、实现测试、Cargo Manifest 或 Cargo Lock 被本轮越权修改。

## 4. Reviewer 输出要求

- Review Line：`INFORMAL_INDEPENDENT`，不得冒充正式 C04；
- 先计算开始 Target Digest；不一致立即报告 `TARGET_CHANGED_REVIEW_INVALID`；
- 只读 Target，不得修改任何 Target 文件；
- 唯一允许新增/修改：`05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK019.md`；
- 逐项给出 BF-02～BF-10 的 `CLOSED / OPEN / PARTIAL`；
- 结论只允许 `READY_FOR_HUMAN_DETERMINATION / CHANGES_RECOMMENDED / TARGET_CHANGED_REVIEW_INVALID`；
- 报告写入前与结束时再次计算同一 Target Digest并记录；
- 即使全部关闭，也不能批准公共契约、修改依赖、恢复 TASK-018、建立 Baseline 或签发 Formal Seal。

## 5. 非 Target 实现完整性证据

本轮开始再复审前，`07_src` 下排除 `target/` 的普通文件集合摘要为：

```text
5f80929f91c106ff7c0f43ce57633c78eca890e89d6eb6272e4dede7e9c20230
```

该值只用于帮助检查“未改 Compiler 实现”的范围事实，不替代第 1 节的 33 文件 Target Digest。
