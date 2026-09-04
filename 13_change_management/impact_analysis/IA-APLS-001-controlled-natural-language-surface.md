# IA-APLS-001：受控自然语言公开表面语法影响分析

- 关联变更：`CR-APLS-001`
- 当前 Baseline：`APLS-BASELINE-NOT-ESTABLISHED`
- 变更分类：`SUBSTANTIVE`
- 日期：2026-09-03

## 受影响链路

| 上游变化 | 受影响下游 | 影响 | 必须动作 | Owner |
|---|---|---|---|---|
| 公开 Surface 改为 CNL | `01_product_requirements/PRD.md` | 用户交互与验收标准改变 | 更新产品定位和 P0 要求 | C01/C02 |
| 公开 Surface 改为 CNL | `03_architecture/system_architecture.md` | Frontend 和歧义防火墙改变 | 更新架构 | C02 |
| Unicode/CNL 句子 | `DES-APLS-LEX-001`、正式 EBNF | ASCII DSL 接受集失效 | 重新设计 | C02 |
| 自然术语与句间上下文 | `DES-APLS-NAME-001` | ASCII FQN 源码形式失效 | 保留符号类别，重建 Source 形式 | C02 |
| Semantic Frame 新增 | Compiler/AST 设计 | Stage 边界改变 | 增加 CNL AST/Frame 与歧义验证 | C02 |
| 新公开 Source | `07_src` Lexer/Parser/AST | 旧代码不符新公开语法 | 设计获批后受控重构 | C03 |
| 新诊断类别 | 测试与 Conformance | 需覆盖术语、模糊量、指代、否定等 | 重建 CNL 一致性套件 | C05 |

## 可直接保留或优先复用

| 对象 | 证据 | 原批准是否继续有效 |
|---|---|---|
| `DEC-004` 拒绝歧义 | CNL 方向加强而非弱化该要求 | YES |
| Canonical IR 作为 Agent 契约 | 与表面语言分层 | YES，待映射复核 |
| Rust Workspace 与互斥 Stage Artifact | 实现基础与文本语言无直接冲突 | YES，Parser 适配性待复核 |
| Source Byte/SHA-256/竞态复核 | 文本源的不可变性要求不变 | YES，路径与扩展名待复核 |
| 公共 Diagnostic Envelope/Exit Code | 失败分类原则不变 | YES，诊断目录需扩展 |
| Indexer/Resolver 的确定绑定原则 | 每个有效 Frame Candidate 内的 Symbol Ref 必须确定；不同候选可在最终 Frame 层收敛 | CONCEPT_ONLY，Source 名称形式需重建 |

## 当前失效或不得继续宣称符合

- `DEC-007` 的花括号 DSL 公开语法；
- `DEC-012` 作为新公开语法的完整词法 Profile；
- `DEC-013` 中 `N/A.N/P.N/A.P.N` 作为用户必写的公开源码形式；
- `TASK-009/010/013` 作为 CNL Lexer/Parser/AST/Resolver 符合性证据。

## 重新批准与回归范围

- 必须重新批准：CNL 语言 Profile、精确词法、Grammar、术语/指代规则、Semantic Frame、Source-to-IR 映射、新诊断。
- 无需重新决定：拒绝歧义、Canonical IR 规范性、LLM 不作语义裁决器、Rust 作为当前主实现语言。
- 必须重新验证：Lexer/Parser 工具冲突、分析候选完整性、最终 Canonical Frame 收敛唯一性、同义句归一、否定与并列范围、诊断稳定性、同一语义的 IR 一致性。

## Gate 与 Baseline

- 旧 Implementation Gate：`NO LONGER CURRENT`
- 新 Gate：`C02_CNL_ARCHITECTURE_AND_AMBIGUITY_BOUNDARY`
- 是否需要新 ADR：`REVIEW`（待 Parser 技术适配性评估）
- 是否建立 Baseline：`NO`
- 现有代码修改授权：`NO`
- Formal Seal：`NOT_APPLICABLE`

## 结论

```text
IMPACT_SCOPE: PRODUCT_SURFACE_ARCHITECTURE_LANGUAGE_DESIGN_COMPILER_FRONTEND_TESTS
REAPPROVAL_SCOPE: CNL_PROFILE_LEXER_GRAMMAR_FRAME_DIAGNOSTICS_SOURCE_TO_IR
REGRESSION_SCOPE: DETERMINISM_FAIL_CLOSED_IR_EQUIVALENCE
UNAFFECTED_APPROVALS_PRESERVED: DEC-004_CANONICAL_IR_PRINCIPLE_RUST_CORE_SAFETY
REMAINING_UNKNOWN: FIRST_NORMATIVE_HUMAN_LANGUAGE_PROFILE_AND_PARSER_FIT
NEXT_ACTION: HDP-APLS-011_FIRST_NORMATIVE_LANGUAGE_PROFILE
```
