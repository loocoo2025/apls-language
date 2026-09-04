# IA-GOV-001：CR-GOV-001 影响分析

关联 Change：`CR-GOV-001`

当前版本：`v0.1.4-beta.1`

目标身份：`v0.1.5-candidate`

变更分类：`SUBSTANTIVE / GOVERNANCE_CHANGE`

## 1. 影响矩阵

| 范围 | 影响 | 说明 |
|---|---|---|
| 产品目标与产品行为 | NO | 框架模板变更，不修改使用该模板的产品事实 |
| C00～C06 基本职责 | NO | 固定岗位保持不变，增加动态执行 Profile |
| Current Truth / Owner | NO | 保持现有事实 Owner；新增文件拥有新的执行合同事实 |
| Model / Runtime / Harness / Session / Tool | YES | 增加维度定义、连续性检查和权限不扩张规则 |
| 授权与人工保留决策 | YES | 增加通用生命周期、部分预授权 Baseline Adoption 边界和 Formal Seal |
| Session / Interaction | YES | 增加标准岗位交互和四条审核/裁决运行线 |
| C04 Finding / Decision | NO | 现有 S0～S3、Readiness 和二值 Decision 保持不变 |
| C04 独立性证据 | YES | 增加可声明或机械证明的标准字段 |
| Task 状态 | YES | 把现有建议枚举收口为正式转换矩阵 |
| Testing / Traceability / ADR / V 模型 | NO | 不改变既有机制 |
| Baseline Relearn | YES | 增加 Knowledge Continuation 与完整 Relearn 的分界 |
| Lite | YES | 新权威政策和合同必须纳入 Lite |
| 工具实现 | NO | 只定义工具应消费的治理合同，不增加具体产品代码 |

## 2. 事实所有权

新增：

```text
ROLE_INTERACTION_EXECUTION_POLICY.md
→ Role/Profile/Knowledge/Interaction/Authorization/Review Line/Formal Seal/Enforcement Mode

GOVERNANCE_EXECUTION_CONTRACTS.yaml
→ 上述语义的机器字段、必填项和枚举
```

继续保留：

- 正式 C04 语义归 `AI_ENGINEERING_RULES_V2.md` 第 38.7 节；
- 保障触发归 `PROJECT_ASSURANCE_CADENCE_POLICY.md`；
- Session 编排归 `AI_CONVERSATION_ORCHESTRATION_RULES.md`；
- 当前运行值归 `CURRENT_STATE.md`；
- Task 实例状态归 `ACTIVE_TASKS.md`；
- Baseline 身份归 `BASELINE_INDEX.md`。

没有新增产品 Current Truth 来源。

## 3. 兼容性

### 向后兼容

- v0.1.4 项目可以继续运行，不强制原地修改历史；
- 现有 Role Brief、Review Record、Decision、Baseline 和任务 ID 保留；
- 已关闭 C04 Finding 不追溯重写；
- 已批准产品需求、架构、接口、代码和测试不因升级改变。

### 采用新版本时必须迁移

- 添加两个新增治理 Authority / Contract 文件；
- 为当前执行声明 Dynamic Role Profile 和 Knowledge Manifest；
- 选择 `PROCEDURAL_FALLBACK` 或 `TOOL_ENFORCED`；
- 映射当前 Task 状态；
- 对当前授权补齐 Action、Scope、Target、Side Effect、消费和终态；
- 完成 Candidate Baseline、正式 C04、显式 Adoption 和 Baseline Relearn。

### 不兼容或需人工裁决

- 把 C00～C06 改成可随意重定义角色的项目配置；
- 用模糊“批准”继续执行多个副作用动作；
- 把普通咨询、Self Review 或 Contextual Review 记录成正式 C04；
- 把程序性声明写成机械拦截证据；
- 把 Formal Seal 委托给 AI；
- 将已变化 Current Truth 下的旧预授权继续用于 Baseline Adoption。

## 4. 验证范围

直接验证：

- Markdown 结构、链接和代码围栏；
- YAML 解析和必填字段引用；
- Role / Review Line / Authorization / Task 状态枚举一致；
- One Fact One Owner；
- Lite 完整性；
- Template File Index 精确一致；
- 从 v0.1.4-beta.1 的迁移路径；
- 敏感信息和参考产品事实扫描。

不执行：

- 产品构建、单元测试、集成测试或真实模型调用；
- 外部产品仓库修改；
- Release 验证。

理由：本 Change 只修改治理模板和机器可读治理合同，不修改产品代码或可执行治理工具。

## 5. Gate 与 Baseline

```text
CURRENT_v0.1.4-beta.1_HISTORY_MUTATED: NO
CANDIDATE_COMMIT_REQUIRED_LATER: YES
FORMAL_C04_REQUIRED_LATER: YES
BASELINE_ADOPTION_REQUIRED_LATER: YES
BASELINE_RELEARN_REQUIRED_LATER: YES
CURRENT_AUTHORIZATION_END: V0.1.5_CANDIDATE_OUTPUT_READY
```

## 6. Remaining Risks

- YAML 当前是机器可读合同目录，不是具体 Runtime 的 JSON Schema 实现；
- `PROCEDURAL_FALLBACK` 仍存在人工遗漏风险；
- 引入通用授权生命周期后，已有项目需要对本地自定义授权字段做 Alias Mapping；
- Baseline 部分预授权必须严格验证，否则可能重新产生模糊授权问题；
- 工具实现若不读取正式合同，仍可能形成与治理规则不一致的第二套状态机。

## 7. 结论

```text
IMPACT_SCOPE: GOVERNANCE_EXECUTION_SEMANTICS
REAPPROVAL_SCOPE: CANDIDATE_POLICY_AND_CONTRACTS
REGRESSION_SCOPE: DOCUMENT_AND_CONTRACT_CONSISTENCY
UNAFFECTED_APPROVALS_PRESERVED: YES
REMAINING_UNKNOWN: NONE_WITHIN_APPROVED_CANDIDATE_SCOPE
NEXT_ACTION: IMPLEMENT_WP-GOV-015-02_TO_06
```
