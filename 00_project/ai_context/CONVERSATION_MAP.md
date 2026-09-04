# APLS 对话与角色拓扑

| ID | 角色 | 版本 | 状态 | 职责 | 主要写入范围 |
|---|---|---|---|---|---|
| C00 | Control | v02 | READY | Kimi 接管后执行 Baseline Relearn，再处理 HDP-APLS-022 | `00_project/ai_context/` |
| C01 | Requirements | v01 | READY | 需求事实 Owner；当前无活动工作包 | `01_product_requirements/` |
| C02 | Architecture/Design | v01 | READY | TASK-019 已完成；当前无活动工作包 | `03_architecture/`、`04_design/` |
| C03 | Implementation | v01 | READY | TASK-018 已完成；当前无活动工作包 | `07_src/` |
| C04 | Independent Review | v01 | READY | TASK-022 等待 HDP-APLS-022；触发后必须创建全新独立 Session | 只读 Target + 预定义 Review Record |
| C05 | Verification/Release | v01 | READY | TASK-020 已完成；等待后续 Gate | `06_test_design/`、`09_quality/`、`11_validation/`、`14_release/` |
| C06 | Issues/Change | v01 | READY | 后续问题与变更闭环 | 待建立 |

C00 是项目负责人的持续逻辑控制通道。C00-v01 已通过 `HANDOFFS/C00-v01-to-v02-KIMI-2026-09-04.md` 冻结为历史交接来源；C00-v02 待项目负责人在 Kimi 中手动启动。正式 C04 必须使用与 C00-v02 不同的全新独立 Session。
