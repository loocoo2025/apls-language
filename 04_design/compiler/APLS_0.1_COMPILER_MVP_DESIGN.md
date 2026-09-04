# APLS 0.1 Compiler MVP 详细设计

- 设计 ID：`DES-APLS-COMPILER-001`
- 状态：`FRONTEND_CHANGE_REVIEW_REQUIRED_BY_DEC-014`
- 输入：`ARCH-APLS-001`、`DEC-008`、已批准的 Language/IR/Diagnostic 设计输入
- 适用范围：APLS 0.1 确定性 Compiler MVP

> `DEC-014` 保留失败关闭、Canonical IR 验证和互斥 Stage 原则，但要求将旧 DSL Frontend 替换为确定性 CNL Frontend；相关组件、命令和诊断需重新复核。

## 1. 设计目标与边界

Compiler MVP 必须把一个 `.apls` Entry Source 处理为下列互斥结果之一：

```text
REJECTED + stable diagnostics

或

ACCEPTED + verified Canonical JSON IR
```

不存在“大概正确”、“自动修好后继续”或“部分可信 IR”第三种规范结果。

MVP 包含：

- Parse、Check、Emit IR、Diagnose 四类逻辑能力；
- 本地 Source/Import 图加载；
- Lexer、Parser、Resolver、Type/Unit Checker、Semantic Analyzer、Canonicalizer 和 IR Validator；
- 人类可读与机器可读诊断；
- Verified IR 失败关闭发布。

MVP 不包含：

- LLM 调用、自动修复、自然语言解释或隐式补全；
- 代码生成、目标后端、Graph 生成或测试骨架生成；
- 远程 Import、网络访问、Plugin、用户脚本或动态扩展；
- 增量缓存、Daemon、IDE/LSP 或并行编译保证；
- 未获准的实现语言、Parser 技术、IR 哈希算法或参考后端。

## 2. 核心不变量

```text
Same compiler version
+ Same explicit command and options
+ Same entry-relative source graph bytes
=> Same exit code
+ Same ordered diagnostics
+ Same Canonical IR bytes, when successful
```

- 编译核心不读取时间、随机数、主机名、用户名、Locale 或非声明环境变量来决定语义。
- 实现可内部并行，但并行策略不得改变规范结果。
- 任一 Stage 产生 Error 后，不得进入依赖该 Stage 成功产物的下游 Stage。
- 内部 AST/Model 不是公共契约，不得被 Agent 当作 Verified IR 消费。
- Compiler 私有默认值、修复和扩展不得进入 Verified IR。

## 3. 编译事务状态机

```text
INIT
  -> LOADING
  -> PARSED
  -> BOUND
  -> TYPED
  -> CHECKED
  -> CANONICALIZED
  -> VERIFIED
  -> PUBLISHED

Any non-terminal state -> FAILED
```

| 状态 | 可观测含义 | 允许对外发布 |
|---|---|---|
| `INIT` | 命令和参数已解析 | 无 |
| `LOADING` | Source Graph 正在加载 | 无 |
| `PARSED` | 全部已发现 Source 获得完整 Parse Candidate Forest | Parse 成功状态，不发布候选 AST |
| `BOUND` | 所有名称唯一绑定 | 无 |
| `TYPED` | 类型和工程单位唯一 | 无 |
| `CHECKED` | 所有语义冲突关闭 | Check 成功状态，不发布模型 |
| `CANONICALIZED` | IR 已在内存中生成 | 无 |
| `VERIFIED` | IR Schema 和跨节点不变量通过 | 可交给 Publisher |
| `PUBLISHED` | 完整 Verified IR 已一次性交付 | Verified IR |
| `FAILED` | 事务失败 | 只有诊断，无 AST/Model/IR |

事务状态只能单向前进。`FAILED` 和 `PUBLISHED` 是终态；不得在同一事务中修改 Source 后恢复编译。

## 4. 组件与唯一职责

| 组件 | 输入 | 唯一输出 | 失败条件 |
|---|---|---|---|
| Command Driver | argv | `CompileRequest` | 参数缺失、冲突或未知 |
| Source Graph Builder | Entry Path | `SourceBundle` | I/O、路径、循环、重复物理文件或竞态 |
| Candidate Lexer | Source Bytes | Tokenization Candidate Lattice | 词法错误或候选无法完整表示 |
| Parser | Tokenization Candidates | Parse Candidate Forest | 无完整 Parse、资源失败或非法恢复 |
| Declaration Indexer | Declaration Candidates | Declaration Index | 重复声明或非法作用域 |
| Resolver | Parse Candidates + Index | Bound Frame Candidates | 未定义、类别错误或无有效绑定 |
| Type/Unit Checker | Bound Model | Typed Model | 类型、推断、转换或维度错误 |
| Canonical Frame Convergence Gate | Typed Frame Candidates | 唯一 Canonical Frame 等价类 | 零有效候选或两个及以上不等价 Frame |
| Semantic Analyzer | Typed Model | Checked Model | 状态、Rule、Safety、Channel 或约束冲突 |
| Canonicalizer | Checked Model | Canonical IR Bytes Candidate | 缺少默认、稳定 ID、排序或溯源 |
| IR Validator | IR Bytes Candidate | `VerifiedArtifact` | Schema、Ref、版本或不变量失效 |
| Diagnostic Engine | Stage Findings | Ordered Diagnostics | 诊断自身不符合契约时转 Internal Failure |
| Artifact Publisher | Verified Artifact | Published Bytes | 文件输出不能保证失败前目标不变，或 stdout 无法完整交付 |

组件之间只传递不可变、已标记 Stage 的数据。下游不得读取上游私有缓存、全局可变状态或容错节点来推断语义。

## 5. Source 与 Import 图契约

### 5.1 Entry 和逻辑路径

- 命令必须提供恰好一个 Entry `.apls` 普通文件。
- Import Root 固定为 Entry 所在目录。
- Source Manifest 使用相对 Import Root 的 `/` 分隔逻辑路径；主机绝对路径不进入 Canonical IR。
- Import 字符串必须是相对路径、以 `.apls` 结尾，不得包含空 Segment、`.`、`..`、`\\`、URI Scheme 或绝对路径前缀。
- 路径比较使用 UTF-8 Byte 序列精确匹配；不做大小写折叠、Unicode 规范化或近似匹配。
- MVP 拒绝 Symbolic Link 和非普通文件，防止主机解析差异创建竞争 Source Truth。

### 5.2 发现与稳定顺序

1. 读取并解析 Entry；
2. 只从成功解析的 AST 提取 Import；
3. 按逻辑路径 UTF-8 Byte 升序发现直接 Import；
4. 对新文件重复相同流程，直至图闭包；
5. 在完成图闭包后检查循环、别名和同一物理文件的多逻辑路径映射。

某文件解析失败时，不猜测其 Import；该事务在已发现图的 Parse Stage 完成后失败。

### 5.3 Source 竞态

- 首次读取时对每个 Source 计算 SHA-256，并使用当次读取的不可变 Byte Buffer 完成全部 Stage。
- Publisher 交付前重新读取并比较已发现 Source 的 SHA-256。
- 任一 Source 改变则整个事务失败；不得局部重试或混合新旧 Source。

## 6. Stage 失败关闭

- 每个 Stage 可收集该 Stage 内互不依赖的多条诊断，但不得跨越失败的 Stage 继续生成派生诊断。
- Parser 错误恢复只用于定位同一文件中的更多 Syntax Error；恢复节点不进入 Resolver。
- Poison/Invalid 节点只存在于失败事务的内部诊断路径，不得序列化为任何公共 Artifact。
- 检查器无法证明名称唯一、类型唯一、Guard 互斥或 Rule 无冲突时，必须发出 Error。
- Internal Failure 不得被降级为 Source Error，也不得产生 Verified IR。

## 7. 已批准公共命令契约

> 本节已由 `HDP-APLS-005` / `DEC-009` 批准为实现与 Conformance 设计输入；仍未建立产品 Baseline。

### 7.1 命令

```text
apls parse <entry.apls>
apls check <entry.apls>
apls emit-ir <entry.apls> --output <path|->
apls diagnose <entry.apls> [--through parse|check|emit] [--format json|human]
apls --version
```

| 命令 | 最高 Stage | 成功输出 | 失败输出 |
|---|---|---|---|
| `parse` | `PARSED` | stdout 空；stderr 可含 Warning | stdout 空；stderr 人类可读诊断 |
| `check` | `CHECKED` | stdout 空；stderr 可含 Warning | stdout 空；stderr 人类可读诊断 |
| `emit-ir` | `PUBLISHED` | Verified IR 交付到 `--output` | 文件目标不变；stdout 传输失败的前缀无效；stderr 人类可读诊断 |
| `diagnose` | 默认 `CHECKED` | stdout 诊断 Envelope | stdout 诊断 Envelope |
| `--version` | 无 | stdout 唯一一行版本 | stderr Tool Diagnostic |

- `parse` 不对外输出 Surface AST，防止内部结构演变或无效 AST 成为第二机器契约。
- `diagnose --through emit` 执行到 `VERIFIED`，但不调用 Publisher；它不交付 IR。
- `diagnose` 省略 `--through` 时固定为 `check`，省略 `--format` 时固定为 `json`。
- `--output -` 只在 IR 已完整验证并完成 Source 竞态复核后开始写入 stdout。只有 Exit `0` 且 stdout 是完整、可重新验证的 Canonical IR 时才构成有效交付；stdout 传输失败返回 `APLS-T0005` / Exit `2`，已暴露的前缀无效且不得消费。
- `--output` 非 `-` 路径相对当前工作目录解析；目标不得是 Source、Symbolic Link 或非普通文件。成功时可完整替换已有普通文件，失败时保持原文件不变。
- `apls --version` 固定输出 `apls <compiler-version> language=0.1 ir=0.1`，并以单个 LF 结尾。
- MVP 不读取项目配置文件或环境变量改变命令语义。

### 7.2 退出码

| Exit Code | 含义 |
|---:|---|
| `0` | 命令成功；可包含 Warning |
| `1` | Source 被 `APLS-E...` 诊断拒绝 |
| `2` | 命令、I/O、路径、资源或发布等 `APLS-T...` Tool Failure |
| `3` | Compiler Internal Failure |

退出码只由最高严重类别决定：`Internal > Tool > Source > Success`。Warning 不改变退出码。

## 8. 机器诊断 Envelope

`diagnose --format json` 唯一输出：

```json
{
  "diagnostics": [],
  "format": "apls-diagnostics",
  "format_version": "0.1",
  "status": "accepted"
}
```

- 顶层字段恰好为 `diagnostics/format/format_version/status`，未知字段不允许。
- `status` 只能是 `accepted/rejected/tool_failure/internal_failure`。
- 每条诊断必须包含已批准诊断契约中的全部字段。
- Source Span 的路径使用 Source Manifest 逻辑路径；Line/Column 从 `1` 开始，Column 按 Unicode Scalar Value 计数，Tab 计一个 Column。
- JSON 消息使用 Compiler Version 固定的英文模板；根因身份由稳定 Code 决定，不依赖消息文本。
- MVP 的 Human 诊断同样固定使用英文，不从主机 Locale 自动选择语言；本地化不在 0.1 MVP 范围内。
- `diagnostics` 使用唯一公共 Byte-based 总排序键：`(primary_span_group,logical_path_or_empty,start_byte_or_zero,end_byte_or_zero,code,related_source_spans_canonical_bytes,full_diagnostic_canonical_bytes)`；有 Span Group 为 `0`，无 Span Group 为 `1`。精确定义以 `DES-APLS-CNL-DIAG-001` 第 1.2 节为准。本候选修正替代此前 `(logical_path,start_line,start_column,code,related_spans)` 简写，需随 `HDP-APLS-018` 采用。
- `--format human` 可改变展示排版，但不得改变 Code、Severity、Span、集合或退出码。

## 9. 已批准 Tool Diagnostic

`APLS-T...` 表示 Source 语义之外的 Compiler 运行失败，不得与 `APLS-E...` 混用：

| Code | 含义 |
|---|---|
| `APLS-T0001` | 未知、缺失或冲突的命令参数 |
| `APLS-T0002` | Entry/Import 不存在、不可读或不是普通文件 |
| `APLS-T0003` | 路径越界、Symbolic Link、多逻辑路径或主机解析不唯一 |
| `APLS-T0004` | Source 在编译事务期间改变 |
| `APLS-T0005` | 无法在保留原目标的条件下交付 Artifact |
| `APLS-T0006` | Compiler Internal Failure |
| `APLS-T0007` | 资源上限被触发 |

Tool Diagnostic 也使用统一 Diagnostic 字段；无对应 Source Span 时，`primary_source_span` 为 `null`。该公共扩展已随 `HDP-APLS-005` / `DEC-009` 获准为实现与 Conformance 设计输入，仍未建立产品 Baseline。

## 10. Verified IR 发布

- Canonicalizer 先在事务内存中生成完整 Byte Sequence；Publisher 不边生成边暴露。
- IR Validator 必须从实际待发布 Byte Sequence 重新解析并验证，不只验证上游内存 Object。
- 输出到文件时，Publisher 必须保证：验证前不改变目标，失败时保留原目标，成功时只交付完整新 Artifact。
- 当主机文件系统不能对指定目标提供上述保证时，使用 `APLS-T0005` 失败，不降级为非原子覆盖。
- 输出到 stdout 时，必须在第一个 Byte 写出前完成全部验证和 Source 竞态复核。
- stdout/Pipe 在部分 Byte 已被外部观测后可能返回 I/O 失败；Compiler 无法撤回该前缀。调用者必须等待进程结束并确认 Exit `0` 后才能消费 stdout；此规则由 `HDP-APLS-007 Option A` / `DEC-011` 批准。
- Verified IR 的已批准 MIME/Artifact 身份为 `application/vnd.apls.ir+json;version=0.1`；该值已随 `HDP-APLS-005` / `DEC-009` 获准为设计输入，仍未建立产品 Baseline。

## 11. 安全与 AI 边界

- Compiler Core 不执行 Source 中的字符串、Operation、Rule、表达式或自然语言。
- Compiler Core 不访问网络，不加载项目提供的动态库，不启动子进程执行用户代码。
- AI 可在 Compiler 外部解释诊断或提出 Patch，但 Patch 只能作为新 Source Revision 重新发起完整事务。
- Compiler 不接受“允许猜测”、“最佳努力”或“忽略未知字段”类选项。

## 12. 资源、缓存与并发

- Compiler Version 必须定义固定的最大单文件 Byte、Source 数、Import 深度、AST 节点数和诊断数；达到上限使用 `APLS-T0007` 失败。
- 上限必须由 `apls --version` 对应的 Compiler Version 固定，不从未声明环境自适应。
- MVP 不使用跨进程或跨事务缓存；避免缓存身份和失效规则尚未冻结时引入第二 Source Truth。
- 实现可并行处理互不依赖的 Source 或声明，但诊断、Symbol Table、IR 和任何 Hash 的结果必须在单线程语义模型下等价。

## 13. 可测试接缝

| 接缝 | 必须可观测的结果 |
|---|---|
| Lexer/Parser | 同一 Byte 输入的候选集合结构等价、完整，Grammar 工具冲突为零 |
| Frame Convergence | 中间多候选可收敛；两个及以上不等价 Canonical Frame 以 `AMBIGUOUS` 失败 |
| Source Graph | 逻辑路径、加载顺序和 Source SHA-256 稳定 |
| Resolver | 声明顺序变化不改变绑定结果 |
| Type/Unit | 所有 Expression 唯一定型，非法转换失败 |
| Semantic | Transition/Rule/Safety/Channel 冲突均失败关闭 |
| Canonicalizer | 语义等价 Source 的 IR 字节或规范化结果相等 |
| Diagnostic Engine | Code、Severity、Span、排序和去重稳定 |
| Publisher | 文件输出失败时目标不变；stdout 非零退出前缀无效；成功时为完整 Artifact |
| Command Driver | 命令、stdout/stderr、Envelope 和 Exit Code 一致 |

具体 Test Case、Fixture 数量、执行层级和验收证据由后续 C05 Conformance 测试设计决定；本文不冒充测试已完成。

## 14. 需求追溯

| 需求 | Compiler MVP 设计响应 |
|---|---|
| PRD-001 | Parser 保留全部合法候选，Convergence Gate 只接受唯一 Canonical Frame 等价类 |
| PRD-002 | Stage 失败关闭，无 Guess/Repair/Best Effort 选项 |
| PRD-003 | Canonicalizer + Byte Reparse IR Validator + 完整发布 |
| PRD-004 | Indexer/Resolver/Type/Unit 独立 Stage |
| PRD-005 | Semantic Analyzer 检查状态、Rule、Safety、Channel 和约束 |
| PRD-006 | 统一 Diagnostic Engine、机器 Envelope 和稳定排序 |
| PRD-007 | 只有 `VerifiedArtifact` 可进入 Publisher，AST/Model 不对 Agent 公开 |
| PRD-008 | `parse/check/emit-ir/diagnose` 四类已批准公共能力 |

## 15. 未冻结事项

- Compiler 实现语言与 Parser 技术已由 `DEC-010` 决定为 Rust + LALRPOP 默认 LR(1)，不再属于未冻结事项；
- 首个参考后端（`Q-003`）；
- Canonical IR 语义哈希算法（`Q-007`）；
- Compiler Version 的具体资源上限已由 `DEC-011` 在 `DES-APLS-COMPILER-FOUNDATION-001` 中固定，不再属于未冻结事项；

CLI、Tool Diagnostic、Diagnostic Envelope 和 MIME 已由 `DEC-009` 批准为设计输入，不再属于未冻结事项。stdout 在部分写入后失败时的交付契约已由 `HDP-APLS-007 Option A` / `DEC-011` 精炼。

本设计输入不授权代码实现、依赖安装、Commit、Baseline Adoption、Formal C04、Push、Release 或 Formal Seal。
