# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-008
TARGET: DES-APLS-LEX-001
DECISION: APPROVED
SELECTED_OPTION: A
DECISION_DATE: 2026-09-02
DECISION_EVIDENCE: 项目负责人“HDP-APLS-008: APPROVED / OPTION: A”
```

## WHAT_MUST_BE_DECIDED

是否采用 APLS 0.1 精确词法 Profile，冻结空白字符、换行/Span、非法数字边界、JSON Unicode 转义和非嵌套块注释的唯一行为。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

这些规则会直接改变哪些 `.apls` Byte 被接受、如何切分 Token 以及错误指向哪里，属于公共语言行为。当前已批准设计没有唯一答案，C03 不能用实现细节替项目负责人决定语言规范。

## CONFIRMED_FACTS

- `DEC-004` 要求无法唯一解释时拒绝，不能交给 AI 猜测；
- 已批准设计只精确定义了 ASCII 标识符、数字基本形式、JSON 风格字符串、注释定界符和 UTF-8 无 BOM；
- 当前设计没有列出允许的空白 Code Point，也没有冻结 CRLF、公共列计数、代理项或非法数字最大范围；
- `DES-APLS-COMPILER-FOUNDATION-001` 要求手写 Lexer、Byte Offset 和稳定错误，并禁止 Locale、Unicode Normalization 与隐式容错；
- `TASK-007` 骨架没有实现 Lexer 或部分 Grammar，因此当前没有需要兼容的既有 Lexer 行为。

## OPEN_QUESTIONS

- 本裁决包内无未决项；
- 项目负责人已选择 `OPTION A`；`OPTION B/C` 未被采用。

## OPTIONS_AND_DIFFERENCES

### OPTION A — 固定 ASCII 词法 Profile（推荐）

- 采用 `DES-APLS-LEX-001` 全文；
- 空白只允许 TAB/LF/CR/SPACE，CRLF 是一个换行；
- 公共行列一基、按 Unicode Scalar 计列；
- JSON `\u` 必须形成有效 Unicode Scalar，数字非法粘连整体拒绝；
- 块注释由第一个 `*/` 关闭，不启用嵌套。

### OPTION B — 返回 C02 扩展 Unicode 词法

- 暂不批准候选；
- 由 C02 提交精确、版本固定的 Unicode 空白 Code Point 清单及规范化/同形字风险分析；
- 禁止直接使用 Rust `char::is_whitespace` 充当语言规范。

### OPTION C — 延后词法冻结

- 保持现有文档不变；
- 不得开始正式 Lexer/Parser 实现，后续只能继续不依赖这些行为的内部骨架工作。

## RISKS_AND_TRADEOFFS

- Option A 最保守、跨工具链稳定，对非 ASCII 排版空白不宽容，但诊断明确且最符合 Agent 生成源文件的可重复性目标；
- Option B 更方便人类输入，却扩大 Source 接受集合、同形字与 Unicode 版本治理成本；
- Option C 不增加规范风险，但会阻断 Lexer/Parser 主路径。

## RECOMMENDED_OPTION_AND_REASON

推荐 `OPTION A`。它给每个 Byte 唯一 Token/错误结果，不依赖主机 Locale 或 Rust Unicode 表版本，并让后续完整 Grammar 接入不需要兼容一个临时 Lexer 方言。

## APPROVED_CONSEQUENCE

- 将所选 Profile 记录为新的当前有效决定和已批准设计输入，但不建立产品 Baseline；
- 允许 C00 建立 C03 完整手写 Lexer 实现工作包；
- 不自动授权 Parser、完整 Grammar、Commit、Push 或 Release。

## CHANGES_REQUESTED_CONSEQUENCE

`TASK-008` 返回 `IN_PROGRESS`，只修改项目负责人明确指出的词法规则，再提交同一裁决包。

## DEFERRED_CONSEQUENCE

保持 `TASK-008` 和 Lexer 实现等待，不跨越词法行为冻结 Gate。

## REJECTED_CONSEQUENCE

关闭 `DES-APLS-LEX-001` 候选；由 C02 按新的方向重新建立精确候选。

## EXPLICITLY_NOT_AUTHORIZED

本裁决包不授权修改已批准产品目标、实现 Parser 或完整 Compiler、增加依赖、Commit、Tag、Baseline Adoption、Formal C04、Push、Release、远程修改或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `DEC-004`、`DEC-007`、`DEC-010`、`DEC-011`；
- `04_design/language/APLS_0.1_LANGUAGE_DESIGN.md`；
- `04_design/language/APLS_0.1_GRAMMAR.ebnf`；
- `04_design/diagnostics/APLS_0.1_DIAGNOSTICS.md`；
- `04_design/compiler/APLS_0.1_IMPLEMENTATION_FOUNDATION_DESIGN.md`。

## COPYABLE_RESPONSE_FORMAT

推荐回复：

```text
HDP-APLS-008: APPROVED
OPTION: A
```

或：

```text
HDP-APLS-008: CHANGES_REQUESTED
修改：<具体词法规则>
```
