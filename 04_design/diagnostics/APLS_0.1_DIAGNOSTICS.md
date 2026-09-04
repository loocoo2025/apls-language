# APLS 0.1 诊断目录

- 设计 ID：`DES-APLS-DIAG-001`
- 状态：`CNL_EXTENSION_REQUIRED_BY_DEC-014`
- 适用范围：APLS Source 到 Verified Canonical IR

> 现有稳定编号和失败关闭原则保留为复用候选；`APLS-E1xxx` 中依赖旧 ASCII DSL 的具体根因尚不能代表 CNL 接受集。CNL 术语切分、指代、量化、角色缺失和一句多解诊断须在 `HDP-APLS-011` 后另行冻结。

当前 CNL 扩展见 `APLS_0.1_CNL_DIAGNOSTICS.md`，已由 `HDP-APLS-012` 批准为设计输入，并由 `DEC-017` 冻结最终 Canonical Frame 唯一的触发层级。下表 `APLS-E1102` 仅保留旧 DSL 根因身份，CNL 不因中间 Tokenization/Parse 多候选发出该 Code。

## 1. 诊断契约

每条诊断必须包含：

```text
code
severity
message
primary_source_span
related_source_spans
normative_rule_reference
optional_fix_suggestion
```

- `severity` 只能是 `error` 或 `warning`。
- `primary_source_span` 包含文件、起止行与起止列，并指向能表达根因的最小连续区间。
- 多个声明共同造成冲突时，主要区间指向最后触发检查的位置，其他位置进入 `related_source_spans`。
- `normative_rule_reference` 引用稳定的规范章节或规则 ID，不引用可变的错误消息文本。
- 修复建议只是候选文本；编译器不得自动修改 Source 并继续生成 Verified IR。

Error 存在时必须返回失败，且不发布 Verified IR。Warning 不得表示任何会改变规范语义的问题。

## 2. 稳定编号空间

| 范围 | 类别 | 稳定性 |
|---|---|---|
| `APLS-E1xxx` | 词法与语法 | 同一主版本内含义不漂移 |
| `APLS-E2xxx` | 名称、作用域与导入 | 同上 |
| `APLS-E3xxx` | 类型与工程单位 | 同上 |
| `APLS-E4xxx` | 状态、规则、通信与语义冲突 | 同上 |
| `APLS-E5xxx` | Canonical IR、规范化与版本 | 同上 |
| `APLS-W8xxx` | 不影响规范语义的警告 | 可被用户策略升级为 Error |

诊断文案可改善，但一个旧 Code 不得改表示其他根因；合并或拆分根因时必须分配新 Code。同一 Compiler Version、Language Version 和 Source 必须产生相同诊断集。公共排序唯一采用 `DES-APLS-CNL-DIAG-001` 第 1.2 节的 Byte-based 总排序键；此前 `(file,start,code,related_spans)` 仅为历史简写，不能作为另一套实现选项。本候选同步需由 `HDP-APLS-018` 采用。

## 3. 规范诊断

| Code | 根因 | 最小主要范围 |
|---|---|---|
| `APLS-E1001` | 非法字符、BOM 或不允许的 Unicode 标识符 | 非法字节序列 |
| `APLS-E1002` | 字符串或块注释未闭合，或字符串转义无效 | 开始定界符至文件末/错误转义 |
| `APLS-E1003` | 数字格式非法，包括前导零或指数形式 | 整个数字文本 |
| `APLS-E1101` | 当前 Token 不符合 Grammar | 意外 Token；缺失 Token 时为插入点 |
| `APLS-E1102` | Grammar 存在多个合法解析或 Parser 冲突 | 最小分歧输入 |
| `APLS-E1103` | 必需结构缺失、顺序非法或同一块的单次字段重复 | 缺失点或重复字段 |
| `APLS-E2001` | 名称未定义 | 引用名 |
| `APLS-E2002` | 同一作用域重复定义或导入别名重复 | 后一定义；首一定义为 Related |
| `APLS-E2003` | 名称解析得到多个候选 | 引用名；所有候选为 Related |
| `APLS-E2004` | 导入循环 | 闭合循环的 import；其他边为 Related |
| `APLS-E2005` | 引用符号类别不匹配 | 引用名 |
| `APLS-E3001` | 表达式、参数、字段或返回类型不匹配 | 实际值 |
| `APLS-E3002` | 类型无法唯一推断 | 需要显式类型的表达式 |
| `APLS-E3003` | 工程量维度不匹配 | 不兼容的操作数 |
| `APLS-E3004` | 非法隐式转换、单位换算不唯一或不可表示 | 转换位置 |
| `APLS-E3005` | Operation 的 `pure/action` 种类与调用位置不兼容 | call 或 invoke 目标 |
| `APLS-E4001` | 同一 State/Event 的 Transition 可重叠且没有唯一优先级 | 后一 Transition；冲突项为 Related |
| `APLS-E4002` | 可同时触发的 Rule 对同一目标产生冲突写入 | 后一动作；冲突动作为 Related |
| `APLS-E4003` | Safety 与 Rule、Transition 或其他 Safety 冲突 | Safety 条件/动作；冲突项为 Related |
| `APLS-E4004` | normative 节点依赖 `open` 或 `unknown` | 依赖位置；知识项为 Related |
| `APLS-E4005` | 未知字段、关键字或未声明扩展 | 未知内容 |
| `APLS-E4006` | Channel 的端点、Payload、Transport、Delivery 或时序契约不一致 | 不一致字段 |
| `APLS-E5001` | IR 不符合声明的 JSON Schema | 对应 JSON Pointer |
| `APLS-E5002` | IR 存在重复、未解析或类别不兼容的 ID/Ref | 对应 Ref；候选/原定义为 Related |
| `APLS-E5003` | 数组、Object Key 或有序动作不符合 Canonicalization Profile | 首个非规范位置 |
| `APLS-E5004` | 有 Error 或未展开默认值却声明 `verified` | `header.status` |
| `APLS-E5005` | Language、IR Schema 或 Canonicalization Profile 版本不支持 | 版本字段 |
| `APLS-E5006` | normative 节点缺失 Source Map 或 Source 摘要不匹配 | 对应节点/Manifest 项 |
| `APLS-W8001` | 未被规范节点关联的 informative 内容 | 说明项 |
| `APLS-W8002` | 不影响语义的弃用声明 | 声明名 |

`DEC-013` 从 APLS 0.1 移除了 `set` 动作；因此 `APLS-E4002` 在当前 0.1 接受集中没有可达的发生源。该 Code 保留原有根因身份且不得复用；只有未来获批准的 Mutable Property/State Data 模型恢复可写目标后才可再次发出。

## 4. 失败关闭与去重

- 同一根因在同一主要范围只发出一条诊断。
- 后续阶段不对上游已失败节点伪造派生错误；可用内部 Poison 标记抑制级联，但不得产生部分 Verified IR。
- Parser 错误恢复的节点一律不进入名称、类型或语义判定。
- 如果检查器无法证明唯一性或互斥性，发出对应 Error，不得降级为 Warning。
