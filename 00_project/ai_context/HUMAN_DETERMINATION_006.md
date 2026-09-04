# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-006
TARGET: ADR-APLS-001
DECISION: APPROVED
SELECTED_OPTION: A
DECISION_DATE: 2026-09-02
DECISION_EVIDENCE: 项目负责人“HDP-APLS-006: APPROVED / OPTION: A”
```

## WHAT_MUST_BE_DECIDED

选择 APLS 0.1 Compiler MVP 的主实现语言与 Parser 生成技术：

- `OPTION A`：Rust + LALRPOP（默认 LR(1)）；
- `OPTION B`：Go + goyacc；
- `OPTION C`：Java + ANTLR4。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

该选择将建立长期代码、构建、依赖、部署和人才成本，并决定“Grammar 冲突必须失败”如何机械执行，属于重大架构决策。

## CONFIRMED_FACTS

- `DEC-004` 要求 APLS 拒绝歧义，不允许 Agent 或 Parser 运行时猜测；
- `DES-APLS-COMPILER-001` 已获批为未建立 Baseline 的设计输入；
- 三个候选都能实现 Compiler，但确定序列化与语义唯一性都必须由 APLS 自身设计保证；
- LALRPOP 官方说明默认使用 LR(1)；
- goyacc 官方说明生成可重入 Go Parser；
- ANTLR4 提供 `-Werror`，但其运行时预测和 Alternative 顺序行为需要附加零歧义控制；
- 本包没有安装或实测任何候选工具，不冒充 Prototype 证据。

## OPEN_QUESTIONS

- 本裁决包内无未决项；项目负责人已选择 `OPTION A`；
- 精确工具链与依赖版本在技术路线获批后另行提出；
- `OPTION B/C` 未被采用。

## OPTIONS_AND_DIFFERENCES

### OPTION A — Rust + LALRPOP（推荐）

- 最强项：LR(1) 静态 Parser 路线，加上 Rust Enum/Exhaustive Match，最容易把 Grammar 冲突和 Stage 非法状态变为机械失败；
- 代价：学习、生命周期建模与编译反馈成本较高，LALRPOP 依赖和文档成熟度需要持续控制。

### OPTION B — Go + goyacc

- 最强项：实现与原生 CLI 分发最简洁；
- 代价：零冲突门槛和互斥状态约束需要更多项目封装，goyacc 所在模块仍是 pre-v1。

### OPTION C — Java + ANTLR4

- 最强项：Grammar、Visitor 和 IDE 生态成熟；
- 代价：为满足 APLS 的拒绝歧义目标，需要额外限制 Alternative 顺序消歧、增加运行时歧义检测，并控制 Tool/Runtime/JVM 版本组合。

## RISKS_AND_TRADEOFFS

- Option A 的主要风险是开发成本，不是规范能力；
- Option B 的主要风险是关键不变量更多依赖编码纪律和额外 Gate；
- Option C 的主要风险是工具默认能力与“绝不隐式消歧”的 APLS 目标之间需要更复杂的封装；
- 未做 Prototype 意味着本次选择是架构方向裁定，精确版本与性能仍需后续最小验证。

## RECOMMENDED_OPTION_AND_REASON

推荐 `OPTION A`。它最直接地把首要目标“拒绝歧义”落实为 Parser 生成失败和类型穷尽检查，同时保留原生 CLI 分发能力。

## APPROVED_CONSEQUENCE

- 将 `ADR-APLS-001` 按所选 Option 标记为 `ACCEPTED`；
- 关闭 `Q-002`，记录实现语言与 Parser 技术；
- 允许 C00 准备下一工作包的范围与授权候选；
- 不自动开始编码或安装依赖。

## CHANGES_REQUESTED_CONSEQUENCE

`TASK-005` 返回 `IN_PROGRESS`，仅修改指定比较、约束或候选方案。

## DEFERRED_CONSEQUENCE

保持 ADR 候选和 `Q-002` Open，不进入实现工作包。

## REJECTED_CONSEQUENCE

关闭当前三个技术组合，重新建立候选范围；不得默认选用任一方案。

## EXPLICITLY_NOT_AUTHORIZED

本决定不授权安装 Rust/Go/JDK/Parser 工具或依赖，不授权创建 Compiler 代码、选择精确依赖版本、选择 JSON/Schema/CLI 库、冻结语义哈希或参考后端、Commit、Tag、Baseline Adoption、Formal C04 Dispatch、Push、Release、远程修改或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `DEC-004`、`DEC-009`；
- `04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md`；
- `03_architecture/architecture_decisions/ADR-APLS-001-compiler-implementation-stack.md`；
- ADR 第 12 节所列官方一手资料。

## COPYABLE_RESPONSE_FORMAT

推荐回复：

```text
HDP-APLS-006: APPROVED
OPTION: A
```

或：

```text
HDP-APLS-006: CHANGES_REQUESTED
修改：<具体比较、约束或新增候选>
```
