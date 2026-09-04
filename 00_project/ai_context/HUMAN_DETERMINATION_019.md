# HDP-APLS-019 — TASK-018 实现整改与 Unicode 传递 Feature 闭包裁决

```yaml
determination_id: HDP-APLS-019
status: APPROVED
selected_option: A
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-04
```

## 1. 必须决定什么

是否接受 Cargo 对已锁定依赖解析出的实际传递 Feature 闭包，并授权仅在 `IIR-APLS-TASK018-IMPLEMENTATION-001` 的 `IO-01～IO-07` 范围内重开 `TASK-018` C03 Retry。

## 2. 为什么需要项目负责人裁决

`IO-02～IO-07` 都是当前已批准契约内的实现缺陷，可以直接按原契约整改；但 `IO-01` 涉及已批准依赖边界：原文字写明 `tinyvec 1.6.0` “只启用 `alloc`”，Cargo 实际解析为 `alloc + default + tinyvec_macros`。`tinyvec/default` 在锁定版本中是空 Feature，不增加代码能力，但它仍属于实际激活 Feature，不能由实现者静默忽略或自行修改已批准边界。

## 3. 已确认事实

- 独立增量复审结论：`CHANGES_RECOMMENDED`；Target Digest 三次匹配 `b4e377fe844ffd55403bf8a74bb4954107fef4dfe3719e2dfa461e1bd3952658`。
- `unicode-normalization 0.1.25` 直接依赖保持 `default-features=false`，其 `std` Feature 未启用。
- 该 Crate 对 `tinyvec` 的依赖声明启用 `alloc`，但未关闭 `tinyvec` 的默认 Feature。
- `tinyvec 1.6.0` 的 Feature 定义为：`default = []`、`alloc = ["tinyvec_macros"]`。
- 因此 Cargo 的真实闭包是：`tinyvec/alloc`、`tinyvec/default`（空集合）和 `tinyvec/tinyvec_macros`；版本、Checksum、License 和 MSRV 证据没有变化。
- `IO-02～IO-07` 分别涉及 Parse 接受边界、候选 Provenance 并集、IR 跨节点复验、Bound 资源计数、诊断 Span 和 stdout `APLS-T0005`，均不需要改变产品目标或已批准语言语义。

## 4. 选项

### Option A — 接受真实空 Default Feature，并执行精确 C03 Retry（推荐）

- 将批准边界更正为锁定版本的实际闭包：
  `unicode-normalization/default-features=false`；
  `tinyvec = 1.6.0` 激活 `alloc + default(empty) + tinyvec_macros`；
  不启用 `unicode-normalization/std` 或 `tinyvec/std`。
- 不更换版本、不修改 Checksum、不新增依赖、不修改 `Cargo.lock`。
- 修正 Unicode Profile 和 C03 验证记录，使证据与 Cargo 实际解析一致。
- 授权 C03 仅整改 `IO-02～IO-07` 及 `IO-01` 的证据/契约同步，补充直接关闭用例，并形成新的精确增量 Target。

优点：保持既有依赖和实现路线；`default=[]` 不增加运行代码或功能；改动最小且可机械复核。

### Option B — 强制维持“只有 alloc”的原文字面边界

- 通过受控 Patch/Fork/Vendoring 修改传递依赖声明，显式关闭 `tinyvec` 默认 Feature；
- 重新审查供应链来源、源码摘要、License、构建可复现性和维护责任。

缺点：为消除一个空 Feature 标志引入新的供应链分叉和长期维护成本，风险明显高于收益。

### Option C — 暂缓

保持 `TASK-018 = BLOCKED`，不修改代码、测试、依赖契约或验证记录。

## 5. 推荐

推荐 `Option A`。它忠实记录 Cargo 的真实解析结果，没有把空 `default` 冒充为未激活，也不会改变二进制能力、语言行为、依赖版本或锁文件。

## 6. 批准后的边界

批准 Option A 后：

- C00 可将 `TASK-018` 从 `BLOCKED` 重开为 `IN_PROGRESS`；
- C03 可修改与 `IO-01～IO-07` 直接相关的设计文字、Compiler Source、最小测试和 C03 验证记录；
- 可运行普通锁定构建/测试和每项 Finding 的最小关闭探针；
- 整改完成后只能回到 `OUTPUT_READY`，并由另一个全新独立 Session 执行 `DELTA_ONLY` 再复审。

## 7. 明确不授权

- 不改变 APLS 公开语言目标、Canonical Frame/IR 语义、资源数值、Unicode 版本或 CLI 命令集合；
- 不增加/升级依赖，不修改锁定版本或 Checksum；
- 不授权 Commit、Push、Baseline Adoption、Formal C04、C05、Release 或 Formal Seal；
- 不授权复审与 `IO-01～IO-07` 无关的历史已关闭事项。

## 8. 不同回复的后果

- `APPROVED / OPTION A`：按最小 Retry Scope 修复七项并增量再复审；
- `APPROVED / OPTION B`：先形成新的供应链变更工作包，不立即改代码；
- `DEFERRED`：保持阻塞；
- `REJECTED`：停止当前依赖路线并返回架构裁决。

## 9. 权威来源

- `05_reviews/INFORMAL_INDEPENDENT_REVIEW_TASK018_IMPLEMENTATION.md`
- `04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md`
- `00_project/ai_context/HUMAN_DETERMINATION_018.md`
- `07_src/Cargo.toml`
- `07_src/Cargo.lock`

## 10. 可复制回复

```text
HDP-APLS-019: APPROVED
OPTION: A
```

## 11. 裁决结果

项目负责人于 `2026-09-04` 明确批准 Option A。本批准形成 `DEC-024`：接受 `tinyvec 1.6.0` 的实际 `alloc + default(empty) + tinyvec_macros` Feature 闭包；不改变依赖版本、Checksum、Lock、Unicode 版本或运行能力；仅针对 `IO-01～IO-07` 重开 TASK-018 C03 Retry。未列出的副作用仍不授权。
