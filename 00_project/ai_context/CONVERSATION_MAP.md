# APLS 对话与角色拓扑

| ID | 角色 | 版本 | 状态 | 职责 | 主要写入范围 |
|---|---|---|---|---|---|
| C00 | Control | v01 | ACTIVE | 持续逻辑总控、Current Truth、授权边界 | `00_project/ai_context/` |
| C01 | Requirements | v01 | READY | 需求事实 Owner；当前无活动工作包 | `01_product_requirements/` |
| C02 | Architecture/Design | v01 | READY | `TASK-019` 实施前契约整改包等待 `HDP-APLS-017` | `03_architecture/`、`04_design/language/`、`04_design/ir/` |
| C03 | Implementation | v01 | BLOCKED | `TASK-018` 已授权但被实施前独立复审 Finding 阻断 | `07_src/` |
| C04 | Independent Review | v01 | READY | 精确 Target 独立评审 | 待触发时建立 |
| C05 | Verification/Release | v01 | READY | 后续一致性验证与发布 | 待建立 |
| C06 | Issues/Change | v01 | READY | 后续问题与变更闭环 | 待建立 |

C00 是项目负责人的持续逻辑控制通道。正式 C04 必须使用新的独立 Session；当前 C01 工作不构成 C04。
