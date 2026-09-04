# APLS 0.1 CNL Frontend 技术适配设计

- 设计 ID：`DES-APLS-CNL-FRONTEND-001`
- 状态：`TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018`
- 日期：`2026-09-03`
- 输入：`DEC-010`、`DEC-011`、`DEC-014`～`DEC-018`、`DEC-021`、`TASK-015`、`DES-APLS-CNL-SEMVAL-001`、`DES-APLS-CNL-RESOURCE-001`、`DES-APLS-CNL-UNICODE-001`
- 适用范围：`apls-zh-CN-0.1` 从 Source 到唯一 Canonical Frame 等价类

> 本文只形成实现路线候选，不修改 Compiler Source、依赖或 Baseline。

## 1. 结论

推荐保留 Rust + LALRPOP，但重新界定 LALRPOP 的职责：它解析一条确定的候选 Token Stream，不负责通过冲突消解选择自然语言含义。在其前面增加确定性的候选词法层，在其后面增加语义绑定和 Canonical Frame 收敛 Gate。

```text
Source Sentence
  -> Candidate Lexical Lattice
  -> Complete Token-Stream Enumerator
  -> LALRPOP parse once per candidate stream
  -> Parse Candidate Forest
  -> Deterministic Binding + Type/Unit Validation
  -> Candidate Frame Normalization
  -> Canonical Frame Equivalence Classes
       0 -> INVALID
       1 -> ACCEPTED
      >1 -> AMBIGUOUS / REJECTED
```

这条路线同时保持 `DEC-010` 的 Rust/LALRPOP 决定和 `DEC-017` 的最终语义唯一规则，不需要把中间多候选错误地解释为 Grammar 冲突。

## 2. 组件边界

| 组件 | 输入 | 输出 | 规范责任 |
|---|---|---|---|
| Unicode/Sentence Validator | Source Bytes | Validated Sentences | UTF-8、Unicode 17.0.0 NFC、标点、空白和 Byte Span；按 `DES-APLS-CNL-UNICODE-001` |
| Declaration Bootstrap | 全部 Profile 后 Sentence | Declaration Syntax Candidates / Deferred Sentences | 不查 Terminology Graph；以 `DECLARED_TERM` 和固定字面量构造完整流，调用唯一 Grammar 的声明 Entry；失败不直接拒绝 |
| Declaration Binder | 全部 Declaration Syntax Candidates | Frozen Terminology Graph 或直接声明错误 | 先收集所有 Provisional Header，再统一解析类别、Owner、别名和前向引用；不处理行为含义 |
| Candidate Lexical Analyzer | 句子 + Terminology Graph | Tokenization Lattice | 保留全部 Profile 合法切分，不使用概率或最长匹配 |
| Stable Enumerator | Tokenization Lattice | 完整 Token Stream 序列 | 以稳定顺序枚举，但顺序不具有选择语义 |
| LALRPOP Sentence Parser | 单条 Token Stream | 0 或 1 个 Parser AST；所有流共同形成 Parse Candidate Forest | 使用唯一正式 Grammar；生成冲突必须为零；不选择自然语言含义 |
| Candidate Binder | Parse Candidate + Terminology Graph | Bound Frame Candidate 或确定失败 | 绑定 Symbol、Owner、Action/Target 和 State |
| Type/Unit Validator | Bound Candidate | Typed Frame Candidate 或确定失败 | 按 `DES-APLS-CNL-SEMVAL-001` 关闭类型、单位、必需角色、合取与类别错误 |
| Frame Normalizer | Typed Candidate | Canonical Frame Candidate | 规范模态、比较符、单位、顺序和语义 ID |
| Convergence Gate | 全部 Canonical Frame Candidates | 唯一等价类或拒绝 | 结构语义去重，并执行 `0/1/>1` 契约 |

所有候选集合都是内部 Artifact。用户默认只看到 Source、最终结果和必要诊断。

## 3. “完整候选”要求

- Declaration Bootstrap 必须严格采用 `DES-APLS-CNL-RESOURCE-001` 第 2.1 节：带引号名称在声明解析时是无类别 `DECLARED_TERM`，全部声明 Root 收集后才绑定；它和 Normative Pass 共用 `GRAM-APLS-ZH-CNL-001`，不得另建声明 Parser 或不计数预扫描；
- Candidate Lexical Lattice 必须表示当前 Profile 下全部合法覆盖，不得因声明顺序、遍历顺序、最长词或评分删除 Edge；
- Enumerator 必须在 Compiler Version 固定资源边界内完整处理候选；稳定排序只服务可复现性，不是优先级；
- 每条 Token Stream 由同一 LALRPOP Grammar 解析；同一流的句法结构由 Grammar 规范化，跨流候选全部保留；不得建立快速 Parser、正则旁路或 LLM 旁路；
- 语义规则只能因显式类别、Owner、绑定、类型、单位或 Frame 角色约束淘汰候选；
- Convergence Gate 必须在看见全部有效候选后作结论，不能提前在第一个成功候选处停止。

## 4. LALRPOP 决定的适配性

`DEC-010` 的“零 Grammar 冲突”仍然有效，含义是：LALRPOP 生成阶段不得依赖 shift/reduce 或 reduce/reduce 的隐式选择。它不要求整个 CNL Source 只有一个 Tokenization 或一个中间 Parse Tree。

首版建议采用“候选 Token Stream 多次调用同一 LR(1) Parser”，原因是：

- 无需新增 Parser 依赖或改变已批准技术栈；
- 能明确审计每条候选路径，没有隐藏的最佳候选选择；
- 可以先证明正确性，再在不改变候选集合与最终结果的前提下优化为 DAG 共享；
- 现有旧 DSL Parser 只能作为实现结构参考，不能直接成为 CNL Parser。

若后续实测表明候选组合增长无法在固定资源模型内实现完整分析，应重新触发 ADR，而不是缩小语言接受集或加入概率选择。

## 5. 资源与失败分类

候选 Edge、完整 Token Stream、Parse Candidate、Bound Candidate 和 Canonical Frame Candidate 都必须纳入版本化资源计数。数值来自 `WP-APLS-CNL-C03-001` / `HDP-APLS-016`；实现无关计数对象与稳定顺序由 `DES-APLS-CNL-RESOURCE-001` 冻结，不得借用旧 DSL 或内部数据结构粒度改变边界。

```text
已完整分析且 Canonical Classes >= 2 -> Source Error / AMBIGUOUS / APLS-E1310
已完整分析且 Canonical Classes == 0 -> 具体 Source Error
无法完整枚举或验证候选              -> Tool Failure / APLS-T0007
```

资源耗尽只说明编译器没有完成证明，不能证明 Source 有多个含义。

## 6. 诊断与 Explain

- `APLS-E1102` 不用于 CNL 中间 Tokenization/Parse 多候选；
- `APLS-E1203` 不得仅因引用候选多于一个发出；
- `APLS-E1310` 只在最终存在两个或以上不等价 Canonical Frame 时发出；
- `APLS-E1310` 至少给出两个分歧见证、不同规范角色及最小 Source Span；
- Explain View 可在用户主动请求时展示候选如何收敛，但不得要求用户理解 AST 或 IR 才能修正 Source。

## 7. 最小 C03 迁移工作包候选

迁移应拆成以下顺序，任何一步失败都不向后发布部分 IR：

1. 建立 CNL Unicode/Sentence Validator、无类别 Declaration Bootstrap、Provisional Header 收集与 Declaration Binder；
2. 建立 Candidate Lexical Lattice、稳定枚举和新资源计数；
3. 新建唯一 CNL `.lalrpop` Grammar，并逐候选流解析；
4. 建立 Candidate/Bound/Typed/Canonical Frame 的互斥内部类型；
5. 实现规范等价、Provenance 并集和 `0/1/>1` Convergence Gate；
6. 接入 CNL 诊断，先做到 Source→Canonical Frame；
7. 只有 CNL-to-IR Schema Gap 关闭后，才接入 Canonical IR 和 Publisher；
8. 旧 DSL Frontend 保留为迁移证据但不再作为规范入口，不建立双入口。

## 8. 最小验证责任

- 同一词法切分、同一 Parse 的合法基线；
- 多 Tokenization/Parse 最终同一 Canonical Frame 的合法收敛；
- 两个不等价 Canonical Frame 的 `APLS-E1310`；
- 候选路径被类别、Owner、类型或单位规则确定淘汰；
- 候选遍历顺序变化不改变结果与诊断；
- 第一个成功、最长匹配、概率和 LLM 选择不存在；
- 候选资源耗尽归类为 `APLS-T0007`，且不产生 Frame/IR。

## 9. 未授权与待批准

技术适配路线已由 `HDP-APLS-014 Option A` / `DEC-018` 批准，CNL-to-IR Schema Gap 已由 `HDP-APLS-015 Option A` / `DEC-019` 关闭。本次 TASK-019 的细化差异尚待 `HDP-APLS-018` 采用；`TASK-018` 当前保持 `BLOCKED`。本文不授权代码、测试、依赖、Commit、Push、Baseline、C04、Release 或 Formal Seal。
