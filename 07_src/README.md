# APLS 源代码

> 当前状态：本目录是 `DEC-023`（`HDP-APLS-018 Option A`）批准契约的 CNL Compiler 垂直切片实现（`TASK-018`），实现整改按 `DEC-024` 完成，已通过 `TASK-020` C05 验证（`DEC-025`），并冻结为候选 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e`（`DEC-026`）。首次正式 C04（`FORMAL_C04_APLS_0_1_CANDIDATE_001`）结论为 `CHANGES_REQUESTED`，整改按 `WP-APLS-C04-REMEDIATION-001` / `DEC-031` 执行中。尚未建立产品 Baseline。

- `crates/apls-compiler`：CNL Compiler Core——候选词法 Lattice、逐候选 Parse/绑定/类型/规范化、Canonical Frame 收敛 Gate、Canonical IR 构建与发布前复验、稳定诊断与固定资源边界；
- `crates/apls-cli`：已批准命令表面（`apls parse` / `apls check` / `apls emit-ir` / `apls diagnose` / `apls --version`）、stdout/stderr 边界与退出码 0/1/2/3。

当前实现要点：

- `cnl_lexer.rs` 对每个 Byte 位置生成全部词法候选 Edge，完整枚举全部 Complete Token Stream；不做最长匹配、概率、LLM 或候选排名消歧；
- `apls_grammar.lalrpop` 是当前 CNL Grammar（`APLS_0.1_ZH_CN_GRAMMAR.ebnf`）的可执行形式；LALRPOP 默认 LR(1)，生成冲突或错误使构建失败（零冲突门禁，`DEC-010`）；
- `cnl_pipeline.rs` 按 bind→type→normalize 阶段确定性处理每个候选；收敛 Gate 按 Canonical Frame Byte 去重判定 0/1/>1：零候选按封闭根因聚合失败关闭，两个及以上不等价等价类输出 `APLS-E1310` 与分歧见证；
- IR 发布前对实际输出 Byte 重新解析、JSON Schema 复验与跨节点复验；仅 Exit 0 且完整可验证时构成有效 IR 交付（原子发布、发布前 Source 复读、符号链接/路径穿越拒绝）；
- 资源上限按 `DES-APLS-CNL-RESOURCE-001` 固定；首次超限返回 `APLS-T0007` / Exit 2，不产生部分 Artifact。

## Legacy DSL 迁移参考（非现行代码）

`crates/apls-compiler/src/` 下旧 DSL 文件 `ast.rs`、`lexer.rs`、`parser.rs`、`resolve.rs`、`source.rs`、`index.rs`、`canonical.rs`、`ir.rs`、`semantic.rs`、`types.rs` **未挂入模块树、不参与编译**，仅作为 `DEC-014` 方向变更前旧花括号 DSL 的迁移参考保留（`ARCH-APLS-CNL-001` §8）；旧 `apls-ir-0.1` 仅属 Legacy DSL（`DEC-019`）。不得将这些文件误认为现行实现。

最小验证命令：

```text
cargo fmt --all -- --check
cargo check --workspace --locked
cargo test --workspace --locked
```
