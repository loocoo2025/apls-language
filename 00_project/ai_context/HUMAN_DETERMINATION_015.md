# HDP-APLS-015 — CNL Canonical IR、Semantic ID 与单位语义

```yaml
determination_id: HDP-APLS-015
status: APPROVED
selected_option: A
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-03
what_must_be_decided: 是否批准独立的 apls-cnl-ir-0.1 Schema、机器自动生成的 Semantic ID、SHA-256 匿名节点身份、名义自定义单位和等价重复节点合并规则
why_human_authority_is_required: 这些规则共同决定公共机器契约、跨版本身份、语义哈希、单位兼容和同义 Source 的 Canonical IR 等价性
```

## 已确认事实

- 用户默认只写和查看受控自然语言，不应被要求填写 FQN、UUID、Hash 或 JSON IR；
- `DEC-017` 要求所有有效候选最终收敛到唯一 Canonical Frame，禁止实现猜测；
- `DEC-018` 要求在代码迁移前先闭合 CNL Canonical IR 和无损映射；
- 旧 `apls-ir-0.1` 无法无损承载 CNL Frame，不能直接复用为 CNL Verified IR；
- CNL 当前允许未声明换算关系的自定义单位；Compiler 不能用常识猜测换算；
- Rule、Transition、Invariant、Acceptance 和 InformativeItem 没有用户显式名称，需要确定机器身份。
- 当前 CNL Grammar 没有 Import 句式，因此 0.1 CNL IR 只能有一个 Entry Source；旧 DSL Source Graph 不自动进入新语言。

## Option A — 采用独立 CNL IR 契约（推荐）

批准：

1. `DES-APLS-CNL-IR-001`；
2. `apls-cnl-ir-0.1.schema.json`；
3. 声明节点 ID 由 Kind、Owner 和 NFC 精确显示名机械生成；用户不填写 ID；
4. 匿名节点 ID 为 Kind-separated Canonical Semantic Payload 的 SHA-256；
5. 文档领域语义摘要使用 SHA-256，并关闭 `Q-007`；
6. 语义相同的重复匿名节点合并为一个节点，Provenance 取稳定并集；
7. 自定义单位在 0.1 中成为名义维度基准单位，只能与自身兼容，不猜测换算；
8. 旧 `apls-ir-0.1` 保留为 Legacy DSL 参考，不原地改写。
9. `apls-zh-CN-0.1` 首版采用单 Entry Source；多文件/Import 必须以后通过版本化语言变更加入。

优点：用户表面保持自然；机器身份确定；Frame 映射无损；同义句和中间多候选可生成一致 IR；无需新增 Parser 或运行时依赖。

代价：重命名声明会改变 Semantic ID；新 CNL IR 不与旧 DSL IR 字段级兼容；内容相同的重复规则只保留一份规范节点，但保留全部来源。

## Option B — 扩展旧 IR，并要求 Source 提供显式机器身份与单位换算

保留旧 ASCII FQN、Dimension/Unit 和匿名节点禁止规则，新增自然语言句式让用户显式声明机器 ID、维度、Scale/Offset 及每条规则名称。

优点：更接近旧 Schema，跨重命名身份可由用户维持。

代价：用户必须接触机器概念，显著偏离“自然语言是唯一公开表面”的定位；还需要修改已批准 Profile/Grammar。

## Option C — 暂缓 IR，先只实现到 Canonical Frame

不批准任何 CNL IR Schema，未来另行设计；Compiler 只能验证 Source 并输出内部 Canonical Frame，不能向 Agent 发布 Verified IR。

优点：暂时回避公共格式决定。

代价：无法完成 APLS 的 Agent 可消费机器契约主目标，后续代码会停在中间阶段。

## 推荐理由

推荐 `OPTION A`。它把机器稳定性放在编译器内部解决，同时不把 ID、Hash、单位换算细节转嫁给用户；遇到未知换算时采用名义类型而不是常识猜测，符合“人自然表达、机器证明、无法证明就拒绝”的原则。

## 裁决结果

项目负责人于 `2026-09-03` 明确回复：

```text
HDP-APLS-015: APPROVED
OPTION: A
```

本批准已形成 `DEC-019`。批准范围仅限本文件 Option A 所列 CNL IR 契约及其作为后续 C03 实现输入的用途，不自动授权 Compiler Source、测试、依赖、Baseline、Commit、Push、C04、Release 或 Formal Seal。

## 批准后果

- `APPROVED / OPTION A`：将新 CNL IR 设计与 Schema 批准为 C03 后续设计输入，关闭 `Q-007/Q-016`，同步旧 IR 为 Legacy DSL Only，并建立最小 C03 Frontend/Frame/IR 实现工作包；
- `APPROVED / OPTION B`：重新打开中文 Profile、Grammar 与旧 IR Schema，先设计显式 ID/Unit 句式；
- `APPROVED / OPTION C`：IR Publisher 保持阻塞，只允许后续工作到内部 Canonical Frame；
- `CHANGES_REQUESTED`：按项目负责人指出的具体 ID、Hash、Unit 或重复规则修订；
- `DEFERRED`：`TASK-017` 保持 Output Ready，C03 不开始；
- `REJECTED`：废弃本候选，重新提出不违反 `DEC-014/017/018` 的 IR 路线。

## 明确未授权

- 不授权 Compiler Source、测试、依赖、旧 IR Schema 或 Publisher 修改；
- 不授权 Commit、Push、Baseline Adoption、Formal C04、Release 或 Formal Seal；
- 不授权向用户暴露必须填写的内部 ID 或 Hash；
- 不授权使用 LLM、拼音、翻译、随机 UUID、Source 顺序或 Byte Offset 生成语义身份。

## 权威来源

- `DEC-004`、`DEC-014`、`DEC-017`、`DEC-018`；
- `DES-APLS-CNL-FRAME-001`；
- `DES-APLS-CNL-FRONTEND-001`；
- `DES-APLS-CNL-IR-MAP-001`；
- `DES-APLS-CNL-IR-001`；
- `apls-cnl-ir-0.1.schema.json`。

## 可复制回复格式

```text
HDP-APLS-015: APPROVED
OPTION: A
```
