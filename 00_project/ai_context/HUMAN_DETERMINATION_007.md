# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-007
TARGET: DES-APLS-COMPILER-FOUNDATION-001
DECISION: APPROVED
SELECTED_OPTION: A
DECISION_DATE: 2026-09-02
DECISION_EVIDENCE: 项目负责人“HDP-APLS-007: APPROVED / OPTION: A”
```

## WHAT_MUST_BE_DECIDED

批准或调整 APLS 0.1 Rust Compiler 实现基础，包括：

- Rust 2024、参考 Rust 1.98.0、MSRV 1.86.0 和 Cargo Resolver 3；
- 精确直接依赖版本/Feature 闭包与 `Cargo.lock --locked`规则；
- `apls-compiler` / `apls-cli` 边界和互斥 Stage Artifact；
- 手写 Lexer + LALRPOP 默认 LR(1) 的零冲突生成门禁；
- Canonical JSON 尾 Byte、离线 Schema 复验、Publisher 和固定资源上限；
- 对 stdout 无法撤回已写前缀的公共契约修正。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

精确依赖和资源值会决定实现、兼容、供应链和可接受输入边界。Canonical JSON 终结 Byte 会改变 Artifact 精确字节；stdout 修正会精炼已批准公共契约。这些不能由实现者默认决定。

## CONFIRMED_FACTS

- `DEC-004` 确立“拒绝歧义”为首要目标；
- `DEC-009` 已批准 CLI、Exit Code、Diagnostic Envelope、Tool Diagnostic 和 Verified IR 发布契约为设计输入；
- `DEC-010` 已选择 Rust + LALRPOP 默认 LR(1)，但没有选择精确版本或其他库；
- Rust 官方当前 Stable 为 1.98.0，Rust 2024 Edition 从 1.85.0 可用；
- LALRPOP 0.23.1 的 crates.io 元数据声明 MSRV 1.86，比发布说明中 0.23 线的 1.85 更严格；
- `jsonschema` 默认 Feature 包含 HTTP/File Resolution，必须禁用才与 Compiler 无网络边界一致；
- stdout/Pipe 可能在部分 Byte 已被外部观测后才失败，因此无法实现“任意输出故障都保证 stdout 为空”；
- 本包只查阅官方资料和形成设计，没有安装、构建或实测任何依赖。

## OPEN_QUESTIONS

- 本裁决包内无未决项；
- 项目负责人已选择 `OPTION A`；`OPTION B/C` 未被采用。

## OPTIONS_AND_DIFFERENCES

### OPTION A — 精确锁定 + 受控 stdout（推荐）

- 接受 `DES-APLS-COMPILER-FOUNDATION-001` 第 2～12 节的精确工具链、依赖、边界、资源上限和门禁；
- Canonical IR 顶层 `}` 是最后 Byte，不追加 LF；
- 保留 `emit-ir --output -`；只有 Exit `0` 且完整可验证的 stdout 才是已交付 IR，非零退出的任何前缀一律无效。

### OPTION B — 精确锁定 + 仅文件 IR

- 工具链、依赖、边界、资源上限和无尾 LF 与 Option A 相同；
- 从 0.1 公共契约移除 `emit-ir --output -`，只支持可原子替换的文件交付；
- 交付完整性最强，但降低 Agent/Unix 管道集成便利性并改变已批准 CLI。

### OPTION C — 延后精确实现边界

- 保留 Rust + LALRPOP 方向，但不批准精确版本、依赖、资源上限、尾 Byte 或 stdout 修正；
- 不得进入可重现的 Compiler 骨架实现，需先返回 C02 重新准备候选。

## RISKS_AND_TRADEOFFS

- Option A 保留标准管道，但调用者必须在进程成功前不消费中间前缀；
- Option B 的交付模型更强，但会收窄已批准的公共 CLI；
- 精确直接版本降低静默漂移，也意味安全/缺陷修复必须通过受控升级；
- 当前上限是设计值，未经性能原型验证；实测若不成立必须回到裁决，不得在代码中静默改值。

## RECOMMENDED_OPTION_AND_REASON

推荐 `OPTION A`。它用精确工具链、功能开关、互斥类型和资源计数把“拒绝歧义”落到实现边界，同时对 stdout 的物理限制给出真实、可验的验收条件，不依赖无法撤回的隐含假设。

## APPROVED_CONSEQUENCE

- 将所选 Option 记录为新的当前有效决定；
- 将 `DES-APLS-COMPILER-FOUNDATION-001` 标记为已批准设计输入，仍不是产品 Baseline；
- 允许 C00 准备 C03 Compiler 骨架实现工作包的范围与授权候选；
- 不自动安装依赖或开始写代码。

## CHANGES_REQUESTED_CONSEQUENCE

`TASK-006` 返回 `IN_PROGRESS`，只修改被指定的版本、Feature、边界、上限或契约。

## DEFERRED_CONSEQUENCE

保持本候选与待裁决状态，不进入实现骨架工作包。

## REJECTED_CONSEQUENCE

关闭当前实现基础候选，由 C02 按项目负责人指定的新边界重建方案。

## EXPLICITLY_NOT_AUTHORIZED

本裁决不授权安装 Rust/Crate、创建 Compiler 代码、选择语义哈希或参考后端、更改产品目标，也不授权 Commit、Tag、Baseline Adoption、Formal C04、Push、Release、远程修改或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `DEC-004`、`DEC-009`、`DEC-010`；
- `04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md`；
- `04_design/compiler/APLS_0.1_IMPLEMENTATION_FOUNDATION_DESIGN.md`；
- 设计文档第 14 节所列官方一手资料。

## COPYABLE_RESPONSE_FORMAT

推荐回复：

```text
HDP-APLS-007: APPROVED
OPTION: A
```

或：

```text
HDP-APLS-007: CHANGES_REQUESTED
修改：<具体版本、Feature、上限或契约>
```
