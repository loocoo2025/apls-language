# HDP-APLS-018 — TASK-019 契约采用、Unicode 依赖与 TASK-018 恢复

```yaml
determination_id: HDP-APLS-018
status: APPROVED
selected_option: A
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-04
what_must_be_decided: 是否采用经第三轮增量独立复审通过的 TASK-019 公共契约与 Schema 候选，批准精确 Unicode/NFC 依赖边界，并恢复 TASK-018 实施
why_human_authority_is_required: 该决定会改变当前公共语言和机器契约输入、依赖闭包及 C03 实施授权；非正式独立复审只有建议权
```

## 已确认事实

- `IIR-APLS-TASK019-REREVIEW-003` 的范围为 `DELTA_ONLY`，结论为 `READY_FOR_HUMAN_DETERMINATION`；
- BF-02～BF-10 与 NF-01～NF-05 均为 `CLOSED`；未变化项按 `INHERITED_CLOSED` 继承，没有重复全量复审；
- 评审时 40 文件 Target 在开始、报告写入前和结束均匹配 SHA-256 `05a9e0c556ef87f72ae5f2f21f9cbae9a75b3e59fa3c474da838a71e975fc801`；
- 第三轮只修正资源证据分层，没有改变 `DEC-020` 已批准的任何资源数值、生产 `APLS-T0007` 语义或 Source 接受集；
- 当前没有修改 Compiler Source、实现测试、Cargo Manifest、Cargo Lock 或依赖；现有 `07_src` 仍是旧 DSL 原型；
- 本批准包不建立产品 Baseline，也不等于正式 C04、Release 或 Formal Seal。

## 待采用的 TASK-019 契约

Option A 将以下经复审候选作为 `TASK-018` 的当前受控实现输入：

1. 六类 Property、Literal、比较运算符与单位兼容矩阵，以及精确 Decimal/时间规范化；
2. Transition 的隐式 Source State、有限可达性和同 Trigger 冲突检查边界；
3. Declaration Bootstrap：无类别 `DECLARED_TERM` 预索引、全部 Provisional Header 后绑定、仍使用同一正式 Grammar，不建立第二 Parser；
4. 合取排序去重、单项降为 Atomic、Provenance 稳定并集；
5. 内建 Unit materialization、Source Map 豁免、Role Enum 与 UTF-8 Byte Span 交叉验证；
6. Candidate 资源的计数对象、登记顺序、Span 和 T0007 Payload；前三类 `PUBLIC_SOURCE_REACHABLE` 使用 CLI 证据，后四类 `DEFENSIVE_DOMINATED` 使用私有 `#[cfg(test)]` Ledger 验证同一生产登记函数；
7. Unicode 17.0.0、完整 NFC、IR Header 与 Compiler Binary 绑定断言；不新增独立 Compiler Manifest，也不改变 `apls --version`；
8. `apls-cnl-ir-0.1.schema.json` 和 `apls-cnl-diagnostic-0.1.schema.json` 候选，以及 E1310、T0007、零候选诊断和唯一 Byte-based 总排序；
9. `TESTDES-APLS-CNL-PREIMPL-001` 的 16 个最小高价值 Conformance Case，包括 CNL-C012A/B/C。

精确设计对象以 `IIR-TARGET-APLS-TASK019-003` 中的 TASK-019 语言、Frame、IR、Schema、Compiler、Diagnostics 和 Test Design 文件为准；动态治理文件在评审结束后的状态同步不改变上述候选内容。

## 待批准的依赖边界

允许在 `07_src/crates/apls-compiler/Cargo.toml` 增加：

```toml
unicode-normalization = { version = "=0.1.25", default-features = false }
```

当前候选供应链证据为：

```text
unicode-normalization 0.1.25
crate SHA-256: 5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8

expected transitive: tinyvec 1.6.0 with alloc
crate SHA-256: 87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50

expected transitive: tinyvec_macros 0.1.1
crate SHA-256: 1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20
```

批准的是上述直接依赖边界，不是预先伪造的 Lock 结果。C03 必须由 Cargo 生成实际 `Cargo.lock` 变化，核对精确版本、Checksum、Feature、License 和 MSRV 1.86 构建结果。实际闭包若与候选不一致或任一验证失败，`TASK-018` 立即重新阻塞并返回项目负责人，不得改用缩减 NFC 检查或擅自升级版本。

## Option A — 采用契约、批准依赖并恢复完整实现（推荐）

批准：

1. 采用上一节 TASK-019 契约作为当前 CNL Compiler 实现输入，但不建立正式 Baseline；
2. 在精确依赖边界内修改 Compiler Cargo Manifest 和由 Cargo 生成的 Lock；
3. 将 `TASK-019` 标记为 `DONE`，关闭 `Q-019`；
4. 将 `TASK-018` 从 `BLOCKED` 恢复为 `IN_PROGRESS`；
5. 按 `WP-APLS-CNL-C03-001` 修改 Compiler/CLI Source、最小定向测试和 Lite 实施证据；
6. 实施阶段普通复审继续按 `DEC-022` 默认只审本轮代码、测试、依赖变化及其直接影响闭包。

优点：实施不再需要自行发明语义，项目可以开始构建真正的简体中文“可编译自然语言”端到端路径。

代价：需要增加一个精确直接依赖并生成新的 Lock；完整 CNL 垂直切片仍是较大的实现工作包，必须按阶段运行最小定向验证。

## Option B — 只采用设计契约，继续阻塞实现

采用 TASK-019 公共契约和两个 Schema 作为当前设计输入，但暂不批准依赖、Cargo/Lock、Compiler Source 或测试修改；`TASK-019` 可完成，`TASK-018` 保持 `BLOCKED`。

适用于希望先冻结设计、以后再决定供应链和实施时机的情况。

## Option C — 要求继续修改

不采用当前候选。由项目负责人指出需修改的具体语义、Schema、资源、Unicode、诊断或实施边界；`TASK-019` 返回 `IN_PROGRESS`，`TASK-018` 保持 `BLOCKED`。后续复审默认只覆盖新增改动及其直接影响闭包。

## 推荐理由

推荐 `OPTION A`。全部实施前 Finding 已关闭，第三轮新增问题已经通过增量独立复审确认；继续只扩写设计的边际价值低于按严格契约实现并获得真实 Compiler 证据。Option A 同时保留失败关闭原则：依赖闭包或 MSRV/许可验证不一致时立即停止，不让实现自行选择替代语义。

## 裁决结果

项目负责人于 `2026-09-04` 明确回复：

```text
HDP-APLS-018: APPROVED
OPTION: A
```

本批准形成 `DEC-023`：TASK-019 契约和两个 Schema 成为 TASK-018 的当前受控实现输入；批准精确 Unicode 直接依赖边界；TASK-019 完成，Q-019 关闭，TASK-018 按 `BLOCKED → READY → IN_PROGRESS` 恢复。未列出的副作用仍不授权。

## 批准后果

- `APPROVED / OPTION A`：形成下一项当前决定，采用 TASK-019 契约、批准精确依赖边界、完成 TASK-019，并恢复 TASK-018；
- `APPROVED / OPTION B`：采用设计契约但不批准依赖和代码变化，TASK-018 继续阻塞；
- `CHANGES_REQUESTED / OPTION C`：只整改负责人指出的增量，并按 `DEC-022` 做增量复审；
- `DEFERRED`：所有候选保持未采用，TASK-018 继续阻塞；
- `REJECTED`：废弃当前采用方案，重新确定路线。

## 明确未授权

- 不授权改变 `DEC-004/017` 的“最终语义必须唯一、无法证明就拒绝”；
- 不授权临时公开 CNL 子集、第二 Parser、旧 DSL/CNL 双入口、LLM/概率/候选排名消歧；
- 不授权改变已批准资源数值，或把资源耗尽误报为 `AMBIGUOUS`；
- 不授权 Import、多语言、后端代码生成、LSP、IDE 或当前工作包外的依赖变化；
- 不授权 Commit、Push、Baseline Adoption、正式 C04、Release 或 Formal Seal。

## 可复制回复格式

```text
HDP-APLS-018: APPROVED
OPTION: A
```
