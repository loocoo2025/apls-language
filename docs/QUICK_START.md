# 快速开始

## 前置条件

- Git；
- 能够读取和编辑仓库文件的编码 Agent 或 Harness；
- 仅在运行随附的追溯校验器时需要 Python 3。

长程智构不要求使用特定模型厂商。

## Full Template 设置

1. 将完整发布 Archive 复制或解压到目标项目根目录。
2. 如果目标是已有项目，不要覆盖冲突文件；先遵循 `AI_LEGACY_PROJECT_STANDARDIZATION_GUIDE.md`。
3. 替换 `{{PROJECT_NAME}}`、日期、Gate 以及与项目相关的其他占位符。
4. 在 `00_project/ai_context/CURRENT_STATE.md` 中配置当前阶段、授权、`AUTONOMY_MODE`、Model/Runtime/Harness 槽位、`AUTHORIZED_UNTIL`、`PREAUTHORIZED_GATES` 和 `ENFORCEMENT_MODE`。
5. 按 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 为当前 Session 建立 Dynamic Role Profile 与 Knowledge Manifest，并明确绑定当前或适用 Gate 与适用事实 Owner；需要机械执行时，让工具消费 `00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml`。
6. 如有需要，初始化 Git，并在开始实现前建立稳定锚点。
7. 将 `PROJECT_START_PROMPT.md` 交给当前 Agent。
8. 新项目从 C00/C01 开始，在进入架构或实现前先建立产品需求。

## Lite 设置

复制 `docs/FULL_VS_LITE.md` 中列出的 Lite 文件集。所有选中文件都应保留原始路径，以确保交叉引用和事实所有权仍然有效。

最低要求：

1. 配置 `CURRENT_STATE.md`；
2. 建立当前决策和 Baseline 引用；
3. 创建一个活动任务；
4. 选择当前 C00～C06 Role 并建立 Dynamic Role Profile / Knowledge Manifest；
5. 为受控岗位交互绑定 Interaction / Authorization；
6. 选择 `PROCEDURAL_FALLBACK` 或 `TOOL_ENFORCED`；
7. 正式 C04 评审时使用全新、独立的 Session 并记录独立性证据；
8. 切换 Model、Runtime、Harness 或物理上下文时执行 Knowledge Continuation Check；不满足条件时执行 Baseline Relearn。

## 五分钟核验

- Agent 能说明当前 Role、Dynamic Role Profile、当前或适用 Gate、适用事实 Owner、授权和下一项任务。
- 每个动态事实只有一个所有者文件。
- 当前 Model/Runtime/Harness 路由和 Enforcement Mode 只存在于 `CURRENT_STATE.md`。
- 受控副作用动作有精确 Interaction / Authorization，且 Action Class 不互相隐含。
- Review Target 是精确的 Git Target。
- 正式 C04 有独立 Session 证据，不能修改被评审对象，也不能关闭自己的 Finding。
- 人类审批边界明确。

如果任何答案不清楚，请停留在 C00，先解决治理状态问题，再进入实现。
