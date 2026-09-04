# HDP-APLS-016 — 完整 CNL Compiler 垂直切片与候选资源边界

```yaml
determination_id: HDP-APLS-016
status: APPROVED
selected_option: A
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-03
what_must_be_decided: 是否批准 TASK-018 按 WP-APLS-CNL-C03-001 实现完整 CNL Frontend、Canonical Frame 收敛、CNL IR 与失败关闭 CLI，并采用所列候选资源上限
why_human_authority_is_required: 该决定授权 Compiler Source/测试修改，并冻结资源耗尽与语义歧义的边界；任一资源数值都会影响 Compiler 对同一 Source 能否完成唯一性证明
```

## 已确认事实

- `DEC-014` 已确定受控自然语言是唯一公开 Surface，用户默认不接触 DSL/AST/IR；
- `DEC-017` 允许中间多候选，但要求全部有效候选收敛为唯一 Canonical Frame；
- `DEC-018` 保留 Rust + LALRPOP，并要求完整候选分析；
- `DEC-019` 已批准独立 `apls-cnl-ir-0.1`、Semantic ID、SHA-256、名义单位和 Provenance 规则；
- 当前 `07_src` 是旧 DSL 原型，尚不能编译公开 CNL；
- 旧资源表只覆盖单 Token Stream，不能自行解释为候选 Lattice/Stream/Frame 上限；
- 当前依赖闭包已经足以实现本工作包，无需新增依赖。

## Option A — 批准完整 CNL 垂直切片（推荐）

批准 `WP-APLS-CNL-C03-001`：

1. C03 可修改该工作包明确列出的 Compiler/CLI Source、最小测试和 Lite 证据文件；
2. 一次实现当前 `apls-zh-CN-0.1` 全部公开句式，不发布临时子集；
3. 实现完整候选 Lattice、逐流零冲突 LALRPOP Parse、确定绑定/定型和最终 Frame 收敛；
4. 实现并复验 `apls-cnl-ir-0.1`，旧 DSL 不再是公共成功入口；
5. 采用工作包第 3 节的精确候选资源上限；首次超限必须为 `APLS-T0007`，不得误报 `AMBIGUOUS`；
6. 不改变依赖、公共语言契约、CLI 契约或旧 IR Schema。

优点：形成首个真正可执行的“可编译自然语言”端到端闭环，直接验证 APLS 的核心差异。

代价：工作包覆盖完整 Frontend 到 Verified IR，代码改动会明显大于先前旧 DSL 单阶段任务；必须严格按七步顺序和八组定向验证控制。

## Option B — 只批准 Source 到 Canonical Frame

只实现 Source、候选分析、Parser、Binding、Type/Unit 和 Convergence Gate；暂不实现 CNL IR、Publisher 和完整 CLI 成功路径。

优点：单次代码范围较小。

代价：项目仍不能交付 Agent 可消费的 Verified IR，还需第二次实现授权；两个工作包之间更容易形成临时接口漂移。

## Option C — 要求修改范围或资源值

不批准当前工作包。由项目负责人指出需调整的句式范围、阶段边界或具体资源数值后重新提交。

## 推荐理由

推荐 `OPTION A`。语言、Frame 和 IR 契约现已闭合，当前最有价值的证据不是继续扩写设计，而是用一个无临时公开子集的完整垂直切片证明：自然表达可以保留多个中间分析，但只有最终语义唯一时才能产生 Verified IR。

## 裁决结果

项目负责人于 `2026-09-03` 明确回复：

```text
HDP-APLS-016: APPROVED
OPTION: A
```

本批准形成 `DEC-020`，并授权 `TASK-018` 在 `WP-APLS-CNL-C03-001` 的精确范围内修改 Compiler/CLI Source、最小测试和 Lite 执行证据。未列出的副作用仍不授权。

## 批准后果

- `APPROVED / OPTION A`：形成 `DEC-020`，关闭 `Q-017`，`TASK-018` 进入 `IN_PROGRESS`，C03 按工作包实现并运行最小定向验证；
- `APPROVED / OPTION B`：另建仅到 Canonical Frame 的收缩工作包，IR/CLI 继续阻塞；
- `CHANGES_REQUESTED`：按指出的范围或数值修订后重新裁决；
- `DEFERRED`：`TASK-018` 保持 `READY`，不修改代码；
- `REJECTED`：取消当前实现包，重新设计实现边界。

## 明确未授权

- 不授权新增/升级依赖、改变 Cargo Lock、Rust/MSRV/LALRPOP 技术决定；
- 不授权修改公开 CNL Grammar、Frame、诊断、IR 契约或旧 `apls-ir-0.1.schema.json`；
- 不授权 Import、多语言、LLM 规范解析、模糊匹配、后端代码生成、LSP 或 IDE；
- 不授权 Commit、Push、Baseline Adoption、Formal C04、Release 或 Formal Seal。

## 可复制回复格式

```text
HDP-APLS-016: APPROVED
OPTION: A
```
