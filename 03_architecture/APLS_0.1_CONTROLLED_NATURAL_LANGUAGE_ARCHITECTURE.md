# APLS 0.1 受控自然语言架构

- 架构 ID：`ARCH-APLS-CNL-001`
- 状态：`UPDATED_BY_DEC_017_NOT_BASELINED`
- 日期：`2026-09-03`
- 输入：`DEC-004`、`DEC-014`、`DEC-015`、`DEC-017`、`CR-APLS-001`、`IA-APLS-001`
- 适用范围：APLS 0.1 公开 Surface 和确定性 Compiler Frontend

> 本文是新方向架构候选，不是 Baseline，不授权代码重写。

## 1. 产品定位

APLS 不再是“接近自然语言的 DSL”，而是：

> 一种以受控自然语言为唯一公开表面语法、以唯一形式语义和 Canonical IR 为底层契约、面向人类和 AI 的可编译自然语言。

用户使用“声明对象、描述规则、规定禁止、说明状态变化、给出验收条件”的方式编程，不要求学习花括号、分号、FQN 或 JSON IR。

## 2. 不可破坏的语义不变量

```text
Many approved surface forms  -> One Canonical Frame equivalence class -> ACCEPT
Many intermediate analyses  -> One Canonical Frame equivalence class -> ACCEPT
One surface form            -> Two or more inequivalent Canonical Frames -> AMBIGUOUS / REJECT
One surface form            -> Zero valid Canonical Frames              -> INVALID / REJECT
```

- 合法 Source 中每个规范性句子最终必须得到恰好一个完整、类型化的 Canonical Frame 等价类；词法切分、Parse Tree 和候选 Frame 可以不唯一；
- 所有术语、引用、单位、模态、否定和时间范围必须可机械确定；
- 解析不读取模型温度、会话记忆、用户画像、未声明领域常识或“最可能含义”；
- 只有 Verified Canonical IR 是 Agent 默认消费的机器契约；Surface AST 和内部 Frame 均非公共契约。

## 3. 用户界面与内部产物

| 层 | 默认可见性 | 用途 |
|---|---|---|
| 受控自然语言 Source | 用户默认可见、可编辑 | 规范性源文件 |
| 结构化诊断 | 失败时可见 | 指出缺失概念、多解范围和可操作改写方向 |
| Normalized Meaning / Explain View | 只在用户主动请求时可见 | 查看编译器证明的唯一含义 |
| Surface AST / Semantic Frame | 内部 | Parser、Resolver 和 Checker 阶段产物 |
| Canonical IR | 默认供 Agent/工具消费；人类只在主动请求时查看 | 稳定机器契约 |

任何 CLI、IDE 或 Agent 产品不得把内部 DSL/AST/IR 暴露作为完成基本任务的必经步骤。

## 4. 确定性编译流水线

```text
CNL Source Bytes
  -> Unicode and Sentence Boundary Validation
  -> Terminology Declaration Index
  -> Controlled Analysis Candidate Lattice / Forest
  -> Candidate Sentence Parsing
  -> Candidate Semantic Binding and Type/Unit Checking
  -> Candidate Canonicalization
  -> Canonical Frame Convergence Gate
  -> Semantic Conflict Analysis
  -> Canonical IR Validation
  -> Verified IR
```

中间分析阶段可以产生确定、有限、可完整遍历的候选集合。只有 Convergence Gate 后的成功 Artifact 必须唯一；上游存在致命 Error 或候选集合无法完整处理时，不得构造下游规范模型。

### 4.1 Controlled Lexical Analysis

- Source 固定为 UTF-8，Unicode 规范化、标点和空白等价性必须由语言 Profile 显式定义；
- 中文分词不使用概率分词器决定规范语义；
- 声明中的术语使用明确定界符建立精确词典；
- 正文术语保持自然书写时，Indexer 必须保留所有合法切分候选；重叠或切分多解只有在最终产生两个及以上不等价 Canonical Frame 时才拒绝，并允许用户用定界符或补充句子显式消歧。

### 4.2 Deterministic Sentence Parsing

语言 Profile 以版本化 Grammar 列出可接受的句式族。“当……时”、“如果……”和其他等价表达只有在 Grammar 明确定义同一 Frame 映射时才等价，不由运行时 AI 判断同义。

Parser 必须完整表示或枚举当前 Profile 允许的分析候选，不得选择第一个成功 Parse。Parser Generator 的零冲突 Gate 约束单条候选 Token Stream 的 Grammar 实现质量，不构成 Source 必须只有一个中间 Parse 的语言规则。

### 4.3 Typed Semantic Frames

首版至少需要以下 Frame 族：

| Frame | 核心角色 |
|---|---|
| `TermDeclaration` | 术语、类别、类型、所属与精确名称 |
| `PropertyDeclaration` | 对象、属性、值类型、单位/范围和可观测/可写性 |
| `ActionDeclaration` | 主体、动作、对象、参数与效果类别 |
| `Rule` | 条件、模态、主体、动作和时间范围 |
| `Prohibition` | 触发条件、禁止动作和适用范围 |
| `StateDeclaration` | 主体、状态集和初始状态 |
| `Transition` | 源状态、触发、Guard、目标状态和动作 |
| `Invariant` | 主体、适用范围和必须始终成立的断言 |
| `Acceptance` | 前置、触发、预期和时限 |

每个仍有效的候选 Frame 不得跳过必需角色。例如“适当降低一点速度”无法提供确定对象、变化量和目标值，所有候选都会失效，因而以 `INVALID` 失败。

## 5. 术语和名称模型

- 规范性概念必须先通过受控自然语言声明，不从领域常识隐式注入；
- 每个概念拥有与显示文本分离的稳定语义 ID；
- 同义词、缩写和别名必须显式声明或由语言 Profile 版本化提供；
- 未声明同义词不做近似匹配；
- 最终零个有效 Canonical Frame 时以非法输入失败；两个及以上不等价 Canonical Frame 时以 `AMBIGUOUS` 失败；中间多个名称候选可继续绑定并收敛；
- “它”、“该设备”、“前者”等指代只有在将来冻结了唯一指代规则后才能进入规范性句子；首版默认拒绝。

## 6. AI 辅助边界

AI Authoring Assistant 位于规范编译事务之外：

```text
Free Natural Language
  -> AI proposes Controlled Natural Language draft
  -> Human accepts or edits draft
  -> Deterministic APLS Compiler
  -> Accepted or Rejected
```

- AI 可以解释诊断、给出多个可选改写、补全模板或反问缺失信息；
- AI 不得在当前编译事务中静默改源文、选择候选含义或把概率结果标记为 Verified；
- 任何 AI 修订都产生新 Source，并重新经过全部确定性阶段。

## 7. 说明性候选语句

> 以下只说明目标体验，不是已批准 Grammar。

```text
“水箱液位”是百分比类型的可观测属性。
“灌溉水泵”是设备。
“启动”是“灌溉水泵”支持的动作。

当水箱液位低于 20% 时，系统必须启动灌溉水泵。
急停处于激活状态时，系统禁止启动灌溉水泵。
当传感器状态无效时，系统不得发布该传感器的输出数据，并且必须产生传感器故障。
```

最后一句含有“该传感器”和并列义务；除非 Profile 已精确冻结指代与并列范围，否则首版编译器必须拒绝并要求拆分或显式重述对象。

## 8. 迁移原则

- 旧 `.apls` DSL 不再是公开规范性 Source；
- 旧 Lexer/Parser/AST 保留作为行为参考，不直接宣称符合 CNL；
- Canonical IR 只保留能与新 Semantic Frame 完整、唯一映射的部分；
- 新 Frontend 获批前，禁止一边设计一边重写代码；
- 不提供两条同时规范性的 Source 路径，避免 DSL 与 CNL 对同一语义产生竞争事实。

## 9. 架构验收条件

- 用户可仅使用受控自然语言完成概念、规则、状态、安全和验收的首版规范；
- 无需查看 DSL、AST 或 IR 即可完成正常工作流；
- 每个合法句子最终只产生一个 Canonical Frame 等价类；中间多分析候选的正向样例必须能够通过；
- 每个已批准等价句组产生相同 Canonical Meaning；
- 未定义术语、模糊量、多指代、并列范围多解和否定范围多解均失败关闭；
- 不使用 LLM 作为合法 Source 的解析或含义裁决器；
- 不使用任意优先级、最高概率或第一个 Parse 制造唯一性；
- Explain/Inspect 只由用户主动请求触发。

## 10. 待裁决项

1. `DEC-015` 已决定首版采用简体中文单语 Profile；
2. 实现语法在 CNL Grammar 完成后是否继续使用 LALRPOP，必须由零冲突与可诊断性证据决定；
3. 旧 DSL 是仅保留历史参考，还是提供一次性、非规范性迁移器。

第 2/3 项不阻塞简体中文 Profile 详细设计；在代码迁移前必须关闭 Parser 技术适配问题。
