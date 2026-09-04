# APLS 0.1 Compiler 实现基础设计

- 设计 ID：`DES-APLS-COMPILER-FOUNDATION-001`
- 状态：`FRONTEND_TECHNOLOGY_REVIEW_REQUIRED_BY_DEC-014`
- 日期：`2026-09-02`
- 输入：`DEC-004`、`DEC-009`、`DEC-010`、`DES-APLS-COMPILER-001`、`ADR-APLS-001`
- 裁决包：`HDP-APLS-007`
- 批准依据：`HDP-APLS-007 / OPTION A`、`DEC-011`

> 本文已获准为 Compiler 骨架实现的受控设计输入，但不是产品 Baseline，也不代表依赖已安装、Compiler 已建立或行为已验证。

> Rust Workspace、互斥 Artifact 和确定性门禁仍是复用候选；LALRPOP 与旧 ASCII Lexer 对 CNL Grammar 的适配性必须在新 Grammar 形成后重新证明。

## 1. 目标与强制原则

本设计把 Rust + LALRPOP 技术决定收敛为可实现的 Compiler 骨架，首先服务 `DEC-004`：不能唯一解释的 Source 必须拒绝，不得由 Parser 顺序、容错节点、库默认值、遍历顺序或 AI 猜测隐式选择。

强制原则：

- 每个 Stage 只能产生一种成功 Artifact 或一种显式失败；
- 上游失败 Artifact 在类型上不能进入下游；
- 只有从实际 Canonical JSON Byte 重新解析并复验成功才能构造 `VerifiedArtifact`；
- 生成器冲突、生成失败或未经允许的生成模式都必须使构建失败；
- 直接依赖、工具链、Schema Draft 和资源上限由 Compiler Version 固定；
- 不暴露任何第二机器契约；Surface AST、Bound/Typed/Checked Model 都是私有类型。

## 2. 已批准工具链

| 项 | 批准值 | 约束 |
|---|---|---|
| Rust Edition | `2024` | Workspace 统一，不得由 Crate 各自选择 |
| 参考工具链 | `Rust 1.98.0` | 批准时已核验的 Stable；后续升级必须显式审查 |
| MSRV | `1.86.0` | `Cargo.toml` 中声明 `rust-version = "1.86"`，CI 同时验证 MSRV 与参考工具链 |
| Cargo Resolver | `3` | 在顶层 Workspace 声明 |
| Dependency Lock | 提交 `Cargo.lock` | 构建/检查/测试使用 `--locked`；受控离线门禁使用 `--frozen` |
| 版本需求 | 直接依赖使用 `=x.y.z` | 禁止 Git Dependency、Wildcard 和未裁决的自动升级 |

Rust 1.98.0 于 `2026-08-20` 发布。Rust 2024 Edition 从 Rust 1.85.0 起可用。LALRPOP 0.23.1 发布说明仍记载 0.23 线的 MSRV 为 1.85，但 crates.io 0.23.1 机器元数据声明 `rust_version = 1.86`。为拒绝双重口径，本设计取两者更严格的 `1.86.0`。

`=x.y.z` 会降低作为通用库时的依赖兼容弹性，但 APLS Compiler MVP 是最终应用而非通用库；此处优先保证评审 Target 与构建输入精确。

## 3. 直接依赖闭包

### 3.1 Build Dependency

```toml
[build-dependencies]
lalrpop = { version = "=0.23.1", default-features = false }
```

### 3.2 Runtime Dependency

```toml
[dependencies]
lalrpop-util = { version = "=0.23.1", default-features = false, features = ["std"] }
serde = { version = "=1.0.229", features = ["derive"] }
serde_json = { version = "=1.0.151", features = ["arbitrary_precision"] }
clap = { version = "=4.6.6", default-features = false, features = ["std", "help", "usage", "error-context", "derive"] }
jsonschema = { version = "=0.52.1", default-features = false, features = ["arbitrary-precision"] }
sha2 = { version = "=0.11.0", default-features = false }
num-bigint = { version = "=0.5.1" }
num-traits = { version = "=0.2.19" }
tempfile = { version = "=3.27.0" }
```

当前受控闭包不直接引入 `anyhow`、`thiserror`、`miette`、`indexmap` 或 `regex`。它们不是被永久禁用，但新增时必须说明不能由当前闭包满足的必要性，并重新评估诊断稳定性与供应链面。

### 3.3 功能开关约束

- LALRPOP 与 `lalrpop-util` 必须同版；禁用内置 Lexer，使用 APLS 手写 Lexer 实现精确 Token、Span 和稳定错误。
- `serde_json` 不启用 `preserve_order` 或 `unbounded_depth`；Canonical Writer 不依赖普通 `to_string` 的未声明行为。
- `clap` 不启用 color/suggestions 等会改变文本或候选项的功能；库错误必须映射为稳定 `APLS-T0001`，不把库原文当公共诊断。
- `jsonschema` 禁用默认的 HTTP/File Resolver，显式选用 Draft 2020-12；Compiler 不通过 `$id`、`$schema` 或 `$ref` 访问网络/外部文件。
- 任何传递依赖变化都必须通过 `Cargo.lock` Diff、许可证复核和最小 Conformance 验证；本文不将传递依赖组合冒充为已审计供应链。

## 4. Workspace 与 Crate 边界

后续实现工作包在 `07_src/` 建立两个 Crate：

```text
07_src/
├── Cargo.toml                 # virtual workspace, resolver = "3"
├── Cargo.lock                 # 受控依赖图
└── crates/
    ├── apls-compiler/         # 纯编译核心与公共 Compiler API
    └── apls-cli/              # argv、主机文件系统、stdout/stderr、Publisher
```

### 4.1 `apls-compiler`

模块与单一职责：

| Module | 职责 |
|---|---|
| `limits` | Compiler Version 固定的资源上限与计数器 |
| `diagnostic` | 内部 Finding、公共 Diagnostic 归一化和稳定排序 |
| `source` | 逻辑路径、Source Graph、字节与 SHA-256 指纹 |
| `lexer` | 手写确定 Lexer、Token、Span |
| `parser` | LALRPOP 生成 Parser 适配与 Parse Error 归一化 |
| `ast` | 只在 Crate 内部可见的 Surface AST |
| `index` / `resolve` | 声明索引和唯一名称绑定 |
| `types` | 类型、数值和工程单位唯一检查 |
| `semantic` | State/Rule/Safety/Channel/约束冲突检查 |
| `ir` | 已检查 Model 到 IR 候选的全映射 |
| `canonical` | 精确 Canonical JSON Byte Writer |
| `validate` | Byte Reparse、Draft 2020-12 Schema 和跨节点不变量 |
| `pipeline` | Stage 编排和互斥最终结果 |

`apls-compiler` 不读 argv、当前目录、环境变量或 stdout/stderr；主机 Source Reader 通过受控接口注入，读取到的数据立即转换为不可变 Byte Buffer 与逻辑路径。

### 4.2 `apls-cli`

| Module | 职责 |
|---|---|
| `command` | 已批准 CLI 参数到 `CompileRequest` 的唯一映射 |
| `host_fs` | Entry/Import 文件边界和路径安全检查 |
| `publish` | 文件原子替换、stdout 交付和 Source 竞态复核 |
| `render` | Human/JSON 诊断与 stdout/stderr 约束 |

CLI 不得跳过 Core Validator，也不得把 `serde_json::Value`、AST 或 Checked Model 直接写出。

## 5. 互斥 Artifact 和失败类型

每个成功 Stage 使用拥有型、字段完整的独立结构，前一 Stage 被消费后才能构造下一 Stage：

```text
LoadedBundle
  -> ParsedProgram
  -> BoundProgram
  -> TypedProgram
  -> CheckedProgram
  -> CanonicalCandidate
  -> VerifiedArtifact
```

- 禁止用“一个大结构 + 大量 `Option<T>` 字段”模拟 Stage；
- `VerifiedArtifact` 构造器是私有的，只有 `validate` 可返回；
- `Publisher` 只接受 `VerifiedArtifact`，不接受字节数组或中间 Model；
- 内部错误 Enum 必须被穷尽映射，不以依赖库错误文本作为稳定身份。

对外最终结果固定为：

```text
CompileOutcome = Accepted(VerifiedArtifact)
               | Rejected(OrderedDiagnostics)
               | ToolFailure(OrderedDiagnostics)
               | InternalFailure(OrderedDiagnostics)
```

`CompileOutcome` 必须穷尽映射到已批准的 Exit Code `0/1/2/3`，不存在“继续但不确定”分支。

## 6. Lexer 与 Parser 生成门禁

### 6.1 手写 Lexer

- Lexer 只接受 UTF-8 无 BOM Byte，按最长匹配和关键字优先级生成 Token；
- Token 携带 Byte Offset；Line/Column 从同一 Source Index 按 Unicode Scalar Value 计算；
- 不使用 Locale、Unicode 正规化、近似匹配或库的隐式容错；
- 每个非法字节/字符只通过 APLS 定义的稳定 Code 归类。

### 6.2 生成与零冲突

- `build.rs` 使用固定 `lalrpop::Configuration`处理正式 `.lalrpop` Grammar；生成文件只写入 Cargo `OUT_DIR`，不提交、不手改；
- 生成器错误或 Grammar 冲突直接使 Build Script 非零结束；不允许“预期冲突数”或忽略冲突的配置；
- `LALRPOP_LANE_TABLE` 存在时构建失败，防止环境变量把已决定的默认 Lane-Table LR(1) 替换为 Legacy LALR 模式；
- 构建系统不将冲突输出当作 Warning 继续；Grammar 候选的唯一合格值是零冲突；
- 首版不启用 Parser Error Recovery。后续如启用，必须另行设计只到 Diagnostic Engine 的单向路径，恢复节点在类型上不能进入 Resolver。

## 7. Canonical JSON 和 IR 复验

### 7.1 字节串行为

Canonicalizer 不把 Rust Struct 直接 `serde_json::to_string` 后发布，而是：

1. 从 `CheckedProgram` 构造字段完整的 IR Value；
2. 递归确认 Object Key 按 Unicode Code Point 顺序；Rust UTF-8 `str` 字节序对合法 Scalar 的词典序与 Code Point 序一致，但仍必须有边界验证；
3. 用专用 Writer 实现 `APLS-C14N-0.1` 转义、禁止不必要空白；
4. 拒绝 JSON Float；`decimal` 始终为规范字符串，整数在内部使用任意精度表示；
5. 输出的最后一个 Byte 是顶层 `}`，不追加 LF。

第 5 项已由 `HDP-APLS-007 Option A` / `DEC-011` 批准，并纳入 `APLS-C14N-0.1`。

### 7.2 唯一 Validator 路径

1. 从实际待交付 Byte 使用 `serde_json` + `arbitrary_precision` 重新解析；
2. 明确以 Draft 2020-12 编译并执行权威 Schema `04_design/ir/apls-ir-0.1.schema.json`；
3. 执行跨节点引用、类别、排序、类型、单位和 Source Map 不变量；
4. 所有库错误归一为 APLS 稳定 Code，按 APLS 排序键排序；库文本不进入公共诊断身份；
5. 只有全部成功时构造 `VerifiedArtifact`。

Schema 的唯一编辑 Source Truth 是已批准设计输入 `04_design/ir/apls-ir-0.1.schema.json`。后续 Crate 中如需嵌入镜像，镜像必须由构建步骤复制/生成并执行 Byte Equality Gate，禁止手工维护两份 Schema。

## 8. Publisher 与可观测交付

### 8.1 文件输出

- 在目标同目录创建随机命名临时普通文件，不跟随 Symbolic Link；
- 完整写入 `VerifiedArtifact`，执行 `flush` 与文件 `sync_all`；
- 在任何目标变更前重新读取已发现 Source 并比较 SHA-256；
- 使用同文件系统原子 Replace 一次交付；Replace 之后不再执行可使命令转为失败的操作；
- 平台适配器无法保证目标不暴露部分字节时，在改变目标前以 `APLS-T0005` 失败；
- 0.1 的原子性是同一主机文件系统上的可观测整体替换，不承诺掉电后的持久性或已发生 Replace 后的异常进程回滚。

### 8.2 stdout 的物理边界与已批准契约

原 `DES-APLS-COMPILER-001` 要求“任一 Error 时 stdout 为空”。对普通 Pipe/stdout，系统可在部分 Byte 写出后才返回 I/O 失败，Compiler 无法撤回已交给外部进程的前缀。因此该字面契约不可实现，已由 `HDP-APLS-007 Option A` 显式裁决，不使用默认 I/O 行为填补。

`HDP-APLS-007 Option A` / `DEC-011` 已将契约修正为：

- Compiler 只在 IR 完整复验和 Source 竞态复核后开始写 stdout；
- `exit 0` 且 stdout 是完整、可重新验证的 Canonical IR 时，才视为已交付 `Verified IR`；
- stdout 写入失败时返回 `APLS-T0005` / Exit `2`；已暴露的前缀是无效、不可消费的传输残留；
- 调用者必须在消费 stdout 前等待进程结束并确认 Exit `0`；边读边将前缀当作 Verified IR 属于调用方违反契约。

`OPTION B/C` 未被采用。APLS 0.1 保留 Agent/Unix 管道接口，用“进程成功 + 完整验证”而不是不可实现的撤回承诺定义交付。

## 9. Compiler 0.1 已批准固定资源上限

| 资源 | 上限 | 唯一计数规则 |
|---|---:|---|
| 单个 Source | `1,048,576` Byte | 原始文件 Byte，不含外部文件系统元数据 |
| Source Graph 总量 | `16,777,216` Byte | 按唯一逻辑路径之不可变 Byte Buffer 求和 |
| Source 数 | `256` | Entry 计 1，按唯一逻辑路径计数 |
| Import 深度 | `32` | Entry 深度 0，取最长有向 Import 路径的 Edge 数 |
| Token 总数 | `1,000,000` | 全 Source Graph 的非空白、非注释、非 EOF Token |
| Surface AST 节点 | `1,000,000` | 每构造一个有类型 AST Node 计 1 |
| 表达式嵌套 | `128` | Literal/Reference 深度 1，取最长 Expression 父子链 |
| 公共诊断 | `1,000` | 稳定 Stage/路径/源码遍历顺序的前 999 条 + 1 条 `APLS-T0007` |
| Canonical IR 候选 | `33,554,432` Byte | 专用 Writer 产生的完整 Byte Sequence |

- 上表的 `Token 总数/Surface AST 节点` 是 `DEC-014` 前单一旧 DSL Stream 的历史计数表述。对唯一公开 CNL 路径，数值不变，但分别由 `candidate_token_occurrences/parse_candidate_syntax_nodes` 承接，精确计数对象、节点 Taxonomy、两阶段顺序和 T0007 Payload 见 `DES-APLS-CNL-RESOURCE-001`；不得同时执行两套计数或按 Rust Struct 粒度计数。
- CNL 的 `parse_candidate_syntax_nodes` 及 Bound/Typed/Canonical Frame 计数器在当前相同上限和处理顺序下受先行 Token/上游计数支配。它们仍是生产防御性检查，但不承诺存在独立触发其 T0007 的公开 Source；公开可达资源用 CLI 验证，受支配资源用 `#[cfg(test)]` 私有 Ledger 初值注入调用生产同一登记函数验证，且注入不得进入 Release Binary 或公共接口。
- 超限是 Compiler 能力边界，统一为 `APLS-T0007` / Exit `2`，不冒充 Source 语义错误；
- 计数次序是 Entry 优先、Import 按逻辑路径 UTF-8 Byte 升序、各 Source 按 Byte Offset 升序；
- 达到上限本身不失败，试图超过才失败；
- 上限不从 CPU、内存、Locale、环境变量或参数自适应；
- 实现内部可并行，但超限点与保留诊断必须与上述单线程次序等价。

本表已由 `HDP-APLS-007 Option A` / `DEC-011` 批准为 Compiler 0.1 配置，但不是 APLS 语言永久上限。改变任一数值会改变同一输入的可接受性，因此必须随 Compiler Version 受控变更。

## 10. 稳定诊断实现接缝

- 依赖库 Error 首先映射为内部枚举，再由单一 Diagnostic Catalog 构造公共字段；
- 消息来自 Compiler Version 固定的英文模板，不包含 Rust `Debug`、OS 错误原文或主机绝对路径；
- Source Error、Tool Failure 和 Internal Failure 在类型和代码空间上互斥；严重度只能向更高类别提升；
- 无 Source Span 的 Tool Diagnostic 使用明确的 `null` 排序组，不使用宿主路径或错误发生时间打破并列；
- 公共 Diagnostic 统一使用 `DES-APLS-CNL-DIAG-001` 第 1.2 节的 Byte-based 总排序键；Line/Column 只作展示，最终 Tie-breaker 是完整 Diagnostic Canonical Bytes；
- 资源诊断的截断规则与第 9 节一致，不因线程时序变化。

## 11. 最小构建与验证门禁

实现工作包建立后，最少需有以下门禁；具体 Test Case 仍由 C05 设计：

1. `cargo ... --locked` 不得改变 `Cargo.lock`；受控离线环境使用 `--frozen`；
2. 参考 Rust 与 MSRV 均能构建，不使用 `--ignore-rust-version`；
3. LALRPOP 生成完成且冲突为零，生成物仅位于 `OUT_DIR`；
4. 正式 Grammar 与 Parser 实现入口只有一套，手写“快速 Parser”不能成为第二条规范路径；该约束不要求 Source 的中间 Tokenization 或 Parse Tree 唯一；
5. Schema 运行时镜像与权威 Schema Byte 一致；
6. Canonical Writer 重复运行字节一致，关键 Unicode/转义/数字/顺序边界合格；
7. 实际待发布 Byte 必须重新解析、Schema 复验和跨节点复验；
8. 每个 Stage 错误都不能构造下游 Artifact；
9. `PUBLIC_SOURCE_REACHABLE` 资源经 CLI 在边界值成功、首个超限值稳定失败；`DEFENSIVE_DOMINATED` 资源经私有 Module Ledger 注入验证同一生产登记函数的边界/超限/Span/Payload，并由普通 Source 验证支配不变量和先行 T0007 立即终止；
10. Publisher 文件失败不改变旧目标，stdout 交付按裁决后契约处理。

本节只定义验证责任，不宣称任何门禁已实现或通过。

## 12. 升级与变更规则

以下任一变化都必须建立显式变更记录，重跑受影响门禁，不得静默替换：

- Rust Edition、参考 Toolchain 或 MSRV；
- LALRPOP/运行时不同版或生成模式；
- 任一直接依赖版本、Feature 或新直接依赖；
- Canonical JSON Writer、Schema Draft/Source Truth、资源上限或稳定诊断映射；
- 输出原子性、stdout 完整性口径或支持平台。

## 13. 明确不在本设计内

- 不选择语义哈希算法或参考后端；
- 不建立 LSP、Plugin、Daemon、缓存、并行编译保证或远程 Import；
- 不安装工具/依赖，不创建 Compiler 代码，不声称 Build/Test 已通过；
- 不自动进入 C03、Baseline Adoption、Formal C04、Commit、Push、Release 或 Formal Seal。

## 14. 官方核验依据

- [Rust 官方发布索引](https://blog.rust-lang.org/releases/latest/)
- [Rust 2024 Edition 官方说明](https://doc.rust-lang.org/stable/edition-guide/rust-2024/index.html)
- [Cargo Resolver 版本与依赖解析](https://doc.rust-lang.org/cargo/reference/resolver.html)
- [Cargo `--locked` / `--frozen`](https://doc.rust-lang.org/cargo/commands/cargo-build.html)
- [Cargo 精确版本需求](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)
- [LALRPOP 官方发布说明](https://github.com/lalrpop/lalrpop/blob/master/RELEASES.md)
- [LALRPOP Quick Start：Tool/Runtime 同版与 `OUT_DIR`](https://lalrpop.github.io/lalrpop/quick_start_guide.html)
- [LALRPOP Advanced Setup：默认 Lane-Table LR(1)](https://lalrpop.github.io/lalrpop/advanced_setup.html)
- [crates.io 各直接依赖 API 元数据](https://crates.io/)
- [`serde_json::Map` 官方 API 说明](https://docs.rs/serde_json/latest/serde_json/map/index.html)
- [`jsonschema` Draft 2020-12 与 Feature 说明](https://docs.rs/jsonschema/latest/jsonschema/)
- [`tempfile` 原子 persist 与同文件系统约束](https://docs.rs/tempfile/latest/tempfile/struct.TempPath.html#method.persist)

## 15. 裁决结果

`HDP-APLS-007 Option A` 已同时批准：

1. 精确工具链、直接依赖与 Feature 闭包；
2. Crate/Module/Stage Artifact 边界；
3. Parser 生成和零冲突门禁；
4. Canonical JSON 不带尾随 LF、Byte Reparse 和 Schema 离线复验；
5. Compiler 0.1 固定资源上限；
6. stdout 不可撤回问题的公共契约修正选项。

上述内容现为 `DEC-011` 的已批准实现输入。该批准不等于依赖安装、代码实现、验证通过、Baseline Adoption 或 Release 授权。
