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
| DEC-028 | 首次正式 C04 授权 | 批准 `HDP-APLS-022 Option A`：以精确 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e` 为唯一只读 Review Target，由全新独立 Session 执行 `FULL_SCOPE` 首次正式 C04，仅写入 `05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md`；不授权 Baseline Adoption、Release、Formal Seal 或对 Target/源码/测试/设计/Schema/Manifest/Lock 的任何修改 | APPROVED | 项目负责人 2026-09-05 明确回复 `HDP-APLS-022: APPROVED / OPTION: A` | 2026-09-05 | 一次性 Formal C04 Dispatch、正式 Review Record、后续 Baseline 裁决输入 |
| DEC-029 | 首次正式 C04 重新 Dispatch 方式 | 第一次 Dispatch（`NISR-APLS-C04-022-001`）已消费且被平台中止，无 Review Record、无 Gate Decision、Target 完好；授权由项目负责人手动新建干净会话执行首次正式 C04，C00 提供自包含启动指令；Target、FULL_SCOPE、只读与唯一写入边界不变 | APPROVED | 项目负责人 2026-09-05 明确指示“我新建一个干净会话，然后独立 C04” | 2026-09-05 | 首次正式 C04 手动独立 Session 与结果回收 |
| DEC-030 | 首次正式 C04 Finding 受控整改组织 | 接受 `FORMAL_C04_APLS_0_1_CANDIDATE_001 = CHANGES_REQUESTED`（F-01～F-09）；授权 C00 组织整改：F-01/F-02（S1）经 Expert Escalation，由全新 C02 Session 形成覆盖全部 Finding 的整改工作包候选与 `HDP-APLS-024`；本授权不含代码修改，批准前不得改动 Compiler/CLI Source、测试、设计、Schema、Manifest 或 Lock | APPROVED | 项目负责人 2026-09-05 明确选择“授权组织整改工作包” | 2026-09-05 | TASK-024 整改工作包、后续精确整改授权 |
| DEC-031 | 整改工作包批准与 F-02/F-06/F-04/F-08 方向 | 批准 `HDP-APLS-024` 全部推荐：`WP-APLS-C04-REMEDIATION-001` 作为整改依据；Q1=A（0.1 冻结最小直接冲突模型，REQUIRE×PROHIBIT 同 Canonical Condition + 同行为三元组拒绝，新码 `APLS-E1405`）；Q2=B（C01 修订 PRD-007 验收措辞与失败关闭设计一致，纯文档变更）；Q3=A（实现诊断目录 §1.1 封闭根因聚合 + 七码处置表）；Q4=A（建立最小完备集 AC-001～009/SYS-001～009/NFR-001～006/IF-001～003 并落盘 FORMAL_TRACE 边，追溯门禁 PASS 为完成条件）；Q4-a=否（PRD §6 不先补编号，NFR 行 Traces-From 用最直接相关 P0 需求 ID 并注明源自 §6）；分两批整改，不含 Commit/新 Anchor/Baseline/Release 授权 | APPROVED | 项目负责人 2026-09-06 明确“按照你推荐的来” | 2026-09-06 | TASK-024 批次 1/2 整改实施、公共语言收窄（E1405）、诊断目录与 PRD-007 文档变更、需求追溯层建立 |
| DEC-032 | Rust 工具链安装授权 | 授权在本机安装 rustup + Rust 1.98.0（参考工具链）与 1.86.0（MSRV 核验），并允许 `cargo fetch/build/test --locked` 按 Cargo.lock 联网拉取精确锁定依赖；不授权变更依赖闭包、版本或 Lock | APPROVED | 项目负责人 2026-09-06 明确选择“授权安装 Rust 工具链” | 2026-09-06 | TASK-024 批次 1 实施与验证环境前提 |
| DEC-033 | 整改收口三项裁决与新 Anchor | ① 接受 C01 对 `system_architecture.md` §8 的扩展修正（F-06 同根因）；② 授权修正 `APLS_0.1_ZH_CN_GRAMMAR.ebnf` 第 3 行过期状态注释（仅注释，不动语法产生式）；③ 参照 DEC-026 模式授权创建一个包含全部整改与治理落盘的新精确 Git Anchor 作为 Finding 关闭复审 Target；随后 C00 发起全新独立 C04 关闭复审；不含 Push/Baseline/Release/Formal Seal 授权 | APPROVED | 项目负责人 2026-09-06 明确选择“三项全部批准” | 2026-09-06 | TASK-024 收口、新 Review Target、Finding 关闭复审 |
| DEC-034 | APLS 0.1 产品 Baseline Adoption 与远程同步 | 批准 `HDP-APLS-025`：Q1=A，采用精确 Commit `fd8b59536fcdfdff2f3b199b882c15d97edb1993` 为 `APLS-0.1-BASELINE-001`（需求、语言规范、Canonical IR、编译器实现、测试与追溯层转为 CURRENT，版本摘除 draft 标记）；Q2=A，授权提交 Anchor 后治理回执为一个新 Commit 并将 `main` Push 到私有远程；不含 Release/Tag/Formal Seal/公开仓库授权 | APPROVED | 项目负责人 2026-09-06 明确回复 `HDP-APLS-025: APPROVED`（Q1=A/Q2=A） | 2026-09-06 | 产品 Baseline 成立、治理回执提交、私有远程同步 |
| DEC-035 | Profile §11 样例缺陷修正 | `APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md` §11“目标体验样例”缺少指向“激活”状态的转换，被 Baselined 编译器以 E1402 拒绝；按项目负责人“立即处理”指令修正样例（补 `急停命令` 事件与转换句），修正后样例经编译器实测 Exit 0；属编辑性文档修正，不改变任何规范性语言行为，不触发正式 C04；交付文档一律使用中文 | APPROVED | 项目负责人 2026-09-06 明确指令“立即处理，另外给我的文档要都是中文的” | 2026-09-06 | Profile 文档货币性、用户教程语言约束 |
| DEC-036 | 教程与样例修正的提交与远程同步 | 授权提交当前全部未提交文件（`docs/APLS_0.1_USER_TUTORIAL.md`、Profile §11 修正、治理回执与 DEC-035/036）为一个新 Commit 并 Push `main` 到私有远程；不含 Release/Tag/Formal Seal 授权 | APPROVED | 项目负责人 2026-09-06 明确指令“提交并同步远程” | 2026-09-06 | 教程发布到仓库、私有远程同步 |

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
- 首次正式 C04：`DEC-028` 已批准 `HDP-APLS-022 Option A`；首次 Dispatch 中断后由 `DEC-029` 授权项目负责人手动新建干净会话执行，Target 与边界不变；Baseline Adoption 仍须另行裁决。
- 产品 Baseline：`DEC-034` 已批准 `HDP-APLS-025`（Q1=A/Q2=A），`APLS-0.1-BASELINE-001` 成立于精确 Commit `fd8b59536fcdfdff2f3b199b882c15d97edb1993`；Release 与 Formal Seal 未授权。
- 复审范围：`LEAN` 默认 `DELTA_ONLY`；已关闭且未变化、未受影响的事项继承结论，不因出现在完整 Target 中而重复审查。
- 用户界面：用户默认不学习、不查看 DSL、AST 或 IR；只在主动检查/调试时展示 Normalized Meaning 或内部产物。
- 详细设计：旧 Grammar、词法和 `apls-ir-0.1` 已被方向变更替代，仅保留为 Legacy DSL 参考；当前 CNL-to-IR 契约为 `apls-cnl-ir-0.1`。
- Compiler MVP：公共命令、退出码、Tool Diagnostic、Envelope 和 Verified IR 发布契约已获准，但尚未建立产品 Baseline。
- 实现基础：Compiler MVP 采用 Rust 2024 + LALRPOP 默认 LR(1)，参考 Rust 1.98.0、MSRV 1.86.0 及 `DES-APLS-COMPILER-FOUNDATION-001` 的精确依赖与资源闭包；`TASK-007` 骨架已实现但尚未建立 Baseline。
- 精确词法：简体中文 CNL Profile 已获准进入精确设计；旧 `DES-APLS-LEX-001` 不再是当前公开 Source Profile。
- 名称解析：零/多候选、类别检查和禁止猜测仍有效；公开 CNL 术语和引用形式尚待重建。
- Action 边界：0.1 只保留 `emit/invoke/transition`；`set` 仅为保留关键字，未来恢复必须先冻结 Mutable Property/State Data 模型。
