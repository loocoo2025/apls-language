# APLS 当前任务注册表

| Task | 类型 | 责任角色 | 输入 | 输出 | 任务状态 |
|---|---|---|---|---|---|
| TASK-001 | PRODUCT_REQUIREMENTS | C01 | 用户目标、DEC-003、DEC-004、APLS 研究资料 | `01_product_requirements/PRD.md` 候选 | DONE |
| TASK-002 | ARCHITECTURE_DESIGN | C02 | `01_product_requirements/PRD.md`、DEC-003～DEC-005 | `03_architecture/system_architecture.md` 候选 | DONE |
| TASK-003 | DETAILED_LANGUAGE_DESIGN | C02 | DEC-004、DEC-006、DEC-007、`03_architecture/system_architecture.md` | 正式语法、IR Schema、诊断目录详细设计候选 | DONE |
| TASK-004 | COMPILER_MVP_DESIGN | C02 | DEC-004、DEC-006、DEC-008、已批准详细设计输入 | Compiler MVP 组件、阶段、公共命令和失败关闭详细设计候选 | DONE |
| TASK-005 | IMPLEMENTATION_TECHNOLOGY_DECISION | C02 | DEC-004、DEC-009、`DES-APLS-COMPILER-001` | Compiler 实现语言与 Parser 技术 ADR、Human Determination Package | DONE |
| TASK-006 | COMPILER_IMPLEMENTATION_FOUNDATION_DESIGN | C02 | DEC-004、DEC-009、DEC-010、`DES-APLS-COMPILER-001` | Rust Compiler 实现基础设计候选、依赖决策包 | DONE |
| TASK-007 | COMPILER_SCAFFOLD_IMPLEMENTATION | C03 | DEC-009、DEC-010、DEC-011、`DES-APLS-COMPILER-FOUNDATION-001` | Cargo Workspace、`apls-compiler` / `apls-cli` 骨架、精确依赖锁与最小构建门禁 | DONE |
| TASK-008 | LEXICAL_SPEC_CLOSURE | C02 | DEC-004、DEC-007、DEC-010、DEC-011、`DES-APLS-LANG-001` | 精确词法 Profile 候选与 `HDP-APLS-008` | DONE |
| TASK-009 | DETERMINISTIC_LEXER_IMPLEMENTATION | C03 | DEC-004、DEC-010～DEC-012、`DES-APLS-LEX-001` | 完整手写 Lexer、Token/Byte Span、稳定词法 Finding 与最小定向测试 | OUTPUT_READY |
| TASK-010 | FORMAL_GRAMMAR_PARSER_IMPLEMENTATION | C03 | DEC-004、DEC-007、DEC-010～DEC-012、`DES-APLS-LANG-001`、正式 EBNF、TASK-009 Lexer | 唯一完整 LALRPOP Grammar、私有 Surface AST、Parser 适配、稳定语法 Finding 与最小定向测试 | OUTPUT_READY |
| TASK-011 | SOURCE_GRAPH_IMPORT_LOADING | C03 | DEC-004、DEC-009、DEC-011、`DES-APLS-COMPILER-001`、`DES-APLS-COMPILER-FOUNDATION-001`、TASK-009/010 | 逻辑路径、注入式 Reader、不可变 Source Bundle、稳定 Import Graph、SHA-256/竞态复核、图诊断与资源上限 | OUTPUT_READY |
| TASK-012 | NAME_RESOLUTION_SEMANTICS_CLOSURE | C02 | DEC-004、DEC-007、DEC-009、已批准 Language/IR/Diagnostic/Compiler 设计 | `DES-APLS-NAME-001`、Rule Gap Report、`HDP-APLS-009`、同步后设计 | DONE |
| TASK-013 | DECLARATION_INDEXER_RESOLVER_IMPLEMENTATION | C03 | DEC-004、DEC-009、DEC-013、同步后的 Language/Grammar/IR/Schema、TASK-009～012 | Parser/AST `set` 同步、Declaration Index、BoundProgram、E2001/E2002/E2003/E2005/E4004 与最小定向测试 | OUTPUT_READY |
| TASK-014 | CONTROLLED_NATURAL_LANGUAGE_ARCHITECTURE | C02 | DEC-004、DEC-014、CR-APLS-001、IA-APLS-001、PRD-009 | CNL 产品/架构同步、Semantic Frame 流水线、歧义边界、迁移影响与 `HDP-APLS-011` | DONE |
| TASK-015 | SIMPLIFIED_CHINESE_CNL_LANGUAGE_CONTRACT | C02 | DEC-004、DEC-014、DEC-015、ARCH-APLS-CNL-001、DES-APLS-CNL-AMB-001 | 简体中文 Profile、正式 EBNF 候选、Semantic Frame、CNL 诊断与 `HDP-APLS-012` | DONE |
| TASK-016 | CNL_FRONTEND_ADAPTATION_AND_IR_MAPPING | C02 | DEC-010、DEC-011、DEC-014～DEC-017、TASK-015 | Parser 技术适配评估、CNL-to-IR 映射和 C03 迁移工作包候选 | DONE |
| TASK-017 | CNL_CANONICAL_IR_SCHEMA_DESIGN | C02 | DEC-014～DEC-018、DES-APLS-CNL-FRAME-001、DES-APLS-CNL-IR-MAP-001 | CNL Canonical IR 设计、JSON Schema、稳定 Semantic ID、无损 Frame 映射与 `HDP-APLS-015` | DONE |
| TASK-018 | CNL_FRONTEND_FRAME_IR_IMPLEMENTATION | C03 | DEC-010、DEC-011、DEC-014～DEC-020、DEC-023、DEC-024、WP-APLS-CNL-C03-001 | 完整简体中文 CNL Frontend、候选收敛、Canonical Frame、`apls-cnl-ir-0.1`、失败关闭 Pipeline/CLI 与最小定向测试 | DONE |
| TASK-019 | CNL_PREIMPLEMENTATION_CONTRACT_CLOSURE | C02 | IIR-APLS-TASK018-PREIMPL-001、IIR-APLS-TASK019-REREVIEW-001/002/003、DEC-014～DEC-023 | 关闭 BF-02～BF-10、NF-01～NF-05，形成一致语言/Frame/IR/资源/NFC/诊断契约候选与新 Target | DONE |
| TASK-020 | CNL_IMPLEMENTATION_VERIFICATION | C05 | TASK-018 READY_FOR_REVIEW、IIR-APLS-TASK018-IMPLEMENTATION-003、CNL-C001～C016、DEC-025 | 风险驱动验证计划、锁定构建/测试、资源与 Unicode 一致性证据、剩余风险和验证结论 | DONE |
| TASK-021 | IMMUTABLE_CANDIDATE_COMMIT | C00 | TASK-018/TASK-020 DONE、DEC-026 | 一个本地初始 Commit、精确 Commit Hash、staged set 和禁止对象核验 | DONE |
| TASK-022 | INITIAL_FORMAL_C04 | C04 | 精确 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e`、C05 PASS、HDP-APLS-022（APPROVED Option A / DEC-028）、DEC-029 | 首次正式 C04 Review Record 与 Gate Decision | DONE |
| TASK-023 | PRIVATE_GITHUB_SYNC_AND_KIMI_HANDOFF | C00 | 用户明确授权、Commit `3289037bee1aab64dfa2d58188379a68dcfa601e` | GitHub 私有仓库、同步 `main`、Kimi C00 HANDOFF 与启动指令 | DONE |
| TASK-024 | FORMAL_C04_FINDING_REMEDIATION | C02/C03 | `FORMAL_C04_APLS_0_1_CANDIDATE_001`（CHANGES_REQUESTED，F-01～F-09）、DEC-030、DEC-031、DEC-032、DEC-033 | 整改工作包、HDP-APLS-024、精确整改、新 Review Target 与 Finding 关闭复审 | DONE |

允许状态：`TODO → READY → IN_PROGRESS → OUTPUT_READY → READY_FOR_REVIEW → DONE`，以及受控的 `BLOCKED / CANCELLED` 分支。

> `TASK-009/010/013` 保留为 `DEC-014` 前旧 DSL 方向的已完成输出和迁移证据；它们不构成当前 CNL Compiler 符合性证据。

`TASK-001` 的 Done 条件：

- 产品定位、用户、范围和非目标明确；
- “拒绝歧义”被转化为可验证的 P0 要求；
- 不把尚未决定的语法、技术栈或后端写成事实；
- 候选完成自检，但不冒充正式需求 Baseline。

`TASK-002` 的 Done 条件：

- 确定性编译流水线和组件边界明确；
- 每个歧义类别都有失败关闭位置；
- Canonical IR 的规范性、排序、版本和溯源原则明确；
- LLM 不进入规范编译路径；
- 未决的表面语法、实现语言和参考后端不被擅自冻结；
- 候选完成最小一致性检查，但不冒充已批准架构或产品 Baseline。

`TASK-003` 的 Done 条件：

- `.apls` 词法规则和无歧义 Grammar 候选完整；
- 每个规范性语法结构存在唯一 Canonical IR 映射；
- JSON IR Schema 关闭未知字段并区分 normative/informative/open/unknown；
- 诊断分类覆盖词法、语法、名称、类型、语义和 IR；
- Q-002/Q-003 不被擅自冻结；
- 产物通过最小机械一致性检查，但不冒充已批准详细设计或 Baseline。

`TASK-004` 的 Done 条件：

- Compiler MVP 阶段和组件边界与 `ARCH-APLS-001` 一致；
- Parse、Check、Emit IR、Diagnose 四类能力具有唯一的命令契约和退出语义；
- Import 加载、诊断排序、阶段交付和 IR 发布均为确定、失败关闭；
- Debug AST 与 Verified IR 机械区分，Agent 不会误消费部分结果；
- Q-002/Q-003/Q-007 不被擅自冻结；
- 候选完成最小一致性检查，但不冒充已实现、已验证或已建立 Baseline。

`TASK-005` 的 Done 条件：

- 候选技术栈使用一手官方资料核验当前能力；
- 比较 Grammar 冲突失败、类型安全、确定序列化、可移植分发、开发成本和依赖风险；
- 明确推荐方案及不采用其他方案的理由；
- 形成 ADR 候选和可复制的 Human Determination 回复格式；
- 不安装依赖、不编写代码、不冒充已选定技术栈。

`TASK-006` 的 Done 条件：

- 使用官方一手资料提出 Rust Edition、MSRV、LALRPOP/Runtime 和直接依赖的精确版本候选；
- 将 Compiler Stage 映射到明确的 Rust Crate/Module 边界与互斥 Artifact 类型；
- 定义 Parser 生成、零冲突失败、生成物管理和 `Cargo.lock` 规则；
- 定义 Canonical JSON、IR 复验、Publisher、资源上限和稳定诊断的实现接缝；
- 形成依赖与实现基础 Human Determination Package；
- 不安装依赖、不创建 Compiler 代码、不冒充已经构建或验证。

`TASK-007` 的 Done 条件：

- `07_src/` 建立 Rust 2024 / Resolver 3 Workspace，两个 Crate 边界与 `DEC-011` 一致；
- Workspace 声明 MSRV 1.86.0、参考 Rust 1.98.0，直接依赖精确锁定且无 Git Dependency；
- LALRPOP Build Script 写入 `OUT_DIR`、禁止 `LALRPOP_LANE_TABLE`，不建立伪造或不完整的第二 Grammar；
- Compiler Stage 用互斥 Rust 类型建立骨架，`VerifiedArtifact` 无公共构造器；
- CLI 命令表面与已批准契约一致，未实现的编译操作明确失败关闭，不产生 Artifact；
- 生成 `Cargo.lock`，通过最小 `fmt/check/test --locked` 验证；
- 不实现正式 Grammar 转换、完整 Compiler Stage 行为、Canonical Writer 或 Publisher，不 Commit/Push/Release/Baseline/C04。

`TASK-008` 的 Done 条件：

- 识别当前已批准词法设计中会改变 Source 接受集合或诊断结果的未定义项；
- 精确定义空白、换行、Byte Span、行列、数字非法边界、JSON Unicode 转义和块注释行为候选；
- 不使用实现库默认值或 Unicode 版本表替代语言规则；
- 形成边界完整、可复制回复的 Human Determination Package；
- 批准前不修改已批准语言设计、不实现 Lexer/Parser、不建立 Baseline。

`TASK-009` 的 Done 条件：

- Token Kind 完整覆盖正式 Grammar 的关键字、标点、运算符和字面量类别；
- 严格实现 `DES-APLS-LEX-001` 的 UTF-8/BOM、ASCII 空白、注释、标识符、数字和字符串规则；
- Token 与内部 Finding 携带零起点半开 Byte Span，并可按批准规则换算一基行列；
- 词法失败稳定区分 `APLS-E1001/E1002/E1003`，Token 超限稳定形成 `APLS-T0007`，不产生下游 Artifact；
- 使用最少高价值测试覆盖主要合法路径、边界与三类词法错误，不进行 Fuzz、全排列或 Parser 测试；
- 不实现 Parser、Surface AST、Resolver、Canonical IR 或 Publisher，不增加依赖，不 Commit/Push/Release/Baseline/C04。

`TASK-010` 的 Done 条件：

- `APLS_0.1_GRAMMAR.ebnf` 的全部生产式一次接入唯一 `.lalrpop` Grammar，不存在临时子集或第二 Parser；
- 默认 Lane-Table LR(1) 生成零冲突，任何冲突或生成失败直接使构建失败；
- Parser 仅消费手写 Lexer Token，不启用内置 Lexer、Error Recovery、AI 猜测或隐式顺序消歧；
- 完整合法 Source 产生 Crate 内私有的有类型 Surface AST，字符串按已批准 JSON Scalar 规则解码；
- 词法失败不构造 `ParsedProgram`，意外 Token 归一为 `APLS-E1101`，必需结构缺失、重复或顺序非法归一为 `APLS-E1103`；
- 用 3～8 个高价值测试覆盖完整声明集正常路径、表达式优先级、字符串解码、词法失败和语法失败；
- 不实现 Source Graph、Resolver、Types、Semantic、Canonical IR、CLI 或 Publisher，不增加依赖，不 Commit/Push/Release/Baseline/C04。

`TASK-011` 的 Done 条件：

- Import 路径仅接受相对 Entry Import Root 的 `/` 分隔 `.apls` 逻辑路径，拒绝空段、`.`、`..`、反斜线、URI Scheme 和绝对路径前缀；
- Compiler Core 只通过注入式 Reader 读取 Source，不直接读宿主路径、当前目录、环境变量或网络；
- Entry 优先、其他 Source 按逻辑路径 UTF-8 Byte 升序发现，只从完整 Parser 成功 AST 提取 Import；
- 每个 Source 保存不可变 Byte Buffer 和 SHA-256，并提供交付前重读比对接缝，改变时以 `APLS-T0004` 失败；
- 非法/不安全路径、不可读/非普通文件、同一物理文件多逻辑路径分别稳定归一为 `APLS-T0002/T0003`；
- Import 循环和同一 Source 的重复别名分别归一为 `APLS-E2004/E2002`，不依赖声明顺序选择解释；
- 严格执行单 Source Byte、Source Graph 总 Byte、Source 数、Import 深度和全图 Token 总数上限，首个超限点以 `APLS-T0007` 失败；
- 用 3～8 个高价值测试覆盖正常图、非法路径/读取、循环/重复、竞态和资源边界；
- 不实现真实主机文件系统适配、CLI、Resolver、Types、Semantic、Canonical IR 或 Publisher，不增加依赖，不 Commit/Push/Release/Baseline/C04。

`TASK-012` 的 Done 条件：

- 给出顶层、State、Transition、Knowledge 和局部 Binding 的唯一命名空间模型；
- 给出当前 Spec、显式 Import Alias 和 State 的唯一限定形式；
- 覆盖每个 Qualified Name 语法位置的期望符号类别；
- 明确识别 `set` 缺少可写目标模型，给出不由实现猜测的处理选项；
- 形成可复制回复的 Human Determination Package；
- 批准前不修改已批准 Grammar/IR，不修改 Compiler Source，不建立 Baseline。

`TASK-013` 的 Done 条件：

- 唯一 LALRPOP Grammar、Surface AST 和 Parser 测试与 `DEC-013` 同步，`set` 保留 Token 但不再形成合法 Action；
- 索引 Spec、顶层声明、State/Transition、Enum Variant、Knowledge 与必要局部 Binding，不依赖遍历顺序选择声明；
- 实现当前 Spec、局部 Binding、显式 Import Alias 和 State/Enum Variant 的唯一限定形式；
- 所有 Qualified Name 按 `DES-APLS-NAME-001` 类别矩阵唯一绑定，失败稳定归一为 `APLS-E2001/E2002/E2003/E2005`；normative 依赖 `open/unknown` 以 `APLS-E4004` 失败且不重复发出 E2005；
- Indexer 或 Resolver 存在 Error 时不构造部分 Declaration Index 或 `BoundProgram`；
- 用最少高价值测试覆盖本地/导入正常绑定、State/Enum Variant、重复、未定义、多候选防御和类别错误；
- 不实现 Type/Unit、Semantic、Canonicalizer、IR Validator、CLI 或 Publisher，不增加依赖，不 Commit/Push/Release/Baseline/C04。

`TASK-014` 的 Done 条件：

- `DEC-014` 正式替代 `DEC-007`，旧词法和名称源码形式的适用性影响已明确；
- PRD 和总体架构已同步“受控自然语言唯一公开 Surface”定位；
- 建立确定性 CNL Source 到 Typed Semantic Frame、Ambiguity Gate 和 Canonical IR 的单一流水线；
- 冻结未定义术语、模糊量、所属、指代、模态、否定、并列、时间和单位的候选拒绝边界；
- 明确 AI 只在规范编译事务之外解释或草拟修改，不裁决规范含义；
- 完成旧设计/代码的可复用、失效与重新批准影响分析；
- 形成 `HDP-APLS-011` 供项目负责人决定首个规范性自然语言 Profile；
- 不重写代码，不 Commit/Push/Release/Baseline/C04/Formal Seal。

`TASK-015` 的 Done 条件：

- Profile 明确 UTF-8/NFC、中文标点、结构空白、术语字符与首句版本声明；
- 声明术语与裸引用采用确定的两阶段索引，零候选或多候选失败；
- EBNF 覆盖实体、属性、动作、事件、状态、规则、禁止、转换、安全不变量、验收和说明句；
- 三种批准的条件引导形式映射为同一 Rule Frame；
- Typed Semantic Frame 的必需角色、类别、单位和 Source Span 完整；
- CNL 诊断覆盖编码、句界、术语、角色、模态、模糊量、指代、并列、否定和时间；
- 形成 `HDP-APLS-012` 供项目负责人冻结首版公开语言契约；
- 不修改 Compiler Source/测试/依赖，不 Commit/Push/Release/Baseline/C04/Formal Seal。

`TASK-016` 的 Done 条件：

- 先关闭 `Q-014`，冻结词法切分、Parse 与 Frame 的唯一性层级；
- 证明保留 LALRPOP、增加受控前置层或更换 Parser 路线中的唯一推荐方案；
- 给每个 CNL Frame 建立到 Canonical IR 的逐字段映射或明确 Schema Gap；
- 形成最小 C03 Frontend 迁移工作包候选，不直接修改代码；
- 不改变公开语言接受集，不修改依赖，不 Commit/Push/Release/Baseline/C04/Formal Seal。

`TASK-016` 的 `RULE_CONFLICT / Q-014` 已由 `HDP-APLS-013 Option B` / `DEC-017` 关闭。输出见 `DES-APLS-CNL-FRONTEND-001`、`DES-APLS-CNL-IR-MAP-001` 和 `HDP-APLS-014`；`Q-015` 已关闭，后续由 `TASK-017` 承接。

`TASK-017` 的 Done 条件：

- 新 IR Schema 对每个 CNL Canonical Frame 具有无损、唯一映射，不复用不兼容的旧字段；
- Named Symbol、Owner-scoped State/Action 和匿名规范节点具有确定、可审计的 Semantic ID 候选；
- Condition、Typed Literal、Rule Modality、Transition、Invariant、Acceptance 和 InformativeItem 具有封闭结构；
- 自定义 Unit 在不猜测换算关系的前提下具有确定首版语义；
- Candidate Provenance 并集与规范语义 Hash 输入机械隔离；
- Schema 关闭未知字段并通过 JSON 语法与最小结构自检；
- 形成 `HDP-APLS-015`，批准前不修改 Compiler Source、测试、依赖或旧 IR Schema，不 Commit/Push/Release/Baseline/C04/Formal Seal。

`TASK-017` 已由 `HDP-APLS-015 Option A` / `DEC-019` 接受为后续实现输入并完成。

`TASK-018` 的 Done 条件：

- 唯一公开 Source 路径完整实现 `apls-zh-CN-0.1` 全部声明、规则、转换、不变量、验收和说明句，不发布临时语言子集或双入口；
- Candidate Lattice、完整 Token Stream 枚举、逐候选 LALRPOP 解析、确定绑定/定型和 Canonical Frame `0/1/>1` 收敛全部失败关闭；
- 多个中间候选收敛到同一 Frame 时接受，两个及以上不等价 Frame 使用 `APLS-E1310` 拒绝，资源不足使用 `APLS-T0007` 且不产生 Frame/IR；
- `apls-cnl-ir-0.1` 的 ID、Hash、名义单位、重复节点 Provenance 并集、Canonical Writer、Schema 与跨引用验证全部实现；
- `parse/check/emit-ir/diagnose` 不得把旧 DSL、AST、部分 Frame 或未复验 JSON 作为成功产物；
- 完成 `WP-APLS-CNL-C03-001` 规定的最小定向验证，并审查变更差异；
- 除 `HDP-APLS-018` 后续精确批准的 Unicode 直接/传递闭包外，不改变依赖闭包、公共语言契约或批准资源值；不 Commit/Push/Baseline/C04/Release/Formal Seal。

> `TASK-018` 已由 `HDP-APLS-016 Option A` / `DEC-020` 获得实施授权，但因 `IIR-APLS-TASK018-PREIMPL-001` 的 10 项 Blocking Finding 转为 `BLOCKED`。BF-01 已由 C00 状态同步关闭；BF-02～BF-10 由 `TASK-019` 承接，关闭并完成全新独立再复审前不得恢复编码。

`TASK-019` 的 Done 条件：

- 明确修正旧映射文件的历史 Gate 与当前 Gate；
- 关闭状态比较示例、Property/Comparison 类型单位矩阵和 Transition 可达/冲突算法边界；
- 冻结候选资源的实现无关计数对象、顺序、Span 和 `APLS-T0007` Payload；
- 冻结 Unicode 数据版本、NFC 实现来源及相应依赖/供应链候选；
- 闭合合取去重、内建单位 materialization、Source Map/Span 和公共诊断机器契约；
- 形成最小 Conformance Case，不编写 Compiler 实现；
- 形成新的精确 Target 并通过全新 `INFORMAL_INDEPENDENT` 再复审；
- 不修改 Compiler Source/测试/依赖/Cargo Lock，不 Commit/Push/Baseline/Formal C04/Release/Formal Seal。

> `TASK-019` 已由 `HDP-APLS-017 Option A` / `DEC-021` 批准进入 `IN_PROGRESS`。输出和再复审完成后仍需精确批准，才能改变当前公共契约、依赖闭包或恢复 `TASK-018`。

> 第一次再复审 `IIR-APLS-TASK019-REREVIEW-001` 结论为 `CHANGES_RECOMMENDED`：BF-02/03/04/05/08/09 已关闭，BF-06/07/10 部分关闭，NF-01～NF-04 由同一 TASK-019 Retry Scope 承接。第二次精确 Target 通过前，Done 条件未满足，`TASK-018` 继续阻塞。

> 第二次再复审 `IIR-APLS-TASK019-REREVIEW-002` 仍为 `CHANGES_RECOMMENDED`：NF-01/02/04 和 BF-07 已关闭；BF-06/BF-10/NF-03 仅因 NF-05 的资源超限黑盒不可达而保持部分关闭。第三轮在不改变资源数值的前提下修正证据分层；第三个精确 Target 通过前 `TASK-018` 继续阻塞。

> 第三次再复审 `IIR-APLS-TASK019-REREVIEW-003` 按项目负责人要求采用 `DELTA_ONLY`，结论为 `READY_FOR_HUMAN_DETERMINATION`：BF-02～BF-10 与 NF-01～NF-05 全部关闭，未变化项以 `INHERITED_CLOSED` 继承。`TASK-019` 等待 `HDP-APLS-018` 采用其输出；`TASK-018` 在裁决前继续阻塞。

> `HDP-APLS-018 Option A` / `DEC-023` 已采用 TASK-019 输出并批准精确 Unicode 依赖边界。TASK-019 进入 `DONE`；TASK-018 已按合法状态路径 `BLOCKED → READY → IN_PROGRESS` 恢复，后续普通复审按 `DEC-022` 默认只做增量范围。

> `TASK-018` 曾完成 C03 实施与 `WP-APLS-CNL-C03-001` 最小定向验证并进入 `OUTPUT_READY`，证据见 `11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md`；该状态已被后续独立增量复审结论取代，不冒充 Baseline 或 Release。

> `IIR-APLS-TASK018-IMPLEMENTATION-001` 已按 `DEC-022` 完成 `DELTA_ONLY` 非正式独立复审，结论为 `CHANGES_RECOMMENDED`，形成 `IO-01～IO-07`。TASK-018 已按合法路径 `OUTPUT_READY → BLOCKED`，等待 `HDP-APLS-019`；批准前不修改实现或证据。

> `HDP-APLS-019 Option A` / `DEC-024` 已接受实际空 `tinyvec/default` Feature 闭包，并仅针对 `IO-01～IO-07` 重开 TASK-018。任务已按合法路径 `BLOCKED → READY → IN_PROGRESS` 进入精确 C03 Retry；完成后必须形成新增量 Target，不能直接进入 Baseline 或 Release。

> TASK-018 Retry 1 已完成 C03 修正与最小定向验证，当前按 `IN_PROGRESS → OUTPUT_READY` 等待新独立 Session 的 `DELTA_ONLY` 关闭复审；C03 的关闭主张不替代独立结论。

> `IIR-APLS-TASK018-IMPLEMENTATION-002` 已关闭 IO-01/03/04/05/06/07，IO-02 因 `NB-01` 保持 Partial。NB-01 是 IO-02 的直接闭包，不需要新产品裁决；TASK-018 按 `OUTPUT_READY → BLOCKED → READY → IN_PROGRESS` 进入只含 NB-01 的 Retry 2。

> TASK-018 Retry 2 已仅修正 NB-01 并通过最小定向验证，任务按 `IN_PROGRESS → OUTPUT_READY` 等待全新独立 Session 只复核 NB-01；此前已关闭 IO 全部继承，不重复复审。

> `IIR-APLS-TASK018-IMPLEMENTATION-003` 已按 21 文件小型 Target 完成 `DELTA_ONLY / NB-01 ONLY` 独立复核：`NB-01 / IO-02 = CLOSED`，其余 IO 继承关闭，无新阻断观察。C00 已按合法路径将 TASK-018 从 `OUTPUT_READY → READY_FOR_REVIEW`。TASK-020 已定义但尚未获 C05 执行授权。

> `HDP-APLS-020 Option A` / `DEC-025` 已授权 TASK-020 按最小风险驱动范围进入 C05；只运行批准的普通锁定验证、公共候选资源边界和 Unicode 17.0.0 官方一致性，不修改实现或测试。

> `VAL-APLS-C05-020 = PASS`：普通 Compiler 24/CLI 7 项、Rust 1.98/1.86 锁定检查、公共候选资源边界和 Unicode 17.0.0 官方全量一致性全部通过；74 个受保护设计/源码/测试文件前后摘要一致。C00 已将 TASK-018 与 TASK-020 收口为 `DONE`，但产品 Baseline、正式 C04 和 Release 仍未完成。

> 项目负责人已授权当前唯一下一步；`HDP-APLS-021 Option A / DEC-026` 只允许创建一个本地初始 Commit 冻结当前 C05 PASS 候选。构建缓存已通过 `.gitignore` 排除，不授权 Push、Formal C04、Baseline 或 Release。

> TASK-021 已完成：本地初始 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e`、Tree `51edf42da73237bcb3408e234d2cbb2095758fa1`，共 249 个文件；`07_src/target/` 未进入 Commit。该 Anchor 尚未经过正式 C04，也未采用为产品 Baseline。

> TASK-024 已完成：`HDP-APLS-024`（DEC-031，Q1=A/Q2=B/Q3=A/Q4=A/Q4-a=NO）批准 `WP-APLS-C04-REMEDIATION-001`；批次 1（C03-v02：F-01/F-02/F-03/F-04/F-05/F-07+A-01 代码与契约）与批次 2（C01-v02：F-08 追溯层 27 ID/30 FORMAL_TRACE 边、F-06 PRD-007 措辞、F-09 文档货币性）先后完成；DEC-033 接受 §8 扩展修正、授权 EBNF 状态注释修正并建立新 Anchor `fd8b59536fcdfdff2f3b199b882c15d97edb1993`；关闭复审 `FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001 = PASS`，F-01～F-09 全部 CLOSED，Open Finding = 0，无越权改动。

> TASK-022 已完成：`FORMAL_C04_APLS_0_1_CANDIDATE_001 = CHANGES_REQUESTED`（READY，Open Findings 9：S1 F-01/F-02，S2 F-03/F-04/F-05/F-06/F-08，S3 F-07/F-09；证据受限 E-01～E-03）。首次 Dispatch `NISR-APLS-C04-022-001` 中断后由 `DEC-029` 授权的手动干净会话（APLS-C04-Independent-Review-v02）完成评审；Target 三时点身份核验一致。
