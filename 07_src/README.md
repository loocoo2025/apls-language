# APLS 源代码

> 方向状态：`PRE_DEC_014_LEGACY_DSL_PROTOTYPE`。`DEC-014` 已将受控自然语言设为唯一公开 Surface Syntax；本目录当前输出只是可复用核心与迁移参考，不是 CNL Compiler 符合性实现。新 Frontend 设计获批前不重写代码。

`TASK-007` 在本目录建立 Rust Compiler 骨架，`TASK-009` 加入确定性 Lexer，`TASK-010` 将完整正式 Grammar 接入唯一 LALRPOP Parser，`TASK-011` 加入 Source Graph 与 Import 加载核心，`TASK-013` 加入 Declaration Index 与唯一名称绑定：

- `crates/apls-compiler`：Compiler Core、互斥 Stage Artifact 和稳定诊断边界；
- `crates/apls-cli`：已批准命令表面、stdout/stderr 和进程退出边界。

Lexer 严格执行 `DES-APLS-LEX-001`：UTF-8 无 BOM、固定 ASCII 空白、最长匹配、ASCII 标识符、严格数字与 JSON Unicode Scalar 转义、非嵌套块注释、零起点半开 Byte Span 和稳定 `APLS-E1001/E1002/E1003/APLS-T0007` Finding。

Parser 仅消费上述手写 Lexer Token。`src/apls_grammar.lalrpop` 是 `APLS_0.1_GRAMMAR.ebnf` 的唯一完整可执行转换；默认 Lane-Table LR(1) 生成冲突或错误会使构建失败，生成 Rust 只位于 Cargo `OUT_DIR`。Parser 不使用内置 Lexer、Error Recovery、隐式顺序消歧或第二语法路径；成功时构造 Crate 内私有的有类型 Surface AST，失败时稳定归一为 `APLS-E1101/APLS-E1103`。

Source Graph Core 把 Import 字符串解释为相对 Entry Import Root 的精确 `/` 分隔逻辑路径，按 Entry 优先、其他路径 UTF-8 Byte 升序发现图闭包。Core 只使用注入式 Reader，保存不可变 Source Byte 和 SHA-256；主机适配器必须拒绝 Symbolic Link/非普通文件并提供物理身份。图构建使用同一完整 Parser 提取 Import，检测 `APLS-E2002/E2004`、`APLS-T0002/T0003/T0004/T0007`，并执行已批准的 Source Byte、Graph Byte、Source 数、Import 深度和全图 Token 上限。

Declaration Index 在绑定前检查 Spec、顶层声明、Import Alias、State/Transition、Enum Variant、Record Field、Operation Parameter 和 Acceptance Given 的已批准命名空间。Resolver 只接受当前 Spec、当前 Acceptance Given、显式 Import Alias 以及 State/Enum 限定形式；零候选、重复、多候选、类别不符和 normative `open/unknown` 依赖分别以 `APLS-E2001/E2002/E2003/E2005/E4004` 失败关闭。任一名称错误都不会构造部分 Declaration Index 或 `BoundProgram`。`set` 继续是保留关键字，但不是 APLS 0.1 合法 Action。

当前仍不是可发布 Compiler。除 `--version` 外，已声明但未实现的命令统一以 `APLS-T0006` / Exit `3` 失败关闭，不产生对外 Artifact。真实主机文件系统适配、CLI 接线、Types、Semantic、Canonical IR、Validator 和 Publisher 仍由后续工作包实现。

最小验证命令：

```text
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
```
