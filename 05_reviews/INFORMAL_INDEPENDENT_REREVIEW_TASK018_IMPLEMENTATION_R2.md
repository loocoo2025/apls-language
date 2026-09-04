# TASK-018 Retry 2 NB-01 实现增量非正式独立再复审

## 1. 接任、身份与权限

- Review ID：`IIR-APLS-TASK018-IMPLEMENTATION-003`
- Target ID：`IIR-TARGET-APLS-TASK018-IMPLEMENTATION-003`
- Review Line：`INFORMAL_INDEPENDENT`
- Review Scope：`DELTA_ONLY / NB-01 ONLY`
- Reviewer：全新隔离独立 Reviewer；Session `task018_rereview_r2`；`private_context_inherited: false`
- Gate Authority：`ADVISORY_ONLY`
- Enforcement Mode：`PROCEDURAL_FALLBACK`；Mechanical Enforcement：`PARTIAL`
- 唯一写入：本报告；普通验证仅在已排除的 `07_src/target/` 产生构建输出
- 复审结论：`READY_FOR_C00_DETERMINATION`

项目接管核验：本项目是已规范化持续开发项目，当前处于 `IMPLEMENTATION`；本 Session 以 C04 稳定职责承担非正式独立 Reviewer，不是正式 C04。项目目标是实现拒绝歧义的 APLS CNL 编译器垂直切片；技术栈为 Rust 2024、LALRPOP 与锁定离线 Cargo 闭包。产品 Baseline 尚未建立，仓库尚无 Git Commit Anchor。当前唯一工作是 TASK-018 Retry 2 的 `NB-01 / IO-02` 增量关闭复核，无 P0/P1 未决问题。允许范围仅为读取冻结 Target、运行规定的最小普通验证并写入本报告；禁止修改 Target、实现、治理、Git/远程状态，以及执行 Formal C04、C05、Commit、Push、Baseline、Release 或 Seal。下一步只能由 C00 依本 Advisory 结论裁定。

`DYNAMIC_ROLE_PROFILE` 核验摘要：

```yaml
profile_id: DRP-APLS-C04-018-R2-REREVIEW
schema_version: "1.0"
role_id: C04
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C04_INDEPENDENT_REVIEW.md
project_ref: APLS
task_or_work_package_id: TASK-018 / RETRY2 / IIR-TARGET-APLS-TASK018-IMPLEMENTATION-003
current_or_applicable_gate_binding:
  gate_id: IIR_TASK018_RETRY2_NB01_REVIEW_PENDING
  gate_status: REVIEW_PENDING_AT_DISPATCH
  authority_source: CURRENT_STATE.md + exact Target
applicable_fact_owner_bindings:
  - CURRENT_STATE / DECISION_INDEX / ACTIVE_TASKS
  - approved Grammar and Semantic Validation contracts / C03-owned implementation and validation evidence
upstream_roles: [C00]
downstream_roles: [C00]
inputs: [21-file frozen Target, NB-01, IO-02 PARTIAL, DEC-022, DEC-024]
outputs: [this advisory rereview record]
allowed_process_starts: [INFORMAL_INDEPENDENT delta rereview]
allowed_tools_and_actions: [read Target, hash Target, run two targeted tests and offline fmt/check, write this record]
forbidden_actions_and_side_effects: [Target mutation, remediation, dependency install, network, Git mutation, Baseline, Formal C04, C05, Release, Seal]
authority_source: TASK018_INFORMAL_IMPLEMENTATION_REREVIEW_TARGET_R2.md
authorized_until: rereview record complete
human_escalation_triggers: [Current Truth conflict, Target change, retained human decision]
interaction_contract_refs: [INDEPENDENT_REVIEW / INFORMAL_INDEPENDENT]
model_binding: CURRENT_CODEX_FRONTIER_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: task018_rereview_r2 / isolated
enforcement_mode: PROCEDURAL_FALLBACK
validity: exact Target digest and this review only
invalidation_conditions: [Target digest mismatch, scope/authority/Gate change]
readiness: ROLE_PROFILE_READY
```

`KNOWLEDGE_MANIFEST` 使用最小充分加载：完整读取 `AI_START_HERE.md`、岗位政策/机器合同、C04 Role Brief、当前状态/决定/任务、保障节奏与测试治理；任务特定加载 Retry 2 Target、Retry 1 Target/报告、`HDP-APLS-019 / DEC-024`、三个 Delta 文件及 NB-01 直接引用的 Grammar/AST/语义契约。Target 摘要见第 2 节；未读取实现者私有推理，未扩展到其他已关闭项。`on_demand_governance_search: allowed`，`unresolved_rule_gap: NONE`。

## 2. Target 完整性

摘要严格按 Target 第 1 节算法独立复算：每个普通文件形成 `<sha256><two ASCII spaces><relative path><LF>`，按 C Locale 整行 Byte 升序排序后，对完整 21 行记录流计算 SHA-256。

| 时点 | 文件数 | 缺失 | Target Set SHA-256 | 结果 |
|---|---:|---:|---|---|
| 开始 | 21 | 0 | `07c8ff7935ce59af13890684a2ae82804ccd5fd92a5dc99a05fdb365192dea01` | MATCH |
| 报告写入前 | 21 | 0 | `07c8ff7935ce59af13890684a2ae82804ccd5fd92a5dc99a05fdb365192dea01` | MATCH |
| 结束 | 21 | 0 | `07c8ff7935ce59af13890684a2ae82804ccd5fd92a5dc99a05fdb365192dea01` | MATCH |

本报告不属于 Target Set；`07_src/target/` 也不属于 Target Set。

## 3. Delta、直接影响闭包与继承项

本轮语义复审只覆盖三个实际修改文件：

- `07_src/crates/apls-compiler/src/cnl_pipeline.rs`；
- `07_src/crates/apls-cli/src/lib.rs`；
- `11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md`。

直接影响闭包仅为 `NB-01 / IO-02`、Production 级 `ws1/opt-space` 约束、`20  %` 在 Parse/Check/Emit 阶段的边界、`APLS-E1101/APLS-E1401` 分类与两个定向用例。

| 继承组 | 状态 |
|---|---|
| `IO-01、IO-03～IO-07` | `INHERITED_CLOSED` |
| `IIR-APLS-TASK019-REREVIEW-003` 的 `BF-02～BF-10` | `INHERITED_CLOSED` |
| `IIR-APLS-TASK019-REREVIEW-003` 的 `NF-01～NF-05` | `INHERITED_CLOSED` |
| Retry 1 其他未受本轮变化影响的结论 | `INHERITED_CLOSED` |

未发现 Retry 2 变化破坏上述继承前提；未重复读取、重复测试或重新论证已关闭项，也未扩大为全项目扫描。

## 4. `NB-01 / IO-02` 关闭判定

状态：`CLOSED`。

| 关闭条件 | 独立直接证据 | 判定 |
|---|---|---|
| `Percentage / Temperature / Duration` 按专用 Production 执行 `opt-space` | EBNF `APLS_0.1_ZH_CN_GRAMMAR.ebnf:79-93,167-169` 分别定义三个专用 Production 和 `opt-space`；`cnl_pipeline.rs:1393-1410` 对三个对应 `RawLiteralKind` 只接受空 gap 或单个 ASCII 空格。 | PASS |
| `Quantity(number-with-declared-unit)` 始终执行 `ws1` | LALRPOP `apls_grammar.lalrpop:123-140` 将专用 Literal 和 `Quantity` 保留为不同 AST Kind；`cnl_pipeline.rs:1404-1422` 的 `Quantity` 分支无条件调用 `grammar_ws1`，不再查看解析后 Unit ID。 | PASS |
| `20  %` 专用 Percentage Candidate 失败，合法 Quantity Candidate 保留 | `cnl_pipeline.rs:3902-3921` 用同一 Source 直接断言 Percentage 为 false、Quantity 为 true；`cnl_pipeline.rs:150-204` 只在所有 Candidate 均失败时返回 Parse 诊断，因此保留合法 Quantity Parse Candidate。 | PASS |
| 四条命令保持正确阶段和诊断 | `apls-cli/src/lib.rs:522-552` 在真实公开命令入口上断言 `parse` 成功、`diagnose --through parse` 返回 `accepted`，`check/emit-ir` 为 Exit 1 且只命中 `APLS-E1401`而不是 `APLS-E1101`。 | PASS |
| 不扩张正式 EBNF 公开接受集，不改变 Frame/IR/资源/机器契约/命令/依赖/Lock | 生产变化只恢复 EBNF 已有 `number-with-declared-unit` 候选的 `ws1` 边界；`cnl_pipeline.rs:1262-1359` 仍在 Type Stage 按冻结 Comparison 矩阵使用 `APLS-E1401` 淘汰不兼容 Quantity，与 `APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md:47-72` 一致。CLI 文件的本轮闭包只是定向回归断言，验证记录与实际行为一致；未见其他直接影响偏差。 | PASS |

## 5. 必答问题与观察

| 问题 | 判定 | 说明 |
|---|---|---|
| `NB-01 / IO-02` 是否完成全部关闭条件 | YES | `CLOSED`；见第 4 节。 |
| 三个 Delta 是否只关闭 NB-01，且无新公开语言/阶段/诊断/测试偏差 | YES | 间距判定现与实际 Grammar Production 一致；Parse 与 Type 边界、E1101/E1401 分类和 C03 验证记录一致。 |
| 两个定向测试与最小 locked/offline 验证是否足够 | YES | Compiler 单元用例直接区分两个 Production；CLI 用例覆盖四条公开命令及阶段诊断。源码/契约审查、`fmt --check` 和 workspace locked/offline check 足以覆盖本局部低成本 Delta；Retry 2 C03 已有普通 Compiler/CLI 回归证据，不需重跑全套。 |
| 是否仍有阻止 TASK-018 进入 `READY_FOR_REVIEW` 的直接实现缺陷 | NO | 本轮范围内无新阻断观察；继承关闭前提未被破坏。 |

新阻断观察：`NONE`。Advisory：`NONE`。

## 6. 独立机械核验

使用 Target 指定的本地锁定离线工具环境：`CARGO_HOME=/private/tmp/apls-cargo`、`RUSTUP_HOME=/private/tmp/apls-rustup`、`PATH=/private/tmp/apls-cargo/bin:$PATH`。

| 检查 | 结果 |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo check --workspace --locked --offline` | PASS |
| `cargo test -p apls-compiler --locked --offline literal_spacing_uses_the_actual_grammar_production` | PASS：1 passed，24 filtered out |
| `cargo test -p apls-cli --locked --offline all_public_commands_enforce_state_list_and_literal_spacing_grammar` | PASS：1 passed，7 filtered out；CLI binary target 0 tests |
| Target Digest（开始/报告写入前/结束） | 三次 MATCH |

未运行 Compiler/CLI 全套普通测试，未运行百万级资源边界用例或 Unicode 20,034 行昂贵用例；这些都不是 NB-01 增量关闭的必要证据。Homebrew 启动脚本输出的 `/bin/ps: Operation not permitted` 环境提示不影响上述命令 Exit 0 或验证结果。

## 7. 结论与边界

`NB-01 = CLOSED`；`IO-02 = CLOSED`；`IO-01、IO-03～IO-07 = INHERITED_CLOSED`。本轮无新阻断观察。结论：`READY_FOR_C00_DETERMINATION`。

本结论仅是绑定上述精确 21 文件 Target 的非正式独立 Advisory，不是正式 C04 Gate Decision，不自行把 TASK-018 改为 `READY_FOR_REVIEW`，也不授权 C05、Commit、Push、Baseline Adoption、Release 或 Formal Seal。报告完成后停止，不参与整改。
