# APLS 0.1 CNL Frontend / Frame / IR 实现工作包

- Work Package ID：`WP-APLS-CNL-C03-001`
- Task：`TASK-018`
- 状态：`OUTPUT_READY_BY_C03`
- 日期：`2026-09-03`
- 责任角色：`C03`
- 上游决定：`DEC-010`、`DEC-011`、`DEC-014`～`DEC-023`
- TASK-019 已采用输入：`DES-APLS-CNL-SEMVAL-001`、`DES-APLS-CNL-RESOURCE-001`、`DES-APLS-CNL-UNICODE-001`、`apls-cnl-diagnostic-0.1`

## 1. 目标

把当前 `DEC-014` 前旧 DSL 原型迁移成 APLS 0.1 唯一公开的简体中文受控自然语言编译路径，并实现从 Source 到经复验 `apls-cnl-ir-0.1` 的完整失败关闭垂直切片。

```text
单 Entry CNL Source
  -> Unicode / Sentence Validation
  -> Category-neutral Declaration Bootstrap / Terminology Graph
  -> Candidate Lexical Lattice
  -> Complete Token-Stream Enumeration
  -> one zero-conflict LALRPOP CNL Grammar per stream
  -> deterministic Binding + Type/Unit Checking
  -> Canonical Frame Normalization
  -> 0 / 1 / >1 Convergence Gate
  -> CNL IR Mapping + Canonical Writer
  -> Schema + Cross-node Revalidation
  -> VerifiedArtifact or explicit failure
```

用户公开体验只能是受控自然语言与面向自然语言的诊断。AST、候选路径、Canonical Frame 和 JSON IR 默认不可见；只有用户主动请求 Explain/IR 时才可展示内部视图。

## 2. 实现范围

### 2.1 必须完成

1. 完整实现 `apls-zh-CN-0.1` 当前 Grammar 的 Profile 声明、七类声明句、规则、转换、不变量、验收和说明句；不发布临时语言子集。
2. 验证 UTF-8、无 BOM、Unicode 17.0.0 NFC、允许字符/空白/标点、句界、引号和术语长度，并保持 UTF-8 Byte Span；完整 NFC 路线按 `DES-APLS-CNL-UNICODE-001`。
3. 按 `DES-APLS-CNL-RESOURCE-001` 先以无类别 `DECLARED_TERM` 和同一正式 Grammar Entry 解析全部声明 Candidate，收集全部 Provisional Header 后统一绑定并冻结声明/别名图，再为行为句构造全部合法 category-labelled 引用 Edge；禁止第二声明 Parser、不计数预扫描、最长匹配、评分、概率、声明顺序或“第一个成功”。
4. 以稳定顺序完整枚举候选 Token Stream，并对每条流调用同一个新的 CNL LALRPOP Grammar；生成冲突必须为零。
5. 用互斥内部类型表达 Parse Candidate、Bound Candidate、Typed Candidate 和 Canonical Frame Candidate；失败阶段不能构造下游 Artifact。
6. 实现 Owner、Symbol Category、Action/Target、State、Type、Unit、Condition 合取、Transition 有限检查、Unit Materialization、Source Map/Span 和必需 Frame Role 的确定验证；候选合同按 `DES-APLS-CNL-SEMVAL-001`。
7. 按 `DEC-017` 对全部有效候选做规范化和等价类收敛：0 类给出具体 Source Error，1 类接受，2 类及以上使用 `APLS-E1310` 拒绝。
8. 按 `DEC-019` 实现 `apls-cnl-ir-0.1` 的 Semantic ID、匿名节点 SHA-256、文档领域语义摘要、名义单位、重复匿名节点合并与 Provenance 稳定并集。
9. 实现 Canonical JSON Writer、权威 Schema Byte 镜像、JSON 重新解析、Schema 验证和跨节点不变量验证；只有完整复验后才能构造 `VerifiedArtifact`。
10. 将批准的 `parse/check/emit-ir/diagnose` CLI 契约接到唯一 CNL Pipeline；旧 DSL 路径不得作为成功公共入口。

### 2.2 允许修改

- `07_src/crates/apls-compiler/src/`：CNL Frontend、Frame、IR、验证、诊断、资源计数及最小单元测试；
- `07_src/crates/apls-compiler/build.rs`：确保唯一执行 Grammar 为 CNL Grammar；
- `07_src/crates/apls-cli/src/`：接入现有批准命令，不改变命令名、Exit Code 或 stdout/stderr 契约；
- `07_src/crates/apls-compiler/Cargo.toml`：声明 Build-time Schema 文件跟踪；只有 `HDP-APLS-018` 精确批准后，才可增加 `unicode-normalization =0.1.25, default-features=false`；
- `07_src/Cargo.lock`：只有同一批准生效后，才可生成并审查 Unicode 候选的实际 Lock/Feature/Checksum/License/MSRV 证据；解析结果不符合候选即停止；
- `08_tests/`：仅新增本工作包所需的最小端到端 fixture/test；
- 与上述实现直接对应的 Lite 状态与证据记录。

### 2.3 明确不在范围

- 当前仍不得新增或升级依赖，不改变 Rust Edition、MSRV、LALRPOP 模式或 Cargo Lock；`unicode-normalization =0.1.25` 只是 TASK-019 候选，只有 `HDP-APLS-018` 批准后才进入本工作包，且实际 Lock/MSRV/许可验证失败会立即重新阻塞；
- 不实现 Import、多文件 Source Graph、英文 Profile、LLM 解析、模糊匹配、自动纠错、LSP、IDE、后端代码生成或参考运行时；
- 不修改旧 `apls-ir-0.1.schema.json`，不把旧 DSL 保留为第二公开 Source；
- 不改变已批准 Grammar、Frame、CNL IR、诊断语义或 CLI 公共契约；发现矛盾必须停止相关实现并上报；
- 不 Commit、Push、建立 Baseline、发起 Formal C04、Release 或 Formal Seal。

## 3. 候选分析资源模型（已由 `HDP-APLS-016 Option A` 批准）

下列数值由 `DEC-020` 固定。达到上限可继续，首次试图超过时返回 `APLS-T0007` / Exit `2`，且不得产生 Frame 或 IR。资源耗尽不等于 `AMBIGUOUS`。计数对象、Token Kind Rank、AST/Syntax Node Taxonomy、两阶段顺序、Primary Span 和 Payload 以 `DES-APLS-CNL-RESOURCE-001` 为唯一细化合同。

| 资源 | 上限 | 唯一计数规则 | 最小证据 |
|---|---:|---|---|
| CNL Lexical Lattice Edge | `1,000,000` / Document | 规范 Edge Identity 计 1；非 Symbol 使用空字符串 Sentinel | CLI 黑盒边界/超限 |
| Complete Token Stream | `4,096` / Sentence | 每个完整覆盖该句且符合词法约束的不同 Edge 序列计 1；即使后续 Parse 失败也计数 | CLI 黑盒边界/超限 |
| Enumerated Candidate Token Occurrence | `1,000,000` / Document | 每条完整 Stream 在调用 Parser 前按全部 Edge 计数；失败 Parse 仍计完整 Stream | CLI 黑盒边界/超限 |
| Parse Candidate AST Node | `1,000,000` / Document | 按 `DES-APLS-CNL-RESOURCE-001` 的固定 Syntax Node 表计数，不依赖 Rust 类型粒度 | 受 Token 支配；Module Ledger 注入 |
| Bound Frame Candidate | `1,000,000` / Document | 每个通过全部名称与类别绑定的完整 Sentence Frame 候选计 1 | 受 Token 支配；Module Ledger 注入 |
| Typed Frame Candidate | `1,000,000` / Document | 每个通过类型、单位和角色检查的完整 Sentence Frame 候选计 1 | 受上游支配；Module Ledger 注入 |
| Canonical Frame Candidate | `1,000,000` / Document | 每次完成规范化的完整 Sentence Frame 候选计 1；去重前计数 | 受上游支配；Module Ledger 注入 |

既有 `DEC-011` Source Byte、公共诊断、表达式深度和 IR Byte 上限继续有效。旧 `MAX_TOKENS/MAX_AST_NODES` 在公共 CNL 路径中分别按本表的 Candidate Token Occurrence / Parse Candidate AST Node 语义执行，不得继续按单一旧 DSL Token Stream 解释。

后四类上限是生产防御性不变量，不是可独立触发的 CLI 承诺。测试辅助注入只允许存在于 `#[cfg(test)]` 私有 Module，调用生产相同登记函数；不得进入公共 API/Release Binary 或绕过公开 Pipeline 上限。普通 Source 必须同时验证 Resource Profile 的支配关系，先行 Token T0007 后立即终止。

## 4. 实现顺序与中间边界

1. CNL Source/句界、无类别 Declaration Bootstrap、Provisional Header 收集和声明索引；
2. 候选词法 Lattice、Enumerator、资源计数；
3. 唯一 CNL LALRPOP Grammar 与 Candidate AST；
4. Binding、Type/Unit 和 Canonical Frame；
5. Convergence Gate 与 CNL 诊断；
6. CNL IR、Canonical Writer、Schema/跨节点复验；
7. Pipeline/CLI 接线与端到端验证。

中间步骤可以在私有测试中验证，但任何步骤都不得被宣称为公开 Compiler 完成；只有第 1～7 项全部完成后 `TASK-018` 才能进入 `OUTPUT_READY`。

## 5. 最小定向验证

按测试治理，优先用八组高价值行为证据覆盖 P0 风险，不做覆盖率填充：

1. 完整合法文档覆盖全部 Sentence/Frame 类型，并产生可重新验证的 CNL IR；
2. 三种条件引导表面表达以及多 Tokenization/Parse 候选收敛为同一 Canonical Frame/Hash；
3. 两个不等价 Canonical Frame 产生 `APLS-E1310`，且至少包含两个稳定分歧见证；
4. 零有效候选分别由术语、类别/Owner、类型/单位或必需角色的具体错误解释，不误报 `AMBIGUOUS`；
5. 声明 ID、匿名节点 ID、文档语义 Hash 在同义表达和非语义 Provenance 变化下满足批准的稳定性规则；
6. 等价重复匿名节点合并且 Provenance 取稳定并集；名义自定义单位只与自身兼容，未知换算拒绝；
7. 公开可达候选资源通过 CLI 验证边界/首次超限；受支配的 Syntax/Bound/Typed/Canonical 计数器通过私有 Module Ledger 注入验证相同生产登记函数，并验证普通 Source 的支配不变量；任何 T0007 都不产生 Frame/IR；
8. Schema/跨引用/Source 竞态或发布失败时无 `VerifiedArtifact`，CLI Exit/stdout/stderr 保持批准契约。

最小命令集：

```text
cargo fmt --check
cargo check --workspace --locked
cargo test -p apls-compiler --locked
cargo test -p apls-cli --locked
```

不默认运行性能、压力、Fuzz、跨平台或未获授权的完整发布测试。

## 6. 完成证据

- CNL Grammar 生成成功且零冲突；
- 八组最小行为证据通过；
- `cargo check/test --locked` 未改变 `Cargo.lock`；
- 权威 Schema 与运行时使用 Byte 一致；
- `git diff --check` 与最终差异审查通过；
- 旧 DSL 不可通过公共命令生成成功 Artifact；
- 状态只能进入 `OUTPUT_READY`，不冒充 Baseline、C04 或 Release。

## 7. 当前 Gate

`HDP-APLS-018 Option A` / `DEC-023` 已精确采用 TASK-019 公共语义、Schema 与 Unicode 依赖边界。本工作包已完成 C03 实施与最小定向验证，当前为 `OUTPUT_READY`；实施证据见 `11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md`。其他未授权边界保持不变。
