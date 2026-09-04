# APLS 当前决策注册表

> 同一主题只有一个当前有效决定；本文件是当前有效决定的唯一权威索引。

| ID | 决策主题 | 当前决定 | 状态 | 确认依据 | 日期 | 影响范围 |
|---|---|---|---|---|---|---|
| DEC-001 | 治理框架版本 | 采用 `ai-engineering-governance v0.1.5`，精确 Commit `7197ef3c91c0d97ce2578c121c088ef96a4f4469` | CONFIRMED | 项目负责人明确要求使用该框架 | 2026-09-01 | 全项目治理 |
| DEC-002 | 治理采用方式 | 采用 `Lite`，保障节奏选择 `LEAN` | CONFIRMED | 项目负责人“采用最简治理模式” | 2026-09-02 | 治理文件集与保障频率 |
| DEC-003 | 产品形态 | 设计 APLS 语言规范和类似编译器的工具链 | CONFIRMED | 项目负责人明确目标并授权开始 | 2026-09-01 | 产品与技术方向 |
| DEC-004 | 首要质量目标 | APLS 0.1 必须拒绝歧义，不能把不唯一输入交给 AI 猜测 | CONFIRMED | 项目负责人明确指出这是当前最重要目标 | 2026-09-01 | 语法、语义、IR、诊断、测试 |
| DEC-005 | PRD 候选用途 | 批准 APLS 0.1 PRD 作为语言规范与 Canonical IR 架构设计的受控需求输入；不等于 Baseline Adoption | APPROVED | `HDP-APLS-001` | 2026-09-02 | C02 架构工作包 |
| DEC-006 | 总体架构用途 | 批准 `ARCH-APLS-001` 作为正式语法、Canonical IR Schema、诊断目录和编译器详细设计的受控输入；不等于 Baseline Adoption | APPROVED | `HDP-APLS-002` | 2026-09-02 | C02 详细设计工作包 |
| DEC-007 | APLS 0.1 表面语法 | 历史决定：单一自定义 `.apls` DSL 作为唯一规范性源语言 | SUPERSEDED_BY_DEC-014 | `HDP-APLS-003 Option A` | 2026-09-02 | 仅作历史与迁移参考 |
| DEC-008 | APLS 0.1 详细设计用途 | 批准 Grammar、Canonical IR Schema 和诊断目录作为 Compiler MVP 详细设计的受控输入；不等于 Baseline Adoption | APPROVED | `HDP-APLS-004` | 2026-09-02 | Compiler MVP 设计 |
| DEC-009 | Compiler MVP 设计用途 | 批准 `DES-APLS-COMPILER-001` 及 CLI、Exit Code、Diagnostic Envelope、Tool Diagnostic 和 Verified IR 发布契约作为实现技术决策与 Conformance 设计的受控输入；不等于 Baseline Adoption | APPROVED | `HDP-APLS-005` | 2026-09-02 | 实现技术选型、Conformance 设计 |
| DEC-010 | Compiler 实现语言与 Parser 技术 | APLS 0.1 Compiler MVP 主实现采用 Rust，Parser Generator 采用 LALRPOP 默认 LR(1)；零 Grammar 冲突、禁止隐式顺序消歧 | ACCEPTED | `HDP-APLS-006 Option A`、`ADR-APLS-001` | 2026-09-02 | Compiler 实现设计、构建与部署 |
| DEC-011 | Compiler 实现基础 | 采用 `DES-APLS-COMPILER-FOUNDATION-001` Option A：Rust 2024，参考 Rust 1.98.0，MSRV 1.86.0，精确依赖闭包、互斥 Stage Artifact、零冲突 Parser 门禁、固定资源上限、Canonical IR 无尾 LF；保留 stdout，仅 Exit 0 且完整可验证时构成有效 IR 交付 | APPROVED | `HDP-APLS-007 Option A` | 2026-09-02 | Compiler 骨架、构建、Canonical IR、Publisher 与 Conformance |
| DEC-012 | APLS 0.1 精确词法行为 | 旧 ASCII DSL 词法 Profile；不再定义当前 CNL 公开 Source 接受集 | SUPERSEDED_FOR_PUBLIC_SURFACE_BY_DEC-014 | `HDP-APLS-008 Option A` | 2026-09-02 | 旧 Lexer 参考与迁移证据 |
| DEC-013 | APLS 0.1 名称解析与最小 Action 集合 | 唯一候选、符号类别与无 Mutable Data 时禁用 `set` 的原则可复用；ASCII FQN 源码形式已被新 CNL 方向替代 | PARTIALLY_SUPERSEDED_BY_DEC-014 | `HDP-APLS-009 Option A` | 2026-09-03 | CNL 术语/引用重设输入 |
| DEC-014 | APLS 0.1 公开语言定位 | 受控自然语言是唯一公开 Surface Syntax；Canonical IR 是严格底层契约；用户默认不接触 DSL/AST/IR；多表面形式可归一，一句多解必须拒绝，LLM 不得作规范语义裁决器 | APPROVED | `HDP-APLS-010 Option A` | 2026-09-03 | 产品需求、架构、Language Profile、Compiler Frontend、诊断与 Conformance |
| DEC-015 | APLS 0.1 首个规范性自然语言 Profile | 首版只冻结简体中文单语 Profile；其他语言以后采用独立版本化 Profile，不混入 0.1 中文 Grammar | APPROVED | `HDP-APLS-011 Option A` | 2026-09-03 | CNL 词法、Grammar、Semantic Frame、诊断与 Conformance |
| DEC-016 | 简体中文 CNL 首版语言契约用途 | 批准 `apls-zh-CN-0.1` Profile、Grammar、Semantic Frame 和 CNL 诊断作为 Frontend 后续设计输入；不等于 Baseline 或代码修改授权；原一致性问题已由 DEC-017 关闭 | APPROVED | `HDP-APLS-012 Option A`、`Q-014 CLOSED` | 2026-09-03 | Frontend 技术适配与迁移设计 |
| DEC-017 | CNL 唯一性判定层级 | 采用最终规范语义唯一：中间 Tokenization、Parse Tree 和分析路径允许多候选；全部有效候选必须收敛为一个 Canonical Frame 等价类。零类为非法，两个及以上不等价类为 `AMBIGUOUS`；禁止任意优先级、最高概率、LLM 或第一个 Parse 制造唯一性 | APPROVED | `HDP-APLS-013 Option B`、Human Project Owner 高精度裁决 | 2026-09-03 | Source 合法集、PRD、Frontend、Frame、诊断与 Conformance |
| DEC-018 | CNL Frontend 与 IR Schema 闭合路线 | 保留 Rust + LALRPOP，在前端增加完整候选分析与 Canonical Frame 收敛层；代码迁移前先设计并批准 CNL Canonical IR、稳定 Semantic ID 和无损 Frame-to-IR 映射 | APPROVED | `HDP-APLS-014 Option A` | 2026-09-03 | CNL Frontend、Canonical IR、Schema 与工作包顺序 |
| DEC-019 | CNL Canonical IR、语义身份与单位语义 | 采用独立 `apls-cnl-ir-0.1`；用户不填写机器 ID；声明 ID 由 Kind/Owner/NFC 显示名机械生成，匿名节点和文档领域语义摘要采用版本化 SHA-256；等价匿名节点合并且 Provenance 取稳定并集；自定义单位为名义单位；0.1 仅允许单 Entry Source；旧 `apls-ir-0.1` 仅属 Legacy DSL | APPROVED | `HDP-APLS-015 Option A` | 2026-09-03 | CNL IR、Semantic ID、Hash、Unit、Publisher 与 Legacy 边界 |
| DEC-020 | 完整 CNL Compiler 实现包与候选资源边界 | 批准 `WP-APLS-CNL-C03-001` Option A：一次实现完整 `apls-zh-CN-0.1` Source→候选分析→唯一 Canonical Frame→复验 CNL IR→CLI 垂直切片；采用工作包第 3 节固定候选资源上限，首次超限为 `APLS-T0007`，不得误报歧义或产生部分产物 | APPROVED | `HDP-APLS-016 Option A` | 2026-09-03 | C03 Source/测试修改、候选资源计数、CNL Pipeline 与 CLI |
| DEC-021 | TASK-018 实施前契约整改路线 | 在恢复编码前先执行 `TASK-019`，一次关闭独立复审 BF-02～BF-10，形成语言/Frame/IR/资源/NFC/诊断统一候选并完成全新非正式独立再复审；公共语义、Schema 或依赖变化仍需后续精确批准 | APPROVED | `HDP-APLS-017 Option A` | 2026-09-03 | C02 整改、实施阻断、再复审顺序 |
| DEC-022 | 默认复审范围 | 当前 `LEAN` Profile 下普通评审和再复审默认采用 `DELTA_ONLY`：只审本轮改动、直接影响闭包与最小 Target 完整性证据；已关闭且未变化、未受影响的事项继承为 `INHERITED_CLOSED`。只有负责人明确要求、正式首轮 C04/Baseline/Release 强制范围、影响无法界定或证据不可复现时才升级为全量 | CONFIRMED | 项目负责人明确要求“今后默认只做增量复审” | 2026-09-04 | 全项目普通评审、再复审与保障成本控制 |
| DEC-023 | TASK-019 契约采用、Unicode 依赖与 TASK-018 恢复 | 采用经 `IIR-APLS-TASK019-REREVIEW-003` 通过的 TASK-019 语言/Frame/IR/Schema/资源/NFC/诊断契约作为实现输入；批准 `unicode-normalization =0.1.25, default-features=false` 的精确直接依赖边界和实际 Lock 核验；完成 TASK-019 并恢复 TASK-018 完整 CNL 垂直切片实现。不建立 Baseline | APPROVED | `HDP-APLS-018 Option A` | 2026-09-04 | 当前公共契约输入、依赖闭包、C03 Source/测试实施 |
| DEC-024 | TASK-018 实现整改与 Unicode 传递 Feature 闭包 | 接受锁定版本的实际闭包：`unicode-normalization/default-features=false`，`tinyvec 1.6.0` 激活 `alloc + default(empty) + tinyvec_macros`，不启用 `std`；版本、Checksum 与 Lock 不变。仅针对 IIR 的 `IO-01～IO-07` 重开 C03 Retry，完成后只做增量再复审 | APPROVED | `HDP-APLS-019 Option A` | 2026-09-04 | Unicode 证据同步、TASK-018 精确实现整改与最小测试 |
| DEC-025 | TASK-020 C05 验证范围 | 对 TASK-018 READY_FOR_REVIEW 候选执行最小风险驱动 C05：CNL-C001～C016 追溯、锁定普通构建/测试、公共候选资源边界和 Unicode 17.0.0 官方 NFC 一致性；官方数据仅下载至 `/private/tmp`。失败即返回 C00，不修改实现或测试 | APPROVED | `HDP-APLS-020 Option A` | 2026-09-04 | C05 验证、验证证据与后续正式 C04/Baseline 输入 |
| DEC-026 | APLS 0.1 实现候选 Git Anchor | 创建一个本地初始 Commit，冻结当前 C05 PASS 的非忽略项目文件；排除 `07_src/target/` 等构建/本地数据；Commit Message 固定为 `chore: freeze APLS 0.1 implementation candidate`。不 Push、不建立 Baseline、不发起 Formal C04 | APPROVED | 项目负责人在唯一下一步上下文中明确“授权”、`HDP-APLS-021 Option A` | 2026-09-04 | Git 候选 Anchor 与后续正式 C04 Target 准备 |
| DEC-027 | 私有远程同步与后续 AI 接管 | 创建 GitHub 私有仓库 `loocoo2025/apls-language`，同步本地 `main`，以正式 HANDOFF 将后续逻辑 C00 工作交给 Kimi；不授权 Formal C04、Baseline 或 Release | APPROVED | 项目负责人明确要求、`HDP-APLS-023` | 2026-09-04 | GitHub 私有远程、C00 物理会话交接 |

## 当前有效决定摘要

- 治理：`v0.1.5 + Lite + LEAN`。
- 产品：面向 AI Agent 的声明式软件规格语言及编译器式工具链。
- 首要质量目标：同一合法输入只有一个规范化解释；无法唯一解释时必须拒绝并诊断。
- 需求输入：当前 PRD 已获准用于架构设计，但产品 Baseline 尚未建立。
- 架构输入：`ARCH-APLS-001` 已获准用于详细设计，但总体架构 Baseline 尚未建立。
- 表面语法：受控自然语言是唯一公开 Source；旧花括号 DSL 仅是迁移参考。
- 首版语言范围：只接受简体中文语法结构；领域术语可按 Profile 使用汉字、拉丁字母和数字，但英文或混合自然语言句法不是 0.1 合法输入。
- 中文语言契约：四份 `TASK-015` 文档已获准作为设计输入，并已按 `DEC-017` 统一为“表层分析允许多候选，规范语义必须唯一”。
- 唯一性边界：只有最终存在两个及以上不等价 Canonical Frame 才是 `AMBIGUOUS`；中间 Tokenization/Parse 多候选但最终收敛的 Source 必须合法。
- Frontend 路线：保留 Rust + LALRPOP；新增 Candidate Lattice、逐候选解析和 Canonical Frame Convergence Gate，先闭合 CNL IR Schema 再授权代码迁移。
- CNL IR：`apls-cnl-ir-0.1` 已获准作为实现输入；Semantic ID、SHA-256、名义单位、重复匿名节点合并与单 Entry Source 规则按 `DEC-019` 执行。
- CNL 实现：`TASK-018` 按 `WP-APLS-CNL-C03-001` 完整垂直切片与固定候选资源边界执行；不允许临时公开子集、双入口或资源耗尽误报歧义。
- 实施前整改：`TASK-019` 已由 `HDP-APLS-018 Option A` / `DEC-023` 采用并完成；BF-02～BF-10 与 NF-01～NF-05 全部关闭。
- 当前 CNL 实施：`TASK-018` 已恢复，按 `WP-APLS-CNL-C03-001` 和 `DEC-023` 实现完整简体中文 Source→Canonical Frame→复验 IR→CLI 垂直切片；Unicode 依赖实际 Lock、Feature、License 与 MSRV 证据不一致时立即阻塞。
- 当前实现整改：`DEC-024` 已接受 `tinyvec/default=[]` 的实际空 Feature 闭包，并仅授权 `IO-01～IO-07` 的 C03 Retry；不得借整改改变公开语言、IR 语义、资源数值、依赖版本或 CLI 命令集合。
- 当前验证：`DEC-025` 已授权 TASK-020 只执行 T0/T1 风险驱动 C05、普通锁定验证、公共资源边界和 Unicode 17.0.0 官方一致性；失败时不得由 C05 修改实现或测试。
- 候选冻结：`DEC-026` 已授权创建一个本地初始 Commit 作为当前 C05 PASS 候选的不可变 Git Anchor；该动作不授权 Push、正式 C04、Baseline 或 Release。
- 远程与交接：`DEC-027` 授权 GitHub 私有同步和 Kimi C00 接管；这不构成 `HDP-APLS-022` 批准，Kimi C00 也不能在同一会话中冒充正式 C04。
- 复审范围：`LEAN` 默认 `DELTA_ONLY`；已关闭且未变化、未受影响的事项继承结论，不因出现在完整 Target 中而重复审查。
- 用户界面：用户默认不学习、不查看 DSL、AST 或 IR；只在主动检查/调试时展示 Normalized Meaning 或内部产物。
- 详细设计：旧 Grammar、词法和 `apls-ir-0.1` 已被方向变更替代，仅保留为 Legacy DSL 参考；当前 CNL-to-IR 契约为 `apls-cnl-ir-0.1`。
- Compiler MVP：公共命令、退出码、Tool Diagnostic、Envelope 和 Verified IR 发布契约已获准，但尚未建立产品 Baseline。
- 实现基础：Compiler MVP 采用 Rust 2024 + LALRPOP 默认 LR(1)，参考 Rust 1.98.0、MSRV 1.86.0 及 `DES-APLS-COMPILER-FOUNDATION-001` 的精确依赖与资源闭包；`TASK-007` 骨架已实现但尚未建立 Baseline。
- 精确词法：简体中文 CNL Profile 已获准进入精确设计；旧 `DES-APLS-LEX-001` 不再是当前公开 Source Profile。
- 名称解析：零/多候选、类别检查和禁止猜测仍有效；公开 CNL 术语和引用形式尚待重建。
- Action 边界：0.1 只保留 `emit/invoke/transition`；`set` 仅为保留关键字，未来恢复必须先冻结 Mutable Property/State Data 模型。
