# APLS 0.1 精确词法 Profile

- 设计 ID：`DES-APLS-LEX-001`
- 状态：`SUPERSEDED_AS_PUBLIC_SURFACE_BY_DEC-014`
- 批准依据：`HDP-APLS-008 Option A` / `DEC-012`
- 上游输入：`DEC-004`、`DEC-007`、`DEC-010`、`DEC-011`、`DES-APLS-LANG-001`
- 目标：关闭会改变 Source 接受集合、Token 边界或诊断 Span 的词法未定义项

> 本 Profile 仅适用于旧 ASCII DSL 参考实现；`DEC-014` 要求为受控自然语言重建 Unicode、术语和句子词法 Profile。

## 1. 字节、字符与空白

- Source 必须是合法 UTF-8，且不得以 UTF-8 BOM `EF BB BF` 开头。
- 规范空白只包含 `U+0009 TAB`、`U+000A LF`、`U+000D CR`、`U+0020 SPACE`。
- `CRLF` 作为一个换行；孤立 `CR` 和 `LF` 也各作为一个换行。
- 其他 Unicode 空白、格式字符和控制字符在字符串或注释之外均为 `APLS-E1001`。
- Lexer 不调用 Locale、Unicode Normalization 或随 Unicode 数据版本变化的 `is_whitespace` 类判断。

## 2. Byte Span 与行列

- Token 和内部 Lexer Finding 使用零起点、半开 Byte Span：`[start_byte, end_byte)`。
- 公共诊断行列为一基；列按 Unicode Scalar Value 计数，不按 UTF-8 Byte 或显示宽度计数。
- TAB 占一个 Scalar 列；换行后的下一个 Scalar 位于下一行第 1 列。
- `CRLF` 的两个 Byte 属于同一个换行事件；Span 仍保持原始 Byte 边界。

## 3. 标识符与关键字

- 标识符严格为 `[A-Za-z_][A-Za-z0-9_]*`，区分大小写。
- 关键字集合完全从 `APLS_0.1_GRAMMAR.ebnf` 的固定字面量导出，并优先于标识符。
- 非 ASCII Scalar 不能进入标识符；出现在字符串/注释之外时为 `APLS-E1001`。
- 不进行近似关键字、大小写修复或 Unicode 同形字替换。

## 4. 数字

- Integer：`0|[1-9][0-9]*`。
- Decimal：`(0|[1-9][0-9]*)\.[0-9]+`。
- 负号始终是独立 `-` Token；是否允许负值由 Grammar 决定。
- 从 ASCII 数字开始的最大连续 `[A-Za-z0-9_.]` 序列若不能整体匹配 Integer 或 Decimal，则整个序列为一个 `APLS-E1003` 范围。
- 因此前导零、指数形式、缺失小数位、多小数点及数字与标识符直接粘连均不会被拆成多个看似合法 Token。

## 5. 字符串

- 字符串使用双引号；未转义的 `U+0000..U+001F` 禁止。
- 唯一转义集合为 JSON 转义：`\"`、`\\`、`\/`、`\b`、`\f`、`\n`、`\r`、`\t`、`\uXXXX`。
- `\uXXXX` 必须表示 Unicode Scalar Value；高代理项必须紧邻一个 `\u` 低代理项，孤立高/低代理项拒绝。
- 非法转义的最小 Span 为反斜线至能判定失败的位置；未闭合字符串为开始引号至 EOF，均使用 `APLS-E1002`。
- Lexer 保存原始字符串 Byte；解码值由后续 AST 构造完成，不接受依赖库的宽松替代字符行为。

## 6. 注释

- 行注释从 `//` 开始，在 `CRLF`、孤立 `CR`、`LF` 或 EOF 前结束。
- 块注释从 `/*` 开始，由遇到的第一个 `*/` 结束；内部 `/*` 只是注释文本，不增加嵌套深度。
- 未闭合块注释使用 `APLS-E1002`，范围为开始定界符至 EOF。
- 注释可包含任意合法 UTF-8 Scalar；注释等价于空白且不生成 Token。

## 7. Token 与失败边界

- 多字符运算符 `== != <= >=` 使用最长匹配；其余固定标点和运算符按 Grammar 字面量生成。
- 每个非法字符只产生一个根因 Finding；Lexer 不自动删除、替换或重试输入。
- Token 总数不含空白、注释和 EOF；试图产生第 `1,000,001` 个 Token 时以 `APLS-T0007` Tool Failure 终止。
- 发生任何 Lexer Error 时不得构造 `ParsedProgram` 或后续 Artifact。

## 8. 变更影响

本 Profile 会补充而不替换 `DES-APLS-LANG-001`。它属于 `SUBSTANTIVE` 语法行为冻结：批准前不得由 C03 在代码中自行选择；批准后任何改变都必须重新分析 Source 接受集合、Token/Span、诊断和 Conformance Fixture。
