# APLS 0.1 语言、编译器与 Canonical IR 总体架构

- 架构候选 ID：`ARCH-APLS-001`
- 状态：`CNL_CHANGE_INTEGRATED_NOT_BASELINED`
- 日期：`2026-09-02`
- 责任角色：`C02`
- 需求输入：`01_product_requirements/PRD.md`
- 当前公开表面语法决定：`DEC-014` / `HDP-APLS-010 Option A`
- 当前首版语言 Profile 决定：`DEC-015` / `HDP-APLS-011 Option A`（简体中文单语）
- 当前唯一性层级决定：`DEC-017` / `HDP-APLS-013 Option B`（最终 Canonical Frame 语义唯一）
- CNL 扩展架构：`ARCH-APLS-CNL-001`
- 适用范围：APLS 0.1

## 1. 架构目标

APLS 0.1 建立一条确定、可复现、失败关闭的规格编译路径：

```text
APLS Controlled Natural Language Source
  -> Controlled Analysis Candidate Generator
  -> Candidate Sentence Parser
  -> Candidate Semantic Binding and Type/Unit Checking
  -> Canonical Frame Convergence Gate
  -> Semantic Conflict Analysis
  -> Canonicalization
  -> Canonical IR
  -> Graph / Agent / Verification Consumers
```

核心不变量：

```text
同一 Source + 同一 Language Version + 同一 Compiler Version
=> 同一诊断集合，或同一 Canonical IR
```

如果最终无法证明 Canonical Frame 语义唯一，流水线必须停止，不生成“可能正确”的可消费 IR。中间词法切分、Parse Tree 或候选 Frame 可以不唯一。

## 2. 系统边界

### 2.1 确定性核心以内

- 词法分析与语法分析；
- 术语声明索引、Semantic Frame 构造与歧义完整性检查；
- 名称和作用域解析；
- 类型与工程单位检查；
- 状态机、规则、通信和约束冲突检查；
- 默认值显式化和规范化；
- Canonical IR 生成；
- 稳定诊断和 Source Map；
- 一致性测试接口。

### 2.2 确定性核心以外

- LLM 对错误原因的解释；
- 修复建议或补丁建议；
- 从自由自然语言生成受控 APLS 草稿；
- 从代码恢复 APLS 草稿；
- 代码生成与目标后端；
- 多智能体规划和软件自主演化。

核心外能力只能产生候选输入或建议，不能修改当前编译事务中的 AST、语义模型或 IR。

## 3. 两层语言模型

APLS 采用“表面语言 + 规范语义模型”分离：

### Surface Language

- 面向人类编写和评审，采用单一、版本化的受控自然语言 Profile；
- 具有正式 Unicode/术语/句子词法和版本化 Grammar；
- 每个规范性句子可以产生多个分析候选，但必须最终收敛为一个完整、类型化的 Canonical Frame 等价类；
- 可以保留与规范句机械分离的说明文本；
- 用户默认不查看花括号 DSL、AST 或 IR；Canonical JSON IR 是机器交换格式，不是第二个人类 Source 入口。

### Canonical Semantic Model

- 面向编译器、验证器和 Agent；
- 所有名称已绑定、类型已确定、默认值已展开；
- 不保留会改变语义的隐式信息；
- 采用稳定排序和版本化 Schema；
- 是特定 Source 与工具版本的派生事实，不取代源规格的事实所有权。

表面语言可以演进，但任何语法变化都必须证明其映射到 Canonical Semantic Model 后仍然唯一。

## 4. 编译阶段与失败关闭点

| 阶段 | 输入 | 成功输出 | 必须拒绝的情况 |
|---|---|---|---|
| Candidate Lexical Analyzer | Unicode 字符流 | Tokenization Candidate Lattice/句子边界 | 非法字符、候选无法完整表示、未冻结标点/正规化 |
| Terminology Indexer | 术语声明 | 精确术语图 | 术语重复、别名冲突、类别缺失 |
| Sentence Parser | Tokenization Candidates 和术语图 | Parse Candidate Forest | 无完整 Parse、非法恢复、候选处理不完整 |
| Binder / Type Checker | Parse Candidate Forest | 有效 Bound Frame Candidates | 未定义、重复、类别、类型、单位或角色错误 |
| Canonical Frame Convergence Gate | Bound Frame Candidates | 唯一 Canonical Frame 等价类 | 零有效 Frame，或两个及以上不等价 Canonical Frame |
| Semantic Analyzer | Typed Model | Checked Model | 状态、规则、通信、约束或默认值冲突 |
| Canonicalizer | Checked Model | Canonical IR | 未展开默认值、非确定排序、缺失稳定身份 |
| IR Validator | Canonical IR | Verified IR | Schema、引用、版本或不变量失效 |

任一 Error 诊断存在时，不得向下游发布 Verified IR。调试用途的部分 AST 必须显式标记 `invalid`，且不能被 Agent 当作规范事实消费。

## 5. 歧义防火墙

### 5.1 词法候选

- 关键字、标识符、数值、字符串、时间和工程单位的全部合法候选必须由 Profile 确定；
- 不得使用最长匹配、声明顺序或概率静默删除合法候选；
- Unicode 规范化策略必须显式，视觉相似字符不得静默合并。

### 5.2 语法候选

- 正式 Grammar 不允许 shift/reduce 或 reduce/reduce 冲突被实现工具静默解决；
- 运算符优先级和结合性必须由规范定义；
- 错误恢复只用于产生诊断，不得改变合法程序语义。
- 多条中间 Parse 路径允许保留；它们只有在最终 Canonical Frame 不收敛时才构成 Source 歧义。

### 5.3 名称歧义

- 每个声明拥有唯一 Fully Qualified Name；
- 导入、别名、遮蔽和作用域搜索顺序必须显式；
- 多个候选名称必须全部进入确定性绑定；若最终收敛到同一 Symbol ID 和 Canonical Frame 则接受，否则拒绝，不选择“最近似”名称；
- 大小写和 Unicode 比较规则必须固定。

### 5.4 类型与单位歧义

- 禁止未定义的隐式窄化和跨维度单位转换；
- 数值字面量的类型推断规则必须唯一；
- 单位换算必须产生显式规范值和原始表示溯源；
- 无法唯一推断类型时要求显式标注。

### 5.5 行为与状态歧义

- 同一状态、同一事件下可能同时成立的转换必须证明 Guard 互斥，或声明规范规定的显式优先级；
- 多条 Rule 对同一目标产生冲突动作时必须显式组合，否则报错；
- 未定义的事件排序、消息丢失、重试和超时语义不能由后端自行决定；
- Safety 与普通 Rule 冲突时不能依赖声明顺序解决。

### 5.6 自由自然语言边界

- 符合 `apls-zh-CN-0.1` 的受控自然语言可以作为规范 Source；未被 Profile 接受的自由自然语言只进入 `informative` 或 `rationale` 类说明字段；`unknown` 与 `open` 内容不进入 Verified IR，由稳定诊断在编译期拒绝并定位；
- 自由自然语言不能充当类型、条件、目标引用、时间约束或状态转换的规范定义；
- 编译器不得调用 LLM 把说明文本转换成规范语义；
- AI 建议必须形成新的显式 Source 变更后重新编译。

## 6. Canonical IR 契约

### 6.1 顶层结构

Canonical IR 至少包含：

```text
header
source_manifest
symbols
types
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

### 6.2 Header

必须记录：

- `language_version`；
- `ir_schema_version`；
- `compiler_id` 和 `compiler_version`；
- 源文件清单及内容摘要；
- Canonicalization Profile；
- 生成状态 `verified`。

### 6.3 规范性分类

Verified IR 机械区分 `normative` 与 `informative` 两类；`unknown` 与 `open` 内容不进入 Verified IR，由稳定诊断在编译期拒绝并定位（与 PRD-007 修订后验收一致，`DEC-031` / `HDP-APLS-024` Q2=B）：

| 分类 | 是否影响语义 | 编译规则 |
|---|---:|---|
| `normative` | 是 | 必须结构化、类型正确且可验证 |
| `informative` | 否 | 可保留文本，但不能改变规范结果 |
| `unknown` | 否，表示知识缺失 | 不进入 IR；编译期以稳定诊断拒绝并定位 |
| `open` | 否，表示待决事项 | 不进入 IR；编译期以稳定诊断拒绝并定位 |

### 6.4 身份、引用与排序

- 声明身份来自语言规定的 Fully Qualified Name，不由文件遍历顺序决定；
- 所有引用在 IR 中使用已解析的稳定身份；
- Map Key、集合和声明组采用规范规定的稳定排序；
- 有业务语义的顺序必须显式保存，不能为排序而破坏语义；
- 禁止使用内存地址、线程调度、随机 UUID 或系统时间影响 IR 内容。

### 6.5 默认值

- 默认值只能来自对应语言版本的规范；
- Canonical IR 必须写出所有规范默认值；
- 实现私有默认值禁止进入 Verified IR；
- 默认值发生变化必须触发语言版本或兼容性规则评估。

### 6.6 溯源

- 每个规范节点必须可映射回源文件与源码范围；
- 派生值记录 derivation 类型，不复制自然语言推理链；
- IR 必须绑定源摘要和编译器版本，避免派生物脱离来源成为竞争事实。

## 7. 诊断架构

每条诊断至少包含：

```text
code
severity
message
primary_source_span
related_source_spans
normative_rule_reference
optional_fix_suggestion
```

建议的稳定分类：

| 范围 | 类别 |
|---|---|
| `APLS-E1xxx` | 词法与语法 |
| `APLS-E2xxx` | 名称、作用域与导入 |
| `APLS-E3xxx` | 类型与单位 |
| `APLS-E4xxx` | 状态、规则与语义冲突 |
| `APLS-E5xxx` | Canonical IR 与版本 |
| `APLS-W8xxx` | 不影响规范语义的警告 |

错误消息可以改进措辞，但诊断代码的含义在兼容版本内不得漂移。

## 8. 消费者边界

### Agent Consumer

- 默认只消费 `verified` Canonical IR；
- 可以读取 `informative` 类说明内容，但必须保留其分类；`unknown` 与 `open` 内容不进入 Verified IR（编译期已以稳定诊断拒绝），不存在可消费实例；
- 不得把建议或推断写回规范事实，除非经过新的受控 Source 变更。

### Graph Consumer

- 从 IR 派生 Execution、Communication 和 State Graph；
- Graph 是 View，不是新的事实 Owner；
- 图节点必须保留稳定身份和 Source Map。

### Verification Consumer

- 从 Constraint、Safety 和 Acceptance 生成验证输入或测试骨架；
- 生成结果不能证明自身正确；
- 具体验证策略在 C05 阶段定义。

### Backend Consumer

- 目标后端不是 0.1 核心成功条件；
- 后端不得为适配平台重新解释未定义语义；
- 遇到无法映射的规范结构必须拒绝并诊断。

## 9. LLM 辅助边界

允许：

```text
Diagnostic + Source + Verified IR
  -> LLM Explanation / Patch Suggestion
  -> Human or Governed Agent Approval
  -> New Source Revision
  -> Full Recompile
```

禁止：

```text
Ambiguous Source
  -> LLM Guess
  -> Silent AST/IR Mutation
  -> Verified IR
```

任何 AI 生成内容在进入确定性编译器前都只是候选文本。

## 10. 版本与兼容性

- Source 必须声明或可唯一确定 `language_version`；
- IR 必须声明 `ir_schema_version`；
- Compiler 必须声明支持的版本范围；
- 未知规范字段默认报错，不能静默忽略；
- 扩展机制必须定义命名空间、是否规范性以及不支持时的行为；
- Canonicalization 变化必须视为兼容性事件并提供迁移说明。

## 11. 需求覆盖

| 需求 | 架构响应 |
|---|---|
| PRD-001 | 完整候选处理、Canonical Frame 收敛唯一、Parser 工具冲突禁止 |
| PRD-002 | 六类歧义防火墙与失败关闭流水线 |
| PRD-003 | Canonicalization 契约、稳定身份、排序和默认值展开 |
| PRD-004 | Resolver、Type/Unit Checker、引用完整性 |
| PRD-005 | Semantic Analyzer、状态与规则冲突检查 |
| PRD-006 | 稳定诊断分类和 Source Span |
| PRD-007 | Verified IR、规范性分类和 Agent Consumer 边界 |
| PRD-008 | Parse、Check、Emit IR、Diagnose 四类核心能力 |

## 12. 关键故障模式

| 故障 | 架构控制 |
|---|---|
| 不同中间分析得到不同 AST | 完整候选遍历 + Canonical Frame 等价类收敛 Conformance Cases |
| 名称解析随文件顺序变化 | 显式作用域 + 稳定 Fully Qualified Name |
| Hash/IR 随运行变化 | 禁止随机/时间/地址输入 + 规范排序 |
| 自然语言改变规范语义 | informative 与 normative 物理分区 |
| AI 自动“修好”错误输入 | AI 在核心外，变更后必须完整重编译 |
| 后端各自解释未定义行为 | 映射失败即拒绝 |
| 派生 IR 脱离来源 | Source Manifest + 摘要 + Source Map |

## 13. 当前未决架构问题

- `Q-001`：已关闭，采用 Option A（单一 `.apls` DSL + Canonical JSON IR）；
- `Q-002`：已由 `HDP-APLS-006 Option A` / `DEC-010` 关闭，选择 Rust + LALRPOP 默认 LR(1)；
- `Q-003`：首个参考后端。

这些问题不影响实现语言无关的 Compiler 契约设计。`Q-002` 已在编码前裁定；`Q-003` 必须在参考后端设计前裁定。

## 14. 下一阶段产物

若本架构候选获批，下一工作包依次形成：

1. APLS 0.1 词法与 EBNF/PEG 语法候选；
2. 名称、类型、单位和作用域规范；
3. Canonical IR Schema；
4. 诊断目录；
5. Compiler MVP 详细设计；
6. Conformance 测试设计。

本文件不授权选择实现语言、开始编码、安装依赖或采用产品 Baseline。
