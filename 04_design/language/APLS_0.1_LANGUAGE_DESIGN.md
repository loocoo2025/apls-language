# APLS 0.1 表面语言详细设计

- 设计 ID：`DES-APLS-LANG-001`
- 状态：`SUPERSEDED_AS_PUBLIC_SURFACE_BY_DEC-014`
- 决策输入：`DEC-007`
- 正式 Grammar：`APLS_0.1_GRAMMAR.ebnf`
- 精确词法 Profile：`APLS_0.1_LEXICAL_PROFILE.md`（`HDP-APLS-008 Option A` / `DEC-012` 已批准）
- 名称解析 Profile：`APLS_0.1_NAME_RESOLUTION_PROFILE.md`（`HDP-APLS-009 Option A` / `DEC-013` 已批准）

> `DEC-014` 已将 APLS 唯一公开 Surface Syntax 改为受控自然语言。本文仅保留为旧 DSL 设计参考，不再定义当前公开源语言。

## 1. 设计原则

- `.apls` 是唯一规范性源文件格式；JSON 只用于 Canonical IR。
- 花括号定义结构，分号终止语句；缩进和换行不影响语义。
- 0.1 标识符仅允许 ASCII，字符串允许 Unicode。
- 所有规范字段使用保留关键字；不允许任意键值扩展。
- 同一块内同名字段最多出现一次；重复字段报错。
- 任何隐式默认值必须由语言版本规定，并在 IR 中展开。
- Parser 错误恢复只产生诊断，不产生可验证语义模型。

## 2. 词法规则

### 标识符

```text
[A-Za-z_][A-Za-z0-9_]*
```

区分大小写。关键字不能作为标识符。限定名用 `.` 连接，每段分别满足标识符规则。

### 字面量

- 整数：`0` 或非零数字开头的十进制整数；禁止前导零。
- 小数：整数部分、`.` 和至少一位小数；0.1 不支持指数形式。
- 布尔：`true`、`false`。
- 字符串：双引号，转义与 JSON 字符串一致；禁止未转义控制字符。
- 工程量：`number @ UnitName`，例如 `100 @ ms`；`@` 消除数字和单位的词法粘连。

Lexer 使用最长匹配；当文本同时可被识别为关键字和标识符时，关键字优先。`1.0` 是一个小数 Token，`1 @ second` 是一个工程量；词法层不尝试推断单位。

### 注释与空白

- 行注释：`// ...`；
- 块注释：`/* ... */`，不可嵌套；
- 注释等价于空白，不进入规范语义；
- 文件编码固定为 UTF-8，无 BOM。

## 3. 顶层结构

每个文件必须且只能包含：

1. 一个 `apls "0.1";` 语言版本声明；
2. 零个或多个带显式别名的导入；
3. 一个 `spec` 声明，其第一个字段必须且只能是一个 `version`。

导入必须使用字符串路径和显式别名。循环导入、重复别名和同名符号不允许。

## 4. 0.1 声明集合

- `dimension` / `unit`：工程量维度与单位；
- `type` / `enum` / `record`：数据类型；
- `domain` / `component`：运行环境和责任边界；
- `operation`：显式声明为 `pure` 或 `action`；只有 `pure` 可出现在表达式中，只有 `action` 可被 `invoke`；
- `transport`：Channel 引用的显式传输机制声明；
- `event` / `execution` / `channel`：运行与通信；
- `state_machine`：状态与转换；
- `rule` / `constraint` / `safety` / `acceptance`：行为和验证；
- `decision` / `open` / `unknown`：治理说明、未决事项与已知知识缺口；三者均不能隐式改变规范语义；
- `intent`、`responsibility`、`rationale`：说明字段，不参与规范求值。

## 5. 表达式与动作

表达式优先级从低到高固定为：

```text
or
and
== !=
< <= > >=
+ -
* / %
not unary-
primary
```

0.1 不允许用户自定义运算符、赋值表达式或带副作用的表达式。函数调用只允许引用已声明的纯操作或规范内建函数。

动作只有三种：

- `emit Event(...);`
- `invoke Operation(...);`
- `transition Machine to State;`

不允许在规则动作块中出现赋值、循环、条件分支或隐式控制流。`set` 保留为词法关键字，但不是 0.1 Grammar 的合法动作；恢复该动作必须先冻结 Mutable Property/State Data 声明和 IR 契约。

## 6. 名称、类型与默认值

- 名称空间、限定形式、引用类别和失败规则的唯一规范是已批准的 `DES-APLS-NAME-001`。
- 规范内顶层声明的 Fully Qualified Name 为 `<spec>.<name>`；State 和 Transition 的嵌套 ID 为 `<spec>.<machine>.<name>`；Enum Variant 的内部语义身份为 `<spec>.<enum>.<variant>`。
- 未限定名只查找当前局部 Binding 或当前 Spec；导入符号必须使用当前 Source 的显式 Alias，不允许使用被导入 Spec 名或透传 Import。
- `N`、`Alias.N`、`Parent.N` 和 `Alias.Parent.N` 是 0.1 唯一允许的名称形式；`Parent` 只能是 State Machine 或 Enum。
- 同一 Spec 的具名顶层声明共享名称空间；State/Transition 共享所属 Machine 的嵌套 ID 空间；Import Alias 不得与当前 Spec 顶层声明同名。
- 0.1 禁止通配导入、遮蔽、近似匹配和“最近名称”回退；重复、零候选、多候选或类别不匹配分别失败。
- 布尔字面量的类型为 `bool`；字符串为 `string`；无小数点的数字为 `int`；有小数点的数字为 `decimal`。
- 工程量的类型由已解析单位唯一决定；跨维度运算和未声明的隐式转换失败。
- Base Unit 的 `scale` 必须为 `1`、`offset` 必须为 `0`；其他 Unit 的 `scale` 必须大于 `0`。
- `operation kind pure` 必须声明 `returns`，`operation kind action` 必须省略 `returns`。
- `schedule periodic(...)` 必须且只能包含命名参数 `period`，其值必须是大于零的 Time 维度工程量；其他 Schedule 不接受参数。
- 0.1 不为缺失的规范字段提供实现私有默认值；Grammar 中可选字段在 IR 中必须按下表显式展开。

| Surface 省略项 | Canonical IR 值 |
|---|---|
| Unit `offset` | `"0"` |
| Record Field Default | `null` |
| Action Operation `returns` | `null` |
| Event `payload/source` | `null` / `null` |
| Execution `owner` | `null` |
| Channel `capacity/timeout` | `null` / `null` |
| State `terminal` | `false` |
| Transition `guard/priority/then` | `null` / `null` / `[]` |
| Rule `hold_for` | `null` |
| Acceptance `within` | `null` |
| 非 Periodic Schedule Parameter Map | `{}` |
| 无关联 Knowledge `subject_ref` | `null` |
| 非 Decision Knowledge `decision_status` | `null` |

## 7. 歧义拒绝规则

- Grammar 生成器报告任何解析冲突时，Grammar 候选不合格。
- 名称解析出现零个或多个候选时均失败。
- 类型无法唯一推断时要求显式类型。
- 同一状态/事件的转换 Guard 无法证明互斥且没有显式唯一优先级时失败。
- `open` 或 `unknown` 被规范字段依赖时失败。
- 未知关键字、未知字段和未知扩展默认失败。

## 8. 语法到 IR 的唯一映射

| Surface | Canonical IR |
|---|---|
| `apls` | `header.language_version` |
| `spec` | `header.spec_name` |
| `version` | `header.spec_version` |
| import | `source_manifest.imports` |
| dimension/unit | `dimensions` / `units` |
| type/enum/record | `types` |
| operation/transport | `operations` / `transports` |
| domain/component/execution | `domains/components/execution_units` |
| event/channel | `events/channels` |
| state_machine | `state_machines` |
| rule | `rules` |
| constraint/safety | `constraints` 中带有互斥 `kind` 的节点 |
| acceptance | `acceptance` |
| intent/responsibility/rationale/decision | `knowledge_items`，分类为 `informative` |
| open/unknown | `knowledge_items`，分类为 `open` / `unknown` |

Surface AST 不作为跨工具公共契约；只有通过完整语义检查的 Canonical IR 可供 Agent 默认消费。

## 9. 尚未冻结

- Parser 技术和实现语言已由 `DEC-010` 决定为 Rust + LALRPOP 默认 LR(1)，不再属于未冻结事项；
- 标准库单位清单；
- Mutable Property/State Data 声明以及 `set` 赋值语义；
- 目标后端；
- 代码生成策略；
- 编辑器协议和格式化风格。
