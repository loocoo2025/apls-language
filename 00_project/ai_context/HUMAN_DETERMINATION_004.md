# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-004
TARGET: APLS_0.1_DETAILED_LANGUAGE_DESIGN_CANDIDATE
DECISION: APPROVED
DECISION_DATE: 2026-09-02
DECISION_EVIDENCE: 项目负责人“HDP-APLS-004: APPROVED”
```

## WHAT_MUST_BE_DECIDED

是否批准以下 APLS 0.1 详细设计候选，作为 Compiler MVP 详细设计的受控输入：

- `04_design/language/APLS_0.1_LANGUAGE_DESIGN.md`；
- `04_design/language/APLS_0.1_GRAMMAR.ebnf`；
- `04_design/ir/APLS_0.1_CANONICAL_IR.md`；
- `04_design/ir/apls-ir-0.1.schema.json`；
- `04_design/diagnostics/APLS_0.1_DIAGNOSTICS.md`。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

候选已定义 `.apls` 的关键字、结构、表达式、动作、名称边界、Canonical IR 公共 Schema 和稳定诊断编号。这些属于语言和工具链的长期公共接口，不能由实现者自行冻结。

## CONFIRMED_FACTS

- `HDP-APLS-003 Option A` 已确定 `.apls` 为唯一规范性 Source，Canonical JSON IR 为机器交换格式；
- Grammar 候选定义 74 条非终结规则，未发现重复定义或未定义引用；
- 数字与工程量、名称与调用、一元表达式、动作块均已改为单一结构路径；
- IR 的 Type Reference、Constraint/Safety 和四类 Action 使用互斥结构，未知字段默认失败；
- 诊断目录覆盖词法/语法、名称、类型/单位、语义冲突和 IR/版本；
- 候选只是详细设计，尚未成为产品 Baseline。

## OPEN_QUESTIONS

- `Q-002`：Compiler MVP 实现语言；
- `Q-003`：首个参考后端；
- `Q-007`：Canonical IR 语义哈希算法及版本标识；
- Parser 生成器/技术和标准单位库清单仍未冻结。

以上问题不影响候选作为 Compiler MVP 设计输入，但在对应实现或兼容接口被冻结前必须裁定。

## OPTIONS_AND_DIFFERENCES

- `APPROVED`：保持候选内容，允许进入 Compiler MVP 详细设计；
- `CHANGES_REQUESTED`：指定需修订的语法、IR 或诊断条目，`TASK-003` 返回 `IN_PROGRESS`；
- `DEFERRED`：保留候选，不开始下一工作包；
- `REJECTED`：否定当前详细设计路线，重新建立设计任务。

## RISKS_AND_TRADEOFFS

- 自定义 DSL 使 Parser、格式化和编辑器支持的初期工作量高于复用通用配置格式；
- 严格 Schema 与显式 `null/空集合` 增大 IR 体积，但可防止下游工具自行推断默认语义；
- 当前只完成 Schema 的 JSON 语法、本地 `$ref`、Required/Properties 和 Grammar 引用一致性检查；独立 Parser 冲突检查和正反向 Conformance Cases 属于后续验证工作。

## RECOMMENDED_OPTION_AND_REASON

推荐 `APPROVED`。候选已把“拒绝歧义”从原则落到单一语法路径、结构化 Type Reference、互斥 IR 节点、未知字段失败和稳定诊断上，且没有擅自选择实现语言或后端。

## CONSEQUENCES

- `APPROVED`：详细设计标记为 `APPROVED_DESIGN_INPUT`，建立 Compiler MVP 设计任务；
- 本次批准不等于 Language/IR Baseline Adoption，不声明已通过独立评审或 Conformance 验证。

## EXPLICITLY_NOT_AUTHORIZED

本决定不授权选择实现语言、安装依赖、编写编译器代码、冻结参考后端或哈希算法、Commit、Tag、Baseline Adoption、Formal C04 Dispatch、Push、Release、远程修改或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `01_product_requirements/PRD.md`；
- `03_architecture/system_architecture.md`；
- `00_project/ai_context/DECISION_INDEX.md`；
- 本包 `WHAT_MUST_BE_DECIDED` 列出的 5 个详细设计候选。

## COPYABLE_RESPONSE_FORMAT

```text
HDP-APLS-004: APPROVED
```

或：

```text
HDP-APLS-004: CHANGES_REQUESTED
修改：<具体语法、IR 或诊断条目>
```
