# HDP-APLS-017 — TASK-018 实施前契约整改

```yaml
determination_id: HDP-APLS-017
status: APPROVED
selected_option: A
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-03
what_must_be_decided: 是否在编写 Compiler 代码前先执行 TASK-019，关闭 IIR-APLS-TASK018-PREIMPL-001 的剩余九项 Blocking Finding并完成全新独立再复审
why_human_authority_is_required: Finding 涉及公开 Source 接受集、类型和单位语义、Transition 行为、Unicode 版本与依赖、资源边界、IR Hash/Provenance 和公共诊断机器契约，C03 不得自行发明
```

## 独立复审事实

- Review：`IIR-APLS-TASK018-PREIMPL-001`；
- Review Line：`INDEPENDENT_REVIEW / INFORMAL_INDEPENDENT`；
- 结论：`CHANGES_RECOMMENDED`，不具正式 C04 权威；
- 起止 Target Set：45 个文件，SHA-256 均为 `343feebf2e27e9ef2e6feeaac7907a3c8098845b5b7e78d010927b7f6bdc0f61`；
- Finding：10 项 Blocking Finding、3 项 Non-blocking Observation；
- BF-01 是批准后 Current Truth 未及时同步，已由 C00 机械修正；
- BF-02～BF-10 会迫使 C03 在实现中自行决定公共语义，当前不能直接编码。

## Option A — 先完成统一契约整改与独立再复审（推荐）

批准 `TASK-019` 进入 C02 设计整改：

1. 将旧 Frame→IR 文件中的阻塞 Gate 明确为历史状态；
2. 把不受 Grammar 支持的状态比较示例修正为已批准状态谓词，不扩张首版语言；
3. 冻结完整 Property/Comparison 类型、单位、运算符和 Canonical Value 兼容矩阵；
4. 将 Transition 0.1 边界冻结为可机械检查的有限算法，不要求实现猜测一般条件可满足性；
5. 冻结实现无关的 Candidate Edge/Stream/AST/Frame 计数、稳定顺序和 `APLS-T0007` Payload；
6. 冻结 Unicode 数据版本与完整 NFC 实现方案，并单独列出任何依赖变更供下一次批准；
7. 冻结“合取去重后仅一项则降为 AtomicCondition”及 Provenance 合并；
8. 冻结内建 Unit 的 materialization closure、排序、Source Map 豁免和完整 Span 不变量；
9. 冻结零候选根因聚合、`APLS-E1310` Witness 与 `APLS-T0007` 的封闭机器结构；
10. 建立最小 Conformance Case，并对新 Target 发起全新非正式独立再复审。

本选项只授权形成一致的设计/Schema/工作包候选和再复审证据。若涉及新增依赖、改变公共 Schema 或批准语义，完成后仍需一个精确 Human Determination 才能恢复 `TASK-018`。

优点：在写代码前消除返工根因，并保护“最终语义唯一、无法证明就拒绝”的核心原则。

代价：增加一个 C02 整改与再复审工作包，Compiler 实施会顺延。

## Option B — 仅修复治理状态后直接实施

关闭 BF-01 后恢复 `TASK-018`，其余 Finding 由 C03 在实现中选择。

该方案会让实现自行决定公共语言、单位、Unicode、资源和诊断语义，违反 `DEC-004/017` 与工作包“不发明公共契约”的限制，不推荐。

## Option C — 暂停项目

保持 `TASK-018 BLOCKED`，不启动整改或编码，等待项目负责人以后重新决定。

## 批准后果

- `APPROVED / OPTION A`：`TASK-019` 进入 `IN_PROGRESS`，C02 形成一次性整改候选和全新独立再复审 Target；
- `APPROVED / OPTION B`：需要项目负责人明确接受九项未关闭 Finding 的风险并覆盖现有禁止条款；
- `APPROVED / OPTION C` 或 `DEFERRED`：项目保持暂停；
- `CHANGES_REQUESTED`：按负责人指定范围调整整改包。

## 裁决结果

项目负责人于 `2026-09-03` 明确回复：

```text
HDP-APLS-017: APPROVED
OPTION: A
```

本批准形成 `DEC-021`，授权 `TASK-019` 在不修改 Compiler Source、实现测试、依赖或 Cargo Lock 的前提下，形成 BF-02～BF-10 的统一设计整改候选和全新独立再复审证据。

## 明确未授权

- 不授权 Compiler Source、实现测试、依赖或 Cargo Lock 修改；
- 不授权直接改变当前公开 Grammar、Frame、IR 或诊断契约的批准状态；
- 不授权 Commit、Push、Baseline Adoption、Formal C04、Release 或 Formal Seal；
- 不把本次 Advisory 复审冒充正式 C04。

## 可复制回复格式

```text
HDP-APLS-017: APPROVED
OPTION: A
```
