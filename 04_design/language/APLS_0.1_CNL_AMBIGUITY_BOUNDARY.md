# APLS 0.1 受控自然语言歧义边界

- 设计 ID：`DES-APLS-CNL-AMB-001`
- 状态：`APPROVED_DESIGN_INPUT_UPDATED_BY_DEC_017_NOT_BASELINED`
- 日期：`2026-09-03`
- 输入：`DEC-004`、`DEC-014`、`DEC-015`、`DEC-017`、`ARCH-APLS-CNL-001`

> 本文冻结“什么必须拒绝”的候选边界，不把说明性例句冒充已批准 Grammar。

## 1. 可接受性公式

编译器必须保留并确定地处理所有完整分析候选。接受性只在规范语义收敛后判断：

```text
analyses          = all_complete_analyses(sentence)
valid_candidates  = semantic_bind_validate_and_normalize(analyses)
canonical_classes = structural_semantic_dedup(valid_candidates)

accepted(sentence)  iff |canonical_classes| == 1
invalid(sentence)   iff |canonical_classes| == 0
ambiguous(sentence) iff |canonical_classes| >= 2
```

`lexical_tokenization_count`、`grammar_parse_count` 或其他中间候选数允许大于 1。多个分析若最终得到结构与语义等价的同一 Canonical Frame，Source 必须接受；不得仅因中间多候选而拒绝。

每个有效候选仍必须满足：所有必需角色完整、每个引用绑定到一个 Symbol ID、每个值具有确定类型和单位、模态/否定/时间范围均已成为明确的 Frame 角色。违反这些规则的候选由规范规则淘汰，而不是由概率或实现顺序淘汰。

## 2. 可明确支持的等价表达

只有被当前语言 Profile 列入同一句式族的表达才能归一。例如，下列候选句可被设计为同一 `Rule` Frame：

```text
当水箱液位低于 20% 时，系统必须启动灌溉水泵。
如果水箱液位低于 20%，系统必须启动灌溉水泵。
水箱液位低于 20% 时，系统必须启动灌溉水泵。
```

实现必须通过同义句 Conformance Case 证明它们产生相同规范 Frame 和 Canonical Meaning。

## 3. 必须拒绝的歧义类别

| 类别 | 例子 | 拒绝原因 | 需要的修正 |
|---|---|---|---|
| 未定义术语 | “启动泵”但没有匹配声明 | 没有有效对象候选 | 使用已声明精确名称 |
| 最终对象多解 | “启动泵”绑定后仍指向不同设备 | 产生两个不等价 Canonical Frame | 使用已声明精确名称 |
| 模糊阈值 | “温度高时” | “高”无唯一数值边界 | 给出比较符、数值和单位 |
| 模糊变化量 | “适当降低一点” | 无唯一目标值/变化量 | 给出精确值或公式 |
| 所属不明 | “降低速度” | 速度属于哪个对象不唯一 | 显式对象和属性 |
| 指代不明 | “启动它” | 指代候选不唯一 | 重述精确术语 |
| 省略主体 | “必须停止” | 谁必须停止不明 | 显式主体 |
| 模态不明 | “最好停止” | 建议、义务或安全禁止不明 | 使用已定义模态 |
| 否定范围 | “不要在 A 或 B 时启动” | 否定作用域可多解 | 拆句或使用已批准结构 |
| 并列范围 | “停止 A 和 B 并报警” | 动作对象与模态共享范围不唯一 | 拆分为多个独立规范句 |
| 时间范围 | “很快停止” | 时限不可计算 | 给出确定时限 |
| 隐式单位 | “超过 20 时” | 量纲/单位不明 | 提供单位或有类型的百分比 |
| 时序不明 | “之后启动” | 参考事件或延迟不明 | 显式事件和时间关系 |
| 未冻结同义词 | “打开泵”与已声明“启动” | 运行时不能自动判断同义 | 声明别名或使用规范动作名 |
| 依赖常识 | “水少了就开泵” | “少”和“泵”依赖未声明领域知识 | 声明术语和精确阈值 |

## 4. 首版建议限制

为了使“自然”不以牺牲唯一语义为代价，首版建议：

- 每个规范性句子只表达一个主 Frame；
- 并列义务或禁止默认拆为多句；
- 不允许跨句省略规范性主体、对象、数值或单位；
- 不允许未冻结指代；
- 不允许程度副词改变规范语义；
- 不使用未声明的常识、因果或等价关系；
- 允许说明性文本，但必须与规范性句子机械区分，说明性文本不改变语义。

## 5. 诊断契约候选

每条歧义诊断至少包含：

- 稳定类别代码；
- 原句中最小多解或缺失范围；
- 缺失的 Semantic Role 或全部候选声明；
- 为什么无法形成唯一 Canonical Frame，或为什么没有有效 Frame；
- 一个或多个不会被自动采用的修改建议。

示例：

```text
APLS-CNL-INVALID-UNQUANTIFIED

无法形成有效规范解释：
“温度高的时候适当降低一点速度。”

未确定内容：
- “高”没有阈值和单位；
- “适当”没有可计算含义；
- “速度”没有唯一所属对象；
- 没有目标值或确定变化量。

本次编译已拒绝，未选择候选含义。
```

该示例的候选集最终为空，因此属于 `INVALID` 而不是 `AMBIGUOUS`。精确公共 Code 见 CNL Diagnostic Catalog；上述名称只是说明性类别名，不是公共代码。

## 6. AI 修改协议

```text
Compiler rejects Source
  -> emits structured ambiguity facts
  -> AI may explain or draft alternatives
  -> user chooses or edits
  -> a new Source revision is compiled from the beginning
```

AI 修改建议不能：

- 改变当前编译事务的 AST/Frame；
- 把未经用户选择的改写当作 Source；
- 将概率置信度当作形式唯一性证明；
- 在无声明时注入领域常识、默认阈值、单位或对象。

## 7. Conformance 最小类别

1. 每个合法句式族的正向样例；
2. 每组同义表面形式的 Frame/IR 等价样例；
3. 未定义术语与同名多候选；
4. 模糊量、单位和所属缺失；
5. 指代、否定、并列和时间范围多解；
6. 非法句子不能构造部分 Frame 或 IR；
7. 相同版本、Source 和术语图重复执行产生字节级一致结果。
8. 多 Tokenization/Parse 但最终 Frame 等价的正向收敛样例；
9. 两个及以上不等价 Canonical Frame 的 `AMBIGUOUS` 反向样例；
10. 候选枚举资源耗尽必须归类为 Tool Failure，不得冒充 `AMBIGUOUS`。
