# APLS 未决问题

| ID | 优先级 | 问题 | 影响 | 责任角色 | 状态 |
|---|---|---|---|---|---|
| Q-001 | P1 | 历史问题：APLS 0.1 表面语法采用 DSL、YAML 还是双入口？ | 旧公共语言接口 | HUMAN_PROJECT_OWNER | SUPERSEDED — DEC-014 |
| Q-002 | P1 | 编译器 MVP 采用哪一种实现语言与 Parser 技术组合？ | 工具链实现、Grammar 零冲突门槛与部署 | HUMAN_PROJECT_OWNER | CLOSED — OPTION A |
| Q-003 | P2 | 第一版是否绑定 FreeRTOS 水肥控制作为参考后端？ | MVP 边界 | C01 | CLOSED — 不绑定（DEC-037） |
| Q-004 | P1 | 是否批准 `HDP-APLS-001` 所绑定的 APLS 0.1 PRD 候选作为下一阶段输入？ | 是否允许进入语言与 IR 架构设计 | HUMAN_PROJECT_OWNER | CLOSED — APPROVED |
| Q-005 | P1 | 是否批准 `ARCH-APLS-001` 作为语言规范、IR Schema 与编译器详细设计的架构输入？ | 是否允许进入详细设计 | HUMAN_PROJECT_OWNER | CLOSED — APPROVED |
| Q-006 | P1 | 是否批准 APLS 0.1 Grammar、Canonical IR Schema 和诊断目录候选作为 Compiler MVP 设计输入？ | 是否允许进入编译器 MVP 详细设计 | HUMAN_PROJECT_OWNER | CLOSED — APPROVED |
| Q-007 | P2 | Canonical IR 0.1 语义哈希采用哪一种算法及版本标识？ | IR 缓存、等价性与兼容性 | C02 | CLOSED — HDP-APLS-015 OPTION A / DEC-019 |
| Q-008 | P1 | 是否批准 Compiler MVP 详细设计候选及 CLI/Tool Diagnostic/Envelope 公共契约？ | 是否冻结 MVP 工具边界并进入实现技术决策 | HUMAN_PROJECT_OWNER | CLOSED — APPROVED |
| Q-009 | P1 | 是否批准 `HDP-APLS-007` 所绑定的精确工具链、依赖闭包、资源上限、Canonical IR 尾 Byte 与 stdout 契约修正？ | 是否形成可重现的 Compiler 骨架实现输入 | HUMAN_PROJECT_OWNER | CLOSED — OPTION A |
| Q-010 | P1 | 是否批准 `HDP-APLS-008` 所绑定的精确 ASCII 词法 Profile？ | 旧 DSL Lexer | HUMAN_PROJECT_OWNER | SUPERSEDED_FOR_PUBLIC_SURFACE — DEC-014 |
| Q-011 | P1 | 是否批准 `HDP-APLS-009` 的名称解析、符号类别矩阵及 `set` 闭合方案？ | 符号类别可复用；旧 Source 形式失效 | HUMAN_PROJECT_OWNER | PARTIALLY_SUPERSEDED — DEC-014 |
| Q-012 | P1 | APLS 0.1 首个规范性自然语言 Profile 采用简体中文单语、中英双语还是首版多语？ | CNL 词法、Grammar、诊断、Conformance 和用户范围 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-011 OPTION A / DEC-015 |
| Q-013 | P1 | 是否批准 `apls-zh-CN-0.1` Profile、Grammar、Semantic Frame 和 CNL 诊断候选作为 Frontend 后续设计输入？ | 首版公开语言接受集与拒绝边界 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-012 OPTION A / DEC-016 |
| Q-014 | P1 | 同一句存在多个词法切分/Parse、但最终可归一为同一个 Frame 时，是立即拒绝还是允许接受？ | Source 接受集、PRD-001、Parser 架构、诊断和性能边界 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-013 OPTION B / DEC-017 |
| Q-015 | P1 | 是否批准保留 Rust/LALRPOP、增加候选分析与 Canonical Frame 收敛层，并在代码迁移前先修订 CNL IR Schema？ | Frontend 组件边界、IR Schema 与下一 C02/C03 工作包顺序 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-014 OPTION A / DEC-018 |
| Q-016 | P1 | 是否批准 CNL Canonical IR 新 Schema、稳定 Semantic ID、匿名节点 SHA-256 身份、名义自定义单位和等价重复节点合并规则？ | 公共 IR Schema、语义身份、单位语义和 C03 实现输入 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-015 OPTION A / DEC-019 |
| Q-017 | P1 | 是否批准 `TASK-018` 完整 CNL Frontend/Frame/IR 实现范围及候选分析资源上限？ | 是否允许修改 Compiler Source/测试并固定无法完成唯一性证明时的 Tool Failure 边界 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-016 OPTION A / DEC-020 |
| Q-018 | P1 | 是否批准先关闭实施前独立复审的九项剩余 Blocking Finding，再恢复 TASK-018？ | 语言接受集、类型/单位、Transition、资源、NFC、Canonicalization、Provenance 和诊断契约完整性 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-017 OPTION A / DEC-021 |
| Q-019 | P1 | 是否批准 TASK-019 输出中的公共语义、CNL IR Schema、Unicode/NFC 依赖和诊断机器契约变更，并恢复 TASK-018？ | 当前 CNL 设计输入、依赖闭包和实施授权是否更新 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-018 OPTION A / DEC-023 |
| Q-020 | P1 | 是否接受 `tinyvec 1.6.0` 实际激活的空 `default` Feature，并仅针对 IO-01～IO-07 重开 TASK-018 C03 Retry？ | 依赖批准证据真实性、TASK-018 实现闭包和后续独立再复审 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-019 OPTION A / DEC-024 |
| Q-021 | P1 | 是否批准 TASK-020 按最小风险驱动范围执行 C05 验证，并获取 Unicode 17.0.0 官方一致性数据？ | 是否形成实现候选的验证证据并为后续正式 C04/Baseline 准备输入 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-020 OPTION A / DEC-025 |
| Q-022 | P1 | 是否授权为当前 C05 PASS 候选创建本地初始 Commit Anchor？ | 是否形成可用于后续正式 C04 的不可变 Git Target | HUMAN_PROJECT_OWNER | CLOSED — HUMAN AUTHORIZATION / HDP-APLS-021 OPTION A / DEC-026 |
| Q-023 | P1 | 是否批准以 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e` 为精确 Target 发起首次正式 C04？ | 是否形成产品 Baseline Adoption 前必需的正式独立 Gate 结论 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-022 APPROVED OPTION A / DEC-028 |
| Q-024 | P1 | 是否批准 `WP-APLS-C04-REMEDIATION-001` 整改工作包，并裁决 Q1（F-02 冲突语义）、Q2（F-06 unknown/open）、Q3（F-04 根因聚合与七码）、Q4/Q4-a（F-08 追溯层范围）？ | 9 项 Open Finding 的整改是否可进入实施 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-024 APPROVED（Q1=A/Q2=B/Q3=A/Q4=A/Q4-a=NO）/ DEC-031 |
| Q-025 | P1 | 是否将精确 Commit `fd8b59536fcdfdff2f3b199b882c15d97edb1993` 采用为 APLS 0.1 产品 Baseline（是否同时授权治理回执提交与私有远程 Push）？ | 产品 Baseline 成立与远程同步 | HUMAN_PROJECT_OWNER | CLOSED — HDP-APLS-025 APPROVED（Q1=A/Q2=A）/ DEC-034 |

`Q-015` 已由 `HDP-APLS-014 Option A` 关闭；`Q-007/Q-016` 已由 `HDP-APLS-015 Option A` / `DEC-019` 关闭；`Q-017` 已由 `HDP-APLS-016 Option A` / `DEC-020` 关闭；`Q-018` 已由 `HDP-APLS-017 Option A` / `DEC-021` 关闭；`Q-019` 已由 `HDP-APLS-018 Option A` / `DEC-023` 关闭；`Q-020` 已由 `HDP-APLS-019 Option A` / `DEC-024` 关闭；`Q-021` 已由 `HDP-APLS-020 Option A` / `DEC-025` 关闭；`Q-022` 已由项目负责人明确授权及 `DEC-026` 关闭；`Q-023` 已由 `HDP-APLS-022 APPROVED Option A` / `DEC-028` 关闭；`Q-024` 已由 `HDP-APLS-024 APPROVED` / `DEC-031` 关闭；`Q-025` 已由 `HDP-APLS-025 APPROVED` / `DEC-034` 关闭；`Q-003` 已由项目负责人裁决“不绑定” / `DEC-037` 关闭。当前无未决问题。
