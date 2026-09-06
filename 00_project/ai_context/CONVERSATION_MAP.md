# APLS 对话与角色拓扑

| ID | 角色 | 版本 | 状态 | 职责 | 主要写入范围 |
|---|---|---|---|---|---|
| C00 | Control | v02 | ACTIVE | 已获 DEC-028 授权，发起首次正式 C04 Dispatch | `00_project/ai_context/` |
| C01 | Requirements | v01 | READY | 需求事实 Owner；当前无活动工作包 | `01_product_requirements/` |
| C01 | Requirements | v02 | COMPLETED | TASK-024 批次 2 完成（27 ID、30 FORMAL_TRACE 边、门禁 PASS），会话冻结 | `01_product_requirements/`、`02_system_requirements/`、`09_quality/traceability/`、`07_src/README.md`、工作包批次 2 列明的状态字段 |
| C02 | Architecture/Design | v01 | READY | TASK-019 已完成；当前无活动工作包 | `03_architecture/`、`04_design/` |
| C02 | Architecture/Design | v02 | COMPLETED | TASK-024 第一阶段完成：`WP-APLS-C04-REMEDIATION-001` + `HDP-APLS-024` 已交付，待负责人裁决 | `04_design/` 工作包文件 + `00_project/ai_context/HUMAN_DETERMINATION_024.md` |
| C03 | Implementation | v01 | READY | TASK-018 已完成；当前无活动工作包 | `07_src/` |
| C03 | Implementation | v02 | COMPLETED | TASK-024 批次 1 完成（36+7 测试全过，MSRV 通过），会话冻结 | `07_src/` + 工作包批次 1 列明的契约文档 |
| C04 | Independent Review | v01 | INTERRUPTED | TASK-022：`NISR-APLS-C04-022-001` 已启动并完成 Target 物化，但在写入 Review Record 前被平台中止；无 Gate Decision，重新 Dispatch 需新授权 | 只读 Target + 仅 `05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md` |
| C04 | Independent Review | v02 | COMPLETED | TASK-022 已完成：`FORMAL_C04_APLS_0_1_CANDIDATE_001 = CHANGES_REQUESTED`（9 Open Findings）；本会话评审结束后冻结 | 只读 Target + 仅 `05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md` |
| C05 | Verification/Release | v01 | READY | TASK-020 已完成；等待后续 Gate | `06_test_design/`、`09_quality/`、`11_validation/`、`14_release/` |
| C06 | Issues/Change | v01 | READY | 后续问题与变更闭环 | 待建立 |

C00 是项目负责人的持续逻辑控制通道。C00-v01 已通过 `HANDOFFS/C00-v01-to-v02-KIMI-2026-09-04.md` 冻结为历史交接来源；C00-v02 为当前活动控制通道。C04-v01 于 2026-09-05 按 `DEC-028` 以全新独立 Session（`NISR-APLS-C04-022-001`，当前 Kimi 环境本地创建）启动首次正式 C04，与本 C00-v02 会话上下文完全隔离。
