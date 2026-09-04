# 正式变更决定模板

> 适用于用户或项目负责人已经明确决定改变当前正式事项。

## 核心原则

> 新决定必须正式落盘；旧决定必须明确 `SUPERSEDED`；必须从正确层级向下做影响分析，不能只改代码。

## 1. 基本信息
- Change ID：`CR-XXX`
- Decision ID：`DEC-XXX`
- 日期：
- 当前 Baseline ID：
- 当前 Git Commit：

## 1.1 Human Determination Package（负责人决策包）

- Determination ID：
- 本次具体决定：
- 为什么需要 Human Project Owner 裁定：
- 已确认事实：
- 未决问题：
- 选项及差异：
- 主要收益、风险和取舍：
- 推荐选项及理由：
- `APPROVED` 的精确后果、Action、Target、Scope 与允许继续的阶段/Gate：
- `CHANGES_REQUESTED` 的后果：
- `DEFERRED` 的后果：
- `REJECTED` 的后果：
- 明确未授权事项：
- 权威正文/证据：
- 可复制回复格式：`DECISION: APPROVED / CHANGES_REQUESTED / DEFERRED / REJECTED; DETERMINATION_ID: {{ID}}; AUTHORIZED_SCOPE: {{SCOPE_OR_NONE}}`

> “批准”只有在唯一待决 Package、Target、Action、Scope 和后果均明确时才有效；模型不得把分析建议提升为人类裁定。

## 2. 变更分类
`REQUIREMENT_CHANGE / ARCHITECTURE_CHANGE / DESIGN_CHANGE / INTERFACE_CHANGE / TEST_CHANGE / OTHER`

正式状态：`APPROVED / CONFIRMED / ACCEPTED`

## 3. 原决定
- 原 Decision ID：
- 原正式内容：
- 原正式文件：
- 原状态：

## 4. 新决定
- 新决定：
- 新状态：

## 5. 替代关系
```text
OLD_DECISION:
OLD_STATUS: SUPERSEDED
SUPERSEDED_BY:

NEW_DECISION:
NEW_STATUS: APPROVED
SUPERSEDES:
```

## 6. 变更原因
-

## 7. 真正起始层级
`PRODUCT_REQUIREMENT / SYSTEM_REQUIREMENT / ARCHITECTURE / INTERFACE / DETAILED_DESIGN / TEST_REQUIREMENT / OTHER`

规则：
- 产品行为变化 → 从 PRD / SRS 开始；
- 技术路线变化但需求不变 → 从 ADR / Architecture 开始；
- 架构不变、模块内部结构变化 → 从 Detailed Design 开始；
- 纯实现错误 → 使用 `IMPLEMENTATION_BUG_FIX_TEMPLATE.md`。

## 8. 影响分析

| 层级 | 是否影响 | 文件/ID | 动作 |
|---|---|---|---|
| PRD | YES / NO | | |
| SRS | YES / NO | | |
| 接口要求 | YES / NO | | |
| ADR | YES / NO | | |
| 架构 | YES / NO | | |
| 详细设计 | YES / NO | | |
| 需求追溯 | YES / NO | | |
| 代码 | YES / NO | | |
| 单元测试 | YES / NO | | |
| 集成/系统测试 | YES / NO | | |
| CI / 质量 | YES / NO | | |
| 部署 | YES / NO | | |
| 运维 | YES / NO | | |

## 9. 明确不受影响
-

原批准继续有效的证据：
-

## 9.1 重新批准与回归范围

- 必须重新批准：
- 无需重新批准：
- 必须重新验证：
- 必要回归：
- 未扩大到全链路的理由：

> 只修改和重新批准真实受影响的内容；重大语义、安全、合规、数据完整性变化或影响不明时扩大范围。

## 10. 修改顺序
```text
正式决定
↓
需求（如受影响）
↓
ADR（如受影响）
↓
架构
↓
详细设计
↓
接口/协议
↓
测试设计
↓
代码
↓
自动化测试
↓
验证
↓
追溯更新
↓
CURRENT_STATE
↓
BASELINE_INDEX
```

## 11. 完成检查
- [ ] 新决定已落盘
- [ ] 旧决定已 `SUPERSEDED`
- [ ] `DECISION_INDEX.md` 已更新
- [ ] 需求/ADR/架构/设计与新决定一致
- [ ] 代码不再依赖旧决定
- [ ] 测试不再验证旧产品行为
- [ ] 必要回归测试通过
- [ ] 需求追溯已更新
- [ ] `CURRENT_STATE.md` 已更新
- [ ] `BASELINE_INDEX.md` 已更新
- [ ] 同一主题不存在两个当前有效决定

## 12. 是否建立新 Baseline
`YES / NO`

新 Baseline ID：
Git Commit：

Baseline Package / Gate 决策边界：
-

采用执行者：Human Project Owner / C00 / {{EXISTING_BASELINE_OWNER}}
Authorization Contract ID：

> C00 仅可依据 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 第 5.1 节所定义的精确预授权执行 `BASELINE_ADOPTION`。

## 12.1 Formal Seal（如适用）

- 是否要求：YES / NO
- Seal ID：
- 精确 Target / Purpose / Scope：
- Human Project Owner 明确回复与证据：

Formal Seal 与本变更决定、Commit、C04 PASS、Baseline Adoption 和 Release 均为独立 Action；不得互相推导授权。

## 13. 给 AI 的直接指令
```text
这是一个正式变更决定，不是方案讨论。

我已经明确确认新决定。

请先识别本变更真正起始层级，然后：
1. 将新决定正式落盘；
2. 将旧决定标记为 SUPERSEDED；
3. 更新 DECISION_INDEX.md；
4. 从正确层级向下做完整影响分析；
5. 只修改真正受影响的内容；
6. 更新需求/ADR/架构/设计/代码/测试中受影响的部分；
7. 更新需求追溯；
8. 运行对应验证；
9. 更新 CURRENT_STATE.md；
10. 更新 BASELINE_INDEX.md；
11. 告诉我当前新的唯一有效决定和 Baseline。

不得因为修改方便而跳过上层正式文件。
```
