# APLS Baseline 索引

> 本文件是 Baseline 身份与组成的唯一权威来源。

## 当前项目 Baseline

- Baseline ID：`APLS-BASELINE-NOT-ESTABLISHED`
- Baseline Status：`DRAFT`
- 产品版本：`0.1.0-draft`
- Project Git Anchor：`MISSING — Git 已初始化，但尚无 Commit`
- 建立日期：`2026-09-02`

当前没有可声明为 `CURRENT` 的 APLS 产品 Baseline。需求、语言规范、Canonical IR、编译器实现和测试均处于未建立或候选状态。

## 治理采用源

| 对象 | 采用值 | 状态 | 证据 |
|---|---|---|---|
| 治理框架 | `ai-engineering-governance v0.1.5` | CURRENT | Tag `v0.1.5` |
| 精确 Commit | `7197ef3c91c0d97ce2578c121c088ef96a4f4469` | VERIFIED | 本地精确 Tag checkout |
| 正式归档 SHA-256 | `6a2c067f942ebc8d0ae0afa8ca6143d053bc468222e2809ca74ccdc837a2b59e` | USER_REPORTED | 本次直接归档下载未完成，未冒充独立核验 |
| 采用方式 | `Lite` | CURRENT | 项目负责人 2026-09-02 明确决定 |
| 保障节奏 | `LEAN` | CURRENT | Lite 最简运行配置 |
| Formal Seal | 未签发 | NOT_APPLICABLE | 不作推导 |

治理框架采用不等于 APLS 产品 Baseline 已采用。

## 当前候选组成

| 对象 | 状态 | 权威文件 |
|---|---|---|
| 产品需求 | DEC_014_CHANGE_INTEGRATED_NOT_BASELINED | `01_product_requirements/PRD.md` |
| 系统需求 | NOT_ESTABLISHED | 未建立 |
| 语言规范 | APPROVED_INPUT_UPDATED_BY_DEC017_NOT_BASELINED | `04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md`、`APLS_0.1_ZH_CN_GRAMMAR.ebnf` |
| Canonical IR | APPROVED_CNL_DESIGN_INPUT_NOT_BASELINED | `04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md`、`04_design/ir/apls-cnl-ir-0.1.schema.json` |
| 编译器实现 | PRE_DEC_014_LEGACY_DSL_PROTOTYPE_NOT_BASELINED | `07_src/` |
| 测试与一致性套件 | NOT_ESTABLISHED | 未建立 |
| 总体架构 | CNL_CHANGE_INTEGRATED_NOT_BASELINED | `03_architecture/system_architecture.md` |
| CNL 扩展架构 | UPDATED_BY_DEC017_NOT_BASELINED | `03_architecture/APLS_0.1_CONTROLLED_NATURAL_LANGUAGE_ARCHITECTURE.md` |
| CNL 歧义边界 | APPROVED_INPUT_UPDATED_BY_DEC017_NOT_BASELINED | `04_design/language/APLS_0.1_CNL_AMBIGUITY_BOUNDARY.md` |
| CNL Semantic Frame | APPROVED_INPUT_UPDATED_BY_DEC017_NOT_BASELINED | `04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md` |
| CNL 诊断目录 | APPROVED_INPUT_UPDATED_BY_DEC017_NOT_BASELINED | `04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md` |
| CNL Canonical IR | APPROVED_DESIGN_INPUT_BY_DEC019_NOT_BASELINED | `04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md`、`04_design/ir/apls-cnl-ir-0.1.schema.json` |
| 旧 DSL 语言与 IR 设计 | LEGACY_DSL_ONLY_OR_SUPERSEDED_BY_DEC014_DEC019 | `04_design/language/APLS_0.1_LANGUAGE_DESIGN.md`、`04_design/language/APLS_0.1_GRAMMAR.ebnf`、`04_design/ir/APLS_0.1_CANONICAL_IR.md`、`04_design/ir/apls-ir-0.1.schema.json`、`04_design/diagnostics/APLS_0.1_DIAGNOSTICS.md` |
| 旧精确词法 Profile | SUPERSEDED_AS_PUBLIC_SURFACE | `04_design/language/APLS_0.1_LEXICAL_PROFILE.md` |
| 名称解析与符号类别 Profile | SOURCE_FORM_REVIEW_REQUIRED | `04_design/language/APLS_0.1_NAME_RESOLUTION_PROFILE.md` |
| Compiler MVP 详细设计 | FRONTEND_CHANGE_REVIEW_REQUIRED | `04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md` |
| Compiler 实现技术决策 | ACCEPTED_DECISION_NOT_BASELINED | `03_architecture/architecture_decisions/ADR-APLS-001-compiler-implementation-stack.md` |
| Compiler 实现基础设计 | APPROVED_INPUT_NOT_BASELINED | `04_design/compiler/APLS_0.1_IMPLEMENTATION_FOUNDATION_DESIGN.md` |
| 当前决策 | CURRENT_SNAPSHOT | `DECISION_INDEX.md` |

## Baseline 成立条件

- 需求候选完成并通过适用质询；
- 精确 Git Anchor 存在；
- 语言边界与验收阈值明确；
- 适用的独立评审与验证完成；
- Open Finding 为 0；
- 获得项目负责人明确 Baseline Adoption。
