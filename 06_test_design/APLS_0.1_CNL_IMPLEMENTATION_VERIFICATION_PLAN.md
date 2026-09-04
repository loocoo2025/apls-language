# APLS 0.1 CNL 实现最小风险驱动验证计划

- Plan ID：`TESTPLAN-APLS-CNL-C05-020`
- Task：`TASK-020`
- 角色：`C05`
- 状态：`COMPLETED / ALL_T0_T1_PASSED`
- 授权：`HDP-APLS-020 Option A / DEC-025`
- 日期：`2026-09-04`

## 1. 当前必须验证的需求

1. `CNL-C001～C016` 均有当前实现测试证据，不把 C03 Self Review 或非正式独立复审冒充 C05；
2. Rust 1.98 和 MSRV 1.86 的锁定离线构建成功，普通 Compiler/CLI 回归通过；
3. CNL-C012 的三个公开可达资源在正常 CLI Pipeline 上满足“边界可继续、首次超限 T0007”；
4. Unicode 17.0.0 官方 `NormalizationTest.txt` 的 NFC 判定全部通过，数据来源和 SHA-256 可追溯；
5. 验证过程中不修改 Source、测试、设计、Schema、Cargo 或 Lock。

## 2. 当前高风险区域

- T0-R1：公共候选 Lattice/Stream/Token Occurrence 的百万级边界只由 ignored 黑盒用例覆盖；
- T0-R2：NFC 实现依赖 Unicode 17.0.0 数据闭包，必须用官方全量向量验证；
- T1-R1：普通测试需要在当前最终实现状态重新执行，确认 Retry 未破坏 CNL-C001～C016 的继承证据；
- T1-R2：MSRV 与锁文件必须保持不变。

## 3. CNL-C001～C016 追溯

| Case | 当前直接测试证据 |
|---|---|
| CNL-C001 | `all_surface_sentence_and_frame_kinds_compile`、`all_public_commands_enforce_state_list_and_literal_spacing_grammar` |
| CNL-C002/C003 | `property_and_unit_matrix_accepts_and_rejects_without_guessing`、`type_mismatch_is_not_reported_as_ambiguity` |
| CNL-C004 | `duplicate_conditions_and_rules_merge_semantics_and_provenance` |
| CNL-C005/C006 | `transition_reachability_and_conflicts_fail_closed`、`stable_diagnostics_use_deadline_and_transition_contract_spans` |
| CNL-C007 | `all_surface_sentence_and_frame_kinds_compile`、`cross_validator_reexecutes_the_semantic_closure_on_output_bytes` |
| CNL-C008 | `unicode_nfc_family_and_source_map_use_original_byte_boundaries`、`schema_and_cross_node_revalidation_reject_tampering` |
| CNL-C009 | `rejects_non_nfc_source`、`unicode_nfc_family_and_source_map_use_original_byte_boundaries`、ignored `unicode_17_official_normalization_conformance` |
| CNL-C010 | `zero_candidate_paths_report_specific_term_category_and_owner_errors`、`type_mismatch_is_not_reported_as_ambiguity` |
| CNL-C011 | `inequivalent_final_frames_are_ambiguous` |
| CNL-C012A/C | ignored `public_candidate_resource_boundaries_use_the_normal_cli_pipeline` |
| CNL-C012B | `defensive_counters_fail_on_first_item_past_limit`、`syntax_node_overflow_identifies_the_exact_first_excess_node`、`production_pipeline_counts_only_fully_bound_candidates` |
| CNL-C013/C016 | `public_diagnostic_limit_is_applied_after_total_ordering` |
| CNL-C014 | `emits_schema_valid_ir`、`schema_and_cross_node_revalidation_reject_tampering`、`cross_validator_reexecutes_the_semantic_closure_on_output_bytes` |
| CNL-C015 | `declaration_forward_references_are_order_independent` |

完整普通 Compiler/CLI 测试用于执行以上非 ignored 证据；不为每个 Case 重复启动单独命令。

## 4. T0 测试

1. `public_candidate_resource_boundaries_use_the_normal_cli_pipeline --ignored --exact`；
2. 下载 `https://www.unicode.org/Public/17.0.0/ucd/NormalizationTest.txt` 到 `/private/tmp/NormalizationTest-17.0.0.txt`，记录 SHA-256；
3. 设置 `APLS_UNICODE_NORMALIZATION_TEST` 后运行 `unicode_17_official_normalization_conformance --ignored --exact`。

任一失败立即形成 C05 `FAIL/BLOCKED`，不得修改实现或测试。

## 5. T1 测试

1. `cargo fmt --all --check`；
2. Rust 1.98：`cargo check --workspace --locked --offline`；
3. Rust 1.86：`cargo check --workspace --locked --offline`；
4. Rust 1.98：Compiler 与 CLI 普通 `cargo test --locked --offline`；
5. `git diff --check`、Cargo/Lock 摘要前后比对。

## 6. T2 候选测试

- 多次随机候选顺序扰动；
- 更多 Unicode 边界组合；
- 额外大规模资源组合。

本轮不执行：现有 T0/T1 已直接覆盖当前剩余风险，继续扩张收益不足。

## 7. T3 默认不执行项

- Fuzz、性能基准、压力、长稳；
- 跨操作系统和跨架构矩阵；
- 安装包、签名、公网发布和完整 Release 流程。

## 8. 预计时间

- 普通验证：约 1～3 分钟；
- Unicode 官方一致性：通常少于 1 分钟；
- 公共资源边界：可能数分钟，以 60 秒以内轮询保持可见进度，不设置人为放宽上限。

## 9. 环境与数据

- 本地锁定 Cargo/Rust 工具链：`/private/tmp/apls-cargo`、`/private/tmp/apls-rustup`；
- Cargo 使用 `--locked --offline`，不安装或更新依赖；
- 唯一网络动作是从 Unicode 官方 HTTPS 地址获取 17.0.0 验证数据到 `/private/tmp`；
- 仓库尚无 Git Commit，使用文件 SHA-256 和工作区检查证明 Cargo/Lock 与授权禁止文件未被改变。

## 10. 当前缺口与停止条件

执行前缺口是 T0 两项的本轮真实执行结果。停止条件：

- 全部 T0/T1 PASS → 输出 C05 验证报告并回到 C00；
- 任一 FAIL、数据版本/来源不符、Cargo/Lock 变化或需修改受保护文件 → 立即停止并以 `BLOCKED/FAIL` 返回 C00；
- 不因“更加全面”追加测试。

执行结果：全部 T0/T1 已通过，未触发 T2/T3 或过度测试报警。权威结果见 `11_validation/validation_reports/TASK020_C05_CNL_VERIFICATION.md`。
