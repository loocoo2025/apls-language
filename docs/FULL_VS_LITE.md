# Full Template 与 Lite

长程智构支持两种采用方式。它们遵循相同的治理原则，仅在初始采用的生命周期结构规模上有所不同。

## 对比

| 范围 | Full Template | Lite |
|---|---|---|
| 推荐项目类型 | 长周期、高风险、受监管、多学科或旧项目 | 小型、低风险、试点或评估项目 |
| C00～C06 角色 | 完整启用 | 保留完整角色边界，按需启用 |
| 产品/系统需求 | 完整文档集 | 可引用已有项目文档 |
| 架构与 ADR | 完整结构 | 当架构决策变得重要时再增加 |
| 追溯 | 包含且进行机械校验 | 在风险或验收要求需要前可选 |
| 测试治理 | 完整测试设计和证据结构 | 保留核心测试规则，按需增加目录 |
| 旧项目迁移 | 完整迁移工作流 | 大范围重构前升级到 Full |
| 发布/运维 | 包含 | 在受治理发布或现场运行前增加 |

## Lite 文件集

以下文件必须保留原始路径：

```text
AGENTS.md
AI_START_HERE.md
AI_ENGINEERING_RULES_V2.md
AI_CONVERSATION_ORCHESTRATION_RULES.md
PROJECT_START_PROMPT.md
INDEPENDENT_REVIEW_PROMPT.md
00_project/ai_context/CURRENT_STATE.md
00_project/ai_context/BASELINE_INDEX.md
00_project/ai_context/DECISION_INDEX.md
00_project/ai_context/ACTIVE_TASKS.md
00_project/ai_context/OPEN_QUESTIONS.md
00_project/ai_context/CONVERSATION_MAP.md
00_project/ai_context/ROLE_BRIEFS/*
00_project/governance/AI_CONTEXT_RESET_AND_BASELINE_RELEARN_RULES.md
00_project/governance/AI_TESTING_GOVERNANCE_RULES.md
00_project/governance/EXTERNAL_AI_TRANSFER_CONFIG.yaml
00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml
00_project/governance/PROJECT_ASSURANCE_CADENCE_POLICY.md
00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md
```

使用正式需求追溯时，增加 `09_quality/traceability/validate_traceability.py`。

复制后必须机械确认上述文件全部存在，且 `AI_START_HERE.md`、`CURRENT_STATE.md` 和 Role Brief 中不存在指向缺失治理 Authority / Configuration 的必需引用。Lite 不得通过省略文件关闭保障节奏策略中的任何不可关闭控制。

## Lite 中不可弱化的规则

Lite 可以减少产物，但不得弱化：

- Current Truth 和事实所有权；
- 已接受需求和决策的权威性；
- 独立评审上下文；
- 人类审批边界；
- 测试范围治理；
- 保障节奏中的不可关闭控制和强制 C04 触发；
- 外部 AI 的权限继承、默认关闭、单次授权与本地/外部 Session 放置边界；
- Dynamic Role Profile、最小知识加载、Rule Gap、标准 Interaction 与通用 Authorization；
- `Role != Model != Runtime != Harness != Session != Tool`；
- `PROCEDURAL_FALLBACK / TOOL_ENFORCED` 使用同一治理语义；
- 上下文重置、Knowledge Continuation 和 Baseline Relearn；
- 基于 Git 的 Review Target 和历史记录。

## 何时升级到 Full

- 项目变成长周期或多学科项目；
- 安全、数据完整性、合规或现场可靠性变得重要；
- 产品和系统需求需要正式追溯；
- 多个团队或供应商需要接口控制；
- 计划对旧项目进行结构重组；
- 必须治理发布、部署或运行证据。

Lite 不是治理规则的分叉版本。稳定规则仍保留在同一组根目录治理文档中，从而避免 Full 与 Lite 演变为互相竞争的 Current Truth 来源。
