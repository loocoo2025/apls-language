# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-003
TARGET: APLS_0.1_SURFACE_SYNTAX_STRATEGY
DECISION: APPROVED
SELECTED_OPTION: A
DECISION_DATE: 2026-09-02
DECISION_EVIDENCE: 项目负责人“HDP-APLS-003: APPROVED / OPTION: A”
```

## WHAT_MUST_BE_DECIDED

选择 APLS 0.1 唯一受支持的人类编写表面语法策略。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

表面语法是用户长期书写和评审 APLS 的公共语言接口。该选择会影响兼容性、学习成本、Parser 复杂度和“拒绝歧义”目标，不能由实现者自行冻结。

## CONFIRMED_FACTS

- 历史候选曾要求唯一 AST；该中间阶段要求已被 `HDP-APLS-013 Option B` / `DEC-017` 取代。当前要求是最终 Canonical Frame 与 Canonical IR 语义唯一；
- 无法唯一解释的输入必须失败关闭；
- LLM 不进入规范编译路径；
- Canonical IR 面向机器消费，表面语言面向人类书写；
- APLS 0.1 只需要一个规范性表面语法入口。

## OPEN_QUESTIONS

- 本包只裁定表面语法策略；
- 编译器实现语言 `Q-002` 和参考后端 `Q-003` 不在本包范围内；
- 具体关键字、标点和完整 Grammar 将在批准后设计。

## OPTIONS_AND_DIFFERENCES

### Option A — 单一自定义 APLS DSL + Canonical JSON IR（推荐）

- 人类只编写 `.apls` DSL；
- 语言拥有独立、严格、无歧义的正式 Grammar；
- 工具和 Agent 主要消费 Canonical JSON IR；
- 优点：可针对系统规格优化可读性、诊断和语义边界；
- 代价：需要自行实现和维护 Parser、格式化器及编辑器支持。

### Option B — 受限 YAML 子集 + Canonical JSON IR

- 人类编写经过裁剪的 YAML 风格文件；
- 必须禁用隐式类型、别名、锚点、合并键及实现相关解析行为；
- 优点：原型较快，初期结构直观；
- 代价：最终仍需维护一套“不是标准 YAML”的自定义规则，错误定位和行为语义表达较弱。

### Option C — DSL 与 YAML 双入口

- 两种人类表面语法映射到同一 Canonical IR；
- 优点：用户选择更多；
- 代价：必须证明两个 Frontend 永久语义等价，显著扩大歧义面、测试矩阵和兼容成本。

## RISKS_AND_TRADEOFFS

- Option A 初期工具工作量较大，但最符合“拒绝歧义”与长期语言演进目标；
- Option B 看似简单，实际会把 YAML 的解析差异转化为 APLS 必须重新定义的边界；
- Option C 在 0.1 阶段引入两套 Source Truth 表示，直接放大最重要的风险。

## RECOMMENDED_OPTION_AND_REASON

推荐 `Option A`：使用单一自定义 APLS DSL 作为唯一规范性源语言，使用 Canonical JSON IR 作为机器交换格式。

理由：它把人类可读性和机器确定性分离，同时只维护一条 Source → AST → IR 规范路径，最有利于拒绝歧义、精确诊断和后续兼容治理。

## CONSEQUENCES

- `APPROVED + OPTION A`：开始 `.apls` 词法、EBNF/PEG Grammar 和 Canonical JSON IR Schema 设计；
- `APPROVED + OPTION B`：先定义允许的 YAML 子集和全部禁用特性，再设计 Schema；
- `APPROVED + OPTION C`：必须先增加双 Frontend 等价性和兼容性要求，不直接开始 Grammar；
- `CHANGES_REQUESTED`：根据指定范围修改选项；
- `DEFERRED`：`TASK-003` 保持 `BLOCKED`；
- `REJECTED`：重新提出表面语法策略。

## EXPLICITLY_NOT_AUTHORIZED

本决定不授权选择实现语言、安装 Parser 依赖、编写编译器代码、Commit、Tag、Baseline Adoption、Formal C04 Dispatch、Push、Release、远程修改或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `01_product_requirements/PRD.md`
- `03_architecture/system_architecture.md`
- `00_project/ai_context/DECISION_INDEX.md`
- `00_project/ai_context/OPEN_QUESTIONS.md`

## COPYABLE_RESPONSE_FORMAT

推荐方案：

```text
HDP-APLS-003: APPROVED
OPTION: A
```

或：

```text
HDP-APLS-003: CHANGES_REQUESTED
修改：<具体要求>
```
