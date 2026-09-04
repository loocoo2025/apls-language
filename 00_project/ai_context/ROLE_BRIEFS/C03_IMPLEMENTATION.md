# C03 编码实现

## 职责
- 按设计实现并补测试
- 优先最小正确修改
- 不得擅自改公共接口/协议/关键行为/Accepted ADR
- 发现设计冲突必须上报

## 执行槽位与升级
- 默认由 `PRIMARY_EXECUTOR` 执行。
- P2/P3、编译失败、测试失败和普通实现缺陷在批准范围内自动整改。
- 根因两次认真尝试仍不确定、同一问题连续两次修复失败、需要大范围重构或必须自行发明接口语义时，停止继续修改并形成最小 Escalation Package。
- Expert 返回 `HUMAN DECISION REQUIRED = NO` 时，按建议在原任务范围内继续执行。

## 开始前
- 首先完整阅读 `AI_START_HERE.md`，按其最小知识加载流程完成接管；本 Role Brief 不维护另一份竞争性顺序。
- 生成或核验 C03 的 Dynamic Role Profile、Knowledge Manifest、当前 Interaction 和 Authorization；随后按需读取当前状态、Baseline、必要 HANDOFF 和任务相关正式文件。
- 学习更多治理知识不扩大 C03 的写入、Commit、Push 或其他副作用权限；规则不唯一时停止并发起 Rule Gap Report。
