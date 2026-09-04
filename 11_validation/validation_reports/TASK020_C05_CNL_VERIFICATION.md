# TASK-020 C05 CNL 实现验证报告

- Report ID：`VAL-APLS-C05-020`
- Task：`TASK-020 / CNL_IMPLEMENTATION_VERIFICATION`
- 日期：`2026-09-04`
- 角色：`C05`
- 授权：`HDP-APLS-020 Option A / DEC-025`
- 结论：`PASS / C05_VERIFICATION_COMPLETE`

## 1. 验证范围

本轮只执行已批准的最小风险驱动范围：

- `CNL-C001～C016` 与现有测试的追溯和普通锁定回归；
- Rust 1.98 参考工具链与 Rust 1.86 MSRV 锁定离线检查；
- 公共候选资源边界 ignored 黑盒用例；
- Unicode 17.0.0 官方 NFC 全量一致性 ignored 用例；
- Cargo/Lock 及受保护设计、源码和测试的前后完整性检查。

未执行性能、Fuzz、跨平台、长稳或完整发布测试；未修改 Compiler/CLI Source、现有测试、设计、Schema、Cargo Manifest 或 Lock。

## 2. 普通 T1 结果

| 验证 | 结果 |
|---|---|
| `cargo fmt --all --check` | PASS |
| Rust `1.98.0`：`cargo check --workspace --locked --offline` | PASS |
| Rust `1.86.0`：`cargo check --workspace --locked --offline` | PASS |
| `cargo test -p apls-compiler --locked --offline` | PASS：24 passed，1 ignored |
| `cargo test -p apls-cli --locked --offline` | PASS：7 passed，1 ignored；main/doc tests 无失败 |
| `git diff --check` | PASS |

工具链身份：

- `rustc 1.98.0 (88d9e12ae 2026-08-18)`；
- `rustc 1.86.0 (05f9846f8 2025-03-31)`。

普通测试覆盖的 `CNL-C001～C016` 精确映射见 `06_test_design/APLS_0.1_CNL_IMPLEMENTATION_VERIFICATION_PLAN.md` 第 3 节。

## 3. T0 公共资源边界

执行：

```text
cargo +1.98.0 test -p apls-cli --locked --offline \
  tests::public_candidate_resource_boundaries_use_the_normal_cli_pipeline \
  -- --ignored --exact
```

结果：`PASS — 1 passed，7 filtered out`，用时约 `4.06s`。

该用例通过正常 CLI Pipeline 验证 `complete_token_streams`、`candidate_token_occurrences` 和 `candidate_lattice_edges` 的边界/首次超限行为；本轮没有用测试参数绕过公开先行资源上限。

## 4. T0 Unicode 17.0.0 官方一致性

数据：

- 来源：`https://www.unicode.org/Public/17.0.0/ucd/NormalizationTest.txt`；
- 本地临时路径：`/private/tmp/NormalizationTest-17.0.0.txt`；
- 文件头：`NormalizationTest-17.0.0.txt`；
- 文件日期：`2025-06-30 06:16:16 GMT`；
- 大小：`2,827,429` Byte；行数：`20,095`；
- SHA-256：`5019ffd530751a741900c849c0e010332f142a3612234639bd200b82138a87db`。

执行：

```text
APLS_UNICODE_NORMALIZATION_TEST=/private/tmp/NormalizationTest-17.0.0.txt \
cargo +1.98.0 test -p apls-compiler --locked --offline \
  cnl_lexer::tests::unicode_17_official_normalization_conformance \
  -- --ignored --exact
```

结果：`PASS — 1 passed，24 filtered out`，用时约 `0.11s`。测试实际遍历官方数据且其内部完整性门槛要求有效 Case 数超过 10,000。

## 5. 完整性与禁止变更核验

对 `04_design/`、`07_src/`、`08_tests/` 中排除 `07_src/target/` 后的 74 个普通文件，执行前后按每文件 SHA-256 记录流计算聚合摘要：

```text
c52271e9e5f25eed993ab00d8af17d27fd3f6258ed151f5a35b1552aa29fb1f8
```

开始/结束文件集合和记录流逐 Byte 相同。

- `07_src/Cargo.toml`：`82cd5057ac88e3ff1f01d8535bdc045bc34894335e1390b104e8152ed94954d2`；
- `07_src/Cargo.lock`：`0375790b09dbfb630a5bc855959cc1542ba835e393454a43079d5328ba6fe3c1`。

两者前后均未变化；未安装或升级依赖。测试只在已排除的 `07_src/target/` 产生构建输出，官方数据只写入 `/private/tmp`。

## 6. 结论与剩余风险

全部批准的 T0/T1 验证通过，`CNL-C001～C016` 的当前最小实施验收证据完整；未发现需要返回 C03/C06 的失败或实现偏差。TASK-020 可由 C00 收口为 `DONE`。

本报告不是正式 C04、Baseline Adoption、Release 或 Formal Seal。项目仍缺少不可变 Git Commit Anchor、正式 C04 和项目负责人 Baseline Adoption；这些动作均需后续独立授权。
