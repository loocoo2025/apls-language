# TASK-018 C03 实施验证记录

- 任务：`TASK-018 / CNL_FRONTEND_FRAME_IR_IMPLEMENTATION / RETRY2`
- 日期：`2026-09-04`
- 角色：`C03`
- 结论：`PASS / RETRY2_OUTPUT_READY_CANDIDATE`
- 范围：`NB-01 / IO-02 PARTIAL` 的 Production 级 spacing 直接闭包；`IO-01、IO-03～IO-07` 继承独立再复审 `CLOSED`

## 1. Retry 2 实施结果

- `NB-01 / IO-02`：`RawLiteralKind::Percentage/Temperature/Duration` 按实际专用 Production 执行 `opt-space`；`RawLiteralKind::Quantity` 一律按 `number-with-declared-unit` 执行 `ws1`，不再根据解析后 Unit ID 改写 Grammar 约束。
- `20  %` 保留合法 Quantity Parse Candidate：`parse` 和 `diagnose --through parse` 成功；该候选不符合当前 Percentage 专用 Source 类型矩阵，因此 `check` 和 `emit-ir` 在 Type Stage 以 `APLS-E1401` 稳定拒绝，而非 Parse Stage `APLS-E1101`。
- Production 级单元探针对同一 `20  %` Source 证明：专用 Percentage Candidate 违反 `opt-space`，Quantity Candidate 满足 `ws1`。
- `IO-01、IO-03～IO-07` 本轮未修改，继承 `IIR-APLS-TASK018-IMPLEMENTATION-002` 的独立 `CLOSED` 结论。

## 2. Retry 2 最小验证

| 验证 | 环境 | 结果 |
|---|---|---|
| `cargo fmt --all --check` | Rust 1.98 rustfmt | PASS |
| `cargo check --workspace --locked --offline` | `rustc 1.98.0` | PASS |
| `cargo check --workspace --locked --offline` | `rustc 1.86.0` | PASS |
| `cargo test -p apls-compiler --locked --offline` | `rustc 1.98.0` | PASS：24 passed，1 ignored |
| `cargo test -p apls-cli --locked --offline` | `rustc 1.98.0` | PASS：7 passed，1 ignored |
| `git diff --check` | 当前工作区 | PASS |

本轮只增加 `literal_spacing_uses_the_actual_grammar_production` 和改写既有公开命令 spacing 用例；实施后先分别运行两个定向用例，再运行普通 Compiler/CLI 回归。未重跑百万级资源用例和 Unicode 20,034 行官方用例；两项保留之前已通过证据，且 NB-01 不要求重复昂贵边界。

## 3. 未变更的 Unicode 与依赖边界

- `unicode-normalization 0.1.25`：`default-features=false`；Registry Checksum `5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8`。
- `tinyvec 1.6.0`：实际激活 `alloc + default(empty) + tinyvec_macros`；`default=[]`，`alloc=["tinyvec_macros"]`；Checksum `87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50`。
- `tinyvec_macros 0.1.1`：无 Feature/运行依赖；Checksum `1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20`。
- `unicode-normalization/std` 和 `tinyvec/std` 均未启用；License 与 MSRV 证据不变，项目 `rustc 1.86.0` 离线构建通过。
- 本轮未修改 `Cargo.toml` / `Cargo.lock`，未增加、升级或安装依赖；本节继承 Retry 1 已独立关闭证据。

## 4. 完成边界

- 本记录是 C03 Retry 2 实施与 Self Review 证据，不是独立 C04/C05 结论。
- 未 Commit、Push、Baseline Adoption、Formal C04、C05、Release 或 Formal Seal。
- 仓库尚无初始 Commit，因此 Git 无可用 HEAD Diff；本轮使用授权文件范围审查、锁定离线构建/测试和 `git diff --check` 完成机械检查。
