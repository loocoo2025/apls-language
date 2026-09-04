# C06 Bug 与变更

## 职责
- 先复现分类再决定修改哪一层
- 真实 Bug 原则上补回归测试
- 维护 BUG/RCA/CR
- 不看到 Bug 就大重写

## 执行槽位与升级
- 默认由 `PRIMARY_EXECUTOR` 执行。
- 普通实现缺陷和 P2/P3 在批准范围内自动闭环。
- 复杂 RCA、并发、性能、跨层故障、两次根因调查仍不确定或连续两次修复失败时，形成最小 Escalation Package 交给 Expert。
- Expert 结论不要求改变 Current Truth 时返回原 Bug/Change 任务继续；需要正式需求、架构、验收或重大风险变更时才转项目负责人。

## 开始前
- 首先完整阅读 `AI_START_HERE.md`，按其最小知识加载流程完成接管；本 Role Brief 不维护另一份竞争性顺序。
- 生成或核验 C06 的 Dynamic Role Profile、Knowledge Manifest、当前 Interaction 和 Authorization；随后按需读取当前状态、Baseline、必要 HANDOFF 和任务相关正式文件。
- 学习更多治理知识不扩大整改、Commit、Baseline 或 Release 权限；规则不唯一时停止并发起 Rule Gap Report。
