# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-002
TARGET: ARCH-APLS-001
DECISION: APPROVED
DECISION_DATE: 2026-09-02
DECISION_EVIDENCE: 项目负责人“HDP-APLS-002: APPROVED”
```

## WHAT_MUST_BE_DECIDED

是否批准 [APLS 0.1 总体架构候选](../../03_architecture/system_architecture.md)，作为正式语法、Canonical IR Schema、诊断目录和编译器详细设计的受控输入。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

该候选定义确定性编译器核心、Canonical IR、Agent 消费边界和 LLM 辅助边界，涉及新的系统边界和未来公共契约，属于项目负责人保留决定。

## CONFIRMED_FACTS

- PRD 已通过 `HDP-APLS-001`，可作为架构输入；
- “拒绝歧义”是首要质量目标；
- 架构候选完整覆盖 PRD-001～PRD-008；
- 确定性核心不调用 LLM；
- 任一 Error 存在时不发布 Verified IR；
- Canonical IR 绑定 Source、语言版本和编译器版本。

## OPEN_QUESTIONS

- `Q-001` 表面语法形式；
- `Q-002` 编译器实现语言；
- `Q-003` 首个参考后端；
- 本次批准不同时裁定以上三个问题。

## OPTIONS_AND_DIFFERENCES

- `APPROVED`：确认总体边界，允许进入语法与 IR 详细设计；
- `CHANGES_REQUESTED`：指出需修改的架构条目，任务返回 C02；
- `DEFERRED`：保留候选，不进入详细设计；
- `REJECTED`：否定当前边界，重新建立架构候选。

## RISKS_AND_TRADEOFFS

- 把 LLM 放入规范编译路径会破坏确定性和可复现性；
- Canonical IR 过早绑定具体序列化格式会限制演进，因此本候选只冻结语义边界，不冻结 JSON/YAML/二进制格式；
- 表面语法、实现语言和参考后端延后决定，可避免把尚未验证的实现偏好写成架构事实。

## RECOMMENDED_OPTION_AND_REASON

推荐 `APPROVED`。该架构把“拒绝歧义”落实为逐阶段失败关闭，并隔离 AI 建议与规范编译事务，同时保留三个尚需研究的实现选择。

## CONSEQUENCES

- `APPROVED`：架构标记为 `APPROVED_DESIGN_INPUT`，建立详细设计工作包；
- `CHANGES_REQUESTED`：仅修订指定范围，`TASK-002` 回到 `IN_PROGRESS`；
- `DEFERRED`：保持当前候选，不开始详细设计；
- `REJECTED`：关闭当前候选并建立新的架构任务。

## EXPLICITLY_NOT_AUTHORIZED

本决定不授权选择实现语言、安装依赖、编写编译器代码、Commit、Tag、Baseline Adoption、Formal C04 Dispatch、Push、Release、远程修改或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `01_product_requirements/PRD.md`
- `03_architecture/system_architecture.md`
- `00_project/ai_context/DECISION_INDEX.md`
- `00_project/ai_context/CURRENT_STATE.md`

## COPYABLE_RESPONSE_FORMAT

```text
HDP-APLS-002: APPROVED
```

或：

```text
HDP-APLS-002: CHANGES_REQUESTED
修改：<具体架构条目和要求>
```
