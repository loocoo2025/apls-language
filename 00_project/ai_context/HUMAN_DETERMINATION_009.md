# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-009
TARGET: DES-APLS-NAME-001
DECISION: APPROVED
SELECTED_OPTION: A
DECISION_DATE: 2026-09-03
DECISION_EVIDENCE: 项目负责人“HDP-APLS-009: APPROVED / OPTION: A”
```

## WHAT_MUST_BE_DECIDED

是否采用 APLS 0.1 名称解析与符号类别 Profile Option A，先关闭引用类别、命名空间和 `set` 无可写目标的语义缺口，再实现 Declaration Indexer / Resolver。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

这些选择会改变 `.apls` Source 的公共接受集、FQN/ID、可见性和诊断结果；其中移除 `set` 还会修订已批准设计输入。C02/C03 不能以实现便利代替项目负责人冻结语言语义。

## CONFIRMED_FACTS

- `DEC-004` 要求不唯一时失败，不得由 AI 或实现猜测；
- 已批准 Language Design 只冻结了顶层/State FQN、当前 Spec 查找、显式 Import Alias 和禁止遮蔽；
- IR Schema 的多个字段仅使用通用 `id/ref`，没有给出完整类别矩阵；
- 原始研究资料支持 Channel 是 Execution Unit 之间的路径，但这些研究草稿不是当前批准设计；
- 当前 Grammar 含 `set`，但没有 Variable/Property/State Data 声明及对应 IR Symbol Kind。

## OPTIONS_AND_DIFFERENCES

### OPTION A — 严格最小 0.1 闭合（推荐）

- 批准 `DES-APLS-NAME-001` 的单一命名空间、Alias、State/Enum Variant 限定形式和引用类别矩阵；
- Channel 端点只能是 Execution，Execution Owner 只能是 Component，Event Source 只能是 Execution；
- 0.1 从 Grammar/AST/IR Action 集合移除 `set`，但保留词法保留字；
- 将 Mutable Property/State Data 作为后续独立语言特性，需要自身的声明、类型、所有权和并发写契约。

### OPTION B — 在 0.1 增加可写数据模型

- 保留 `set`；
- 返回 C01/C02，先设计 Variable/Property/State Data 的语法、FQN、类型、Owner、初值、可变性、写冲突与 Canonical IR；
- Resolver 实现继续等待新模型获批准。

### OPTION C — 保留现状并暂停

- 不修改已批准设计；
- 不实现会依赖未定义类别或 `set` 目标的 Resolver；
- 项目保持在 Parsed/Source Graph 阶段。

## RISKS_AND_TRADEOFFS

- Option A 保持 0.1 最小、可实现、可验证，但暂时缺少直接变量赋值；
- Option B 表达力更强，但会扩大 Grammar、IR、Type/Unit、Semantic 和并发冲突设计，明显延后 Compiler MVP；
- Option C 不产生新语义风险，但主编译链无法前进。

## RECOMMENDED_OPTION_AND_REASON

推荐 `OPTION A`。它不把尚未存在的可写数据模型偷渡进 Resolver，同时保留 `emit/invoke/transition` 三种语义闭合的动作，最符合当前“拒绝歧义”和最小 Compiler MVP 目标。

## APPROVED_CONSEQUENCE

- 允许 C02 同步修订 Language/Grammar/IR/Schema 受影响部分，不建立 Baseline；
- 同步完成后，允许 C00 建立 `TASK-013` Declaration Indexer / Resolver 实现工作包；
- 不自动授权 Type/Unit、Semantic、Canonical IR、Commit、Push、Release 或 Baseline Adoption。

## DECISION_RESULT

- `OPTION A` 已获批准，`OPTION B/C` 未采用；
- 名称解析 Profile 作为已批准设计输入，不建立产品 Baseline；
- C02 可执行本包明确授权的 Language/Grammar/IR/Schema 同步；
- `TASK-013` 在同步完成后可转为 `READY`，但实现仍需新工作包授权。

## EXPLICITLY_NOT_AUTHORIZED

本裁决包不授权静默新增可写数据模型、增加依赖、更改实现技术栈、Commit、Push、Release、Formal C04、Baseline Adoption 或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `DEC-004`、`DEC-007`、`DEC-009`；
- `04_design/language/APLS_0.1_LANGUAGE_DESIGN.md`；
- `04_design/language/APLS_0.1_GRAMMAR.ebnf`；
- `04_design/ir/APLS_0.1_CANONICAL_IR.md`；
- `04_design/ir/apls-ir-0.1.schema.json`；
- `04_design/diagnostics/APLS_0.1_DIAGNOSTICS.md`；
- `04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md`。

## COPYABLE_RESPONSE_FORMAT

推荐回复：

```text
HDP-APLS-009: APPROVED
OPTION: A
```

或：

```text
HDP-APLS-009: CHANGES_REQUESTED
修改：<具体规则>
```
