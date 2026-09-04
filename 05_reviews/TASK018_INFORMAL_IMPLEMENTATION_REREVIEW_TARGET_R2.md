# TASK-018 Retry 2 NB-01 实现增量独立再复审 Target

- Target ID：`IIR-TARGET-APLS-TASK018-IMPLEMENTATION-003`
- 冻结日期：`2026-09-04`
- Review Line：`INFORMAL_INDEPENDENT`
- Review Scope：`DELTA_ONLY / NB-01 ONLY`
- Target Access：`READ_ONLY`
- 文件数：`21`
- Target Set SHA-256：`07c8ff7935ce59af13890684a2ae82804ccd5fd92a5dc99a05fdb365192dea01`
- 直接输入：`IIR-APLS-TASK018-IMPLEMENTATION-002 = CHANGES_RECOMMENDED`
- Retry 授权：`HDP-APLS-019 Option A / DEC-024 / IO-02 DIRECT CLOSURE`

本 Target 和 Reviewer 输出报告不属于 Target Set，避免自引用。仓库尚无 Git Commit，本轮使用受控文件集及 SHA-256 冻结当前版本。本轮不是正式 C04，不产生正式 Gate Decision。

## 1. 摘要算法

对第 4 节列出的每个相对项目根目录普通文件计算 SHA-256，形成：

```text
<64 lowercase hex><two ASCII spaces><relative path><LF>
```

记录按 C Locale 整行 Byte 升序排序，再对完整记录流计算 SHA-256。Reviewer 必须在开始、报告写入前和结束时分别复算；任一不匹配即停止并报告 `TARGET_CHANGED_REVIEW_INVALID`。

## 2. 本轮唯一增量范围

### 2.1 Retry 2 实际 Delta

只复核以下三个实际修改文件：

```text
07_src/crates/apls-compiler/src/cnl_pipeline.rs
07_src/crates/apls-cli/src/lib.rs
11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md
```

### 2.2 NB-01 直接关闭条件

1. `Percentage / Temperature / Duration` Candidate 按各自专用 Production 执行 `opt-space`；
2. `Quantity(number-with-declared-unit)` Candidate 始终执行 `ws1`，不得依据解析后的 Unit ID 改写其 Production 约束；
3. Source `20  %` 的专用 Percentage Candidate 因 `opt-space` 不成立而不得通过该专用路径，但合法 Quantity Candidate 必须被 Parse Gate 保留；
4. `parse` 与 `diagnose --through parse` 对上述 Source 成功；`check` 与 `emit-ir` 按现行 Percentage 专用 Source 类型矩阵以 `APLS-E1401` 拒绝，不得误报 Parse Stage 的 `APLS-E1101`；
5. 修改不得产生超出正式 EBNF 的新公开接受集，不得改变 Canonical Frame、IR、资源、诊断机器契约、公共命令、依赖或 Lock。

### 2.3 继承项

- 上一份报告中的 `IO-01、IO-03～IO-07 = CLOSED`，本轮标记为 `INHERITED_CLOSED`，不得重复阅读、重复测试或重新论证；
- `IIR-APLS-TASK019-REREVIEW-003` 的 `BF-02～BF-10、NF-01～NF-05 = INHERITED_CLOSED`；
- 上一轮其他未被 Retry 2 变化影响的结论全部继承；
- 第 4 节中的治理和契约文件仅用于最小权威、完整性及直接影响闭包，不把全文纳入语义复审。

仅当 Retry 2 的三文件变化直接破坏某个继承结论成立前提时，Reviewer 才可重新打开该项或记录新观察；不得扩张为全项目扫描。

## 3. Reviewer 必须回答的问题

1. `NB-01 / IO-02` 是否满足第 2.2 节全部关闭条件；
2. 三个实际修改文件是否只关闭 NB-01，是否产生新的公开语言、阶段边界、诊断或测试偏差；
3. 两个定向测试与最小锁定离线验证是否足以支持关闭；
4. 是否仍存在阻止 TASK-018 进入 `READY_FOR_REVIEW` 的直接实现缺陷。

`NB-01 / IO-02` 必须给出 `CLOSED / PARTIAL / OPEN` 和直接证据。只有它为 `CLOSED`、继承前提未被破坏且没有新阻断观察时，才可输出 `READY_FOR_C00_DETERMINATION`。

## 4. 精确 Target 文件集

```text
AI_START_HERE.md
00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md
00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml
00_project/governance/PROJECT_ASSURANCE_CADENCE_POLICY.md
00_project/governance/AI_TESTING_GOVERNANCE_RULES.md
00_project/ai_context/ROLE_BRIEFS/C04_INDEPENDENT_REVIEW.md
00_project/ai_context/CURRENT_STATE.md
00_project/ai_context/ACTIVE_TASKS.md
00_project/ai_context/DECISION_INDEX.md
00_project/ai_context/HUMAN_DETERMINATION_019.md
00_project/ai_context/EXECUTION_CONTEXT_TASK018_RETRY2.md
04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md
04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf
04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md
05_reviews/TASK018_INFORMAL_IMPLEMENTATION_REREVIEW_TARGET_R1.md
05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK018_IMPLEMENTATION_R1.md
07_src/crates/apls-compiler/src/apls_grammar.lalrpop
07_src/crates/apls-compiler/src/cnl_ast.rs
07_src/crates/apls-compiler/src/cnl_pipeline.rs
07_src/crates/apls-cli/src/lib.rs
11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md
```

## 5. Reviewer 身份、权限与输出

- 必须使用未参与 TASK-018 实现、Retry 1、Retry 2 或此前复审的全新隔离 Session；`private_context_inherited: false`；
- 角色：独立 Reviewer；Review Line：`INFORMAL_INDEPENDENT`；Gate Authority：`ADVISORY_ONLY`；
- 只读 Target；禁止修改设计、代码、测试、Cargo、Current Truth、任务状态、Git 或远程状态；
- 唯一允许写入：`05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK018_IMPLEMENTATION_R2.md`；允许普通测试在已排除的 `07_src/target/` 产生构建输出；
- 不安装或升级依赖、不联网、不执行 Commit、Push、Baseline Adoption、Formal C04、C05、Release 或 Formal Seal；
- 只复核三个 Delta 文件、NB-01 关闭和直接影响闭包；其他事项一律继承；
- 最小机械检查限于两个定向测试、`fmt --check` 和必要的普通 `check --locked --offline`；若源码与已有普通回归证据足够，不得重跑 Compiler/CLI 全套普通测试；绝不重复百万级资源或 Unicode 20,034 行昂贵用例；
- 新阻断观察必须给出文件/行、违反的精确契约、影响和关闭条件；非阻断事项标记 `ADVISORY`；不得使用正式 C04 S0～S3；
- 结论只允许：`READY_FOR_C00_DETERMINATION`、`CHANGES_RECOMMENDED`、`TARGET_CHANGED_REVIEW_INVALID`；
- 报告必须记录开始、写入前和结束三次 Target Digest；完成后停止，不参与整改。
