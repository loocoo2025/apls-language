# APLS 0.1 Canonical JSON IR 详细设计

- 设计 ID：`DES-APLS-IR-001`
- 状态：`LEGACY_DSL_ONLY_BY_DEC_019`
- Schema：`apls-ir-0.1.schema.json`
- Canonical Byte 补充决定：`HDP-APLS-007 Option A`、`DEC-011`
- 名称与 Action 闭合决定：`HDP-APLS-009 Option A`、`DEC-013`

> 本文件及 `apls-ir-0.1.schema.json` 仅保留为旧 DSL 迁移参考。当前 CNL 机器契约是 `APLS_0.1_CNL_CANONICAL_IR.md` / `apls-cnl-ir-0.1.schema.json`；不得把本文件实现为 CNL Publisher，也不得向用户暴露为第二公开入口。

## 1. 角色

Canonical IR 是通过完整检查后的派生语义模型，用于 Agent、图生成器和验证器。它不是第二份手写规格，也不能脱离 Source Manifest 成为独立 Current Truth。

只有满足以下条件的 IR 才能标记 `verified`：

- Parser 无 Error；
- 名称、类型和单位全部解析；
- 状态、规则、通信和约束冲突全部关闭；
- 所有默认值已经展开；
- Schema 和本文件的不变量全部满足。

## 2. Canonicalization Profile 0.1

- JSON 编码 UTF-8，无 BOM；
- 顶层 `}` 是 Canonical IR 的最后一个 Byte，不追加 LF；
- 序列化不输出非必要空白；对字符串只转义 `"`、`\\` 和 U+0000～U+001F，不转义 `/` 或其他 Unicode Scalar Value；无效 Unicode 代理项失败；
- 控制字符中 U+0008/U+0009/U+000A/U+000C/U+000D 分别使用 `\b/\t/\n/\f/\r`，其余使用小写四位 `\u00xx`；
- Object Key 输出时按 Unicode Code Point 升序；
- 声明集合按 `id` 升序；Record Field、Operation Parameter 和命名 Binding 按 `name/id` 升序；Enum Value、`supports` 和 `prohibited_refs` 按字符串升序；
- 有语义顺序的动作数组保持 Source 顺序，并显式记录 `ordinal`；List Literal 保持元素顺序；
- 十进制值使用规范十进制字符串：无正号、无前导零、无末尾小数零、无负零，小数部为空时省略小数点；不使用 JSON 浮点数；
- 工程量同时记录规范值、维度和规范单位；
- 禁止时间戳、随机 UUID、内存地址或遍历顺序进入语义内容；
- Fully Qualified Name 是稳定 ID 的基础；匿名规范节点禁止进入 0.1 Verified IR。

## 3. 顶层对象

```text
header
source_manifest
symbols
dimensions
units
types
domains
components
operations
transports
execution_units
events
channels
state_machines
rules
constraints
acceptance
knowledge_items
source_map
```

Schema 负责结构类型；以下跨节点规则由 IR Validator 负责：

- 所有 `id/ref` 必须指向唯一且类别兼容的符号；
- 每个 ID 在全局符号空间唯一；
- 所有数组满足 Canonicalization Profile 排序；
- State Machine 的 initial、from、to 必须属于本机；
- Expression 类型已经确定且与使用位置兼容；
- Engineering Quantity 的 dimension/unit/value 一致；
- Base Unit 的 scale/offset 为 `1/0`，其他 Unit 的 scale 大于零；
- Pure Operation 必须有 Return Type，Action Operation 的 Return Type 必须为 `null`；
- Periodic Schedule 只有一个大于零的 Time `period`，其他 Schedule 的 Parameter Map 为空；
- `emit/invoke/transition` 的引用类别与参数必须匹配已解析的目标契约；
- `open/unknown` 不得被 normative 节点依赖；
- Source Map 以 Canonical JSON Pointer 为 Key，覆盖每个 normative 声明以及可定位的表达式/动作节点。

`dimensions/units/types/domains/components/operations/transports` 保留表面声明中会影响规范语义的结构字段；`responsibility/rationale/decision` 等说明文本只进入 `knowledge_items`。

Schema 中只有命名参数 Map、Schedule Parameter Map 和 Source Map 允许动态 Key；它们分别必须与已解析 Operation/Event 签名、已定义 Schedule 参数集和 Canonical JSON Pointer 匹配，不是扩展字段。

## 4. 表达式

表达式只使用以下规范节点：

- `literal`；
- `list`；
- `reference`；
- `unary`；
- `binary`；
- `call`。

每个表达式必须记录唯一的 `result_type`。调用只能引用已解析的纯操作；动作不能出现在表达式树中。

Enum Variant 在 Resolver 中具有 `<spec>.<enum>.<variant>` 内部语义身份，但不单独进入 `symbols`；Canonicalizer 将其输出为具有该 Enum Named Type `result_type` 和 Variant 名字符串 `value` 的 Literal Expression。

Type Reference 不使用可再解析的类型字符串，而是 `primitive/named/list/optional/quantity` 五种互斥结构。因此下游工具不需要二次解析 `list<Foo>` 一类文本。

## 5. 动作

IR 动作是互斥的 `emit/invoke/transition` 节点，并保留 Source 顺序的 `ordinal`。`emit/invoke` 显式携带命名参数，`transition` 分开记录 State Machine 和目标 State；不存在依赖 `kind` 二次解释的通用参数袋。0.1 不产生 `set` IR 节点。

## 6. 工程量

Canonical Quantity：

```json
{
  "kind": "literal",
  "result_type": {
    "kind": "quantity",
    "dimension_ref": "Fertigation.Time",
    "unit_ref": "Fertigation.second"
  },
  "value": "0.1"
}
```

`value` 是相对于维度 Base Unit 的规范十进制字符串，`unit_ref` 固定指向该 Base Unit。原始写法由 Source Map 记录，不参与相等性判断。

## 7. 知识分类

- `normative` 内容进入类型化主体节点；
- `informative/open/unknown` 进入 `knowledge_items`；
- `knowledge_items` 只能通过 `subject_ref` 关联规范节点，不能反向改变其语义；
- 未决或未知项若阻断编译完整性，IR 不得标记 `verified`。

## 8. 哈希与等价性

语义哈希的输入是：按 Canonicalization Profile 序列化、排除非语义展示字段后的 IR。Source 路径显示形式、说明文本和 Source Span 不参与语义等价，但 Source 内容摘要必须保留用于溯源。

0.1 候选暂不冻结具体哈希算法；实现前必须通过 ADR 选择并版本化。

## 9. 失败规则

- 未知 JSON 字段：失败；
- 缺少 Required 字段：失败；
- Schema Version 不支持：失败；
- ID 未解析或重复：失败；
- 数组非规范排序：失败；
- Error 诊断存在却声明 `verified`：失败；
- Compiler 私有扩展未放入正式命名空间：失败。
