# TASK-018 Retry 1 实现增量非正式独立再复审

## 1. 接任、身份与权限

- Review ID：`IIR-APLS-TASK018-IMPLEMENTATION-002`
- Target ID：`IIR-TARGET-APLS-TASK018-IMPLEMENTATION-002`
- Review Line：`INFORMAL_INDEPENDENT`
- Review Scope：`DELTA_ONLY`
- Reviewer：全新隔离独立 Reviewer；Session `task018_rereview_r1`；`private_context_inherited: false`
- Gate Authority：`ADVISORY_ONLY`
- Enforcement Mode：`PROCEDURAL_FALLBACK`；Mechanical Enforcement：`PARTIAL`
- 唯一写入：本报告；普通测试仅在已排除的 `07_src/target/` 产生构建输出
- 复审结论：`CHANGES_RECOMMENDED`

项目接管核验：本项目为已规范化持续开发项目，当前处于 `IMPLEMENTATION`；本 Session 以 C04 稳定 Role Brief 承担非正式独立 Reviewer，而非正式 C04。项目目标是实现拒绝歧义的 APLS CNL 编译器垂直切片；技术栈为 Rust 2024、LALRPOP 与锁定离线 Cargo 闭包。产品 Baseline 尚未建立、Git Commit Anchor 缺失；当前工作是 TASK-018 Retry 1 的七项实现关闭再复审，无 P0/P1 未决问题。允许范围仅为读取冻结 Target、运行最小普通验证和写本报告；禁止修改 Target、设计、代码、测试、Cargo、Current Truth、Git/远程状态及执行 Baseline、Formal C04、C05、Release 或 Seal。下一步只能由 C00 根据本 Advisory 结论裁定。

`DYNAMIC_ROLE_PROFILE` 核验摘要：

```yaml
profile_id: DRP-APLS-C04-018-R1-REREVIEW
schema_version: "1.0"
role_id: C04
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C04_INDEPENDENT_REVIEW.md
project_ref: APLS
task_or_work_package_id: TASK-018 / RETRY1 / IIR-TARGET-APLS-TASK018-IMPLEMENTATION-002
current_or_applicable_gate_binding:
  gate_id: IIR_TASK018_RETRY1_DELTA_REVIEW_PENDING
  gate_status: REVIEW_PENDING_AT_DISPATCH
  authority_source: CURRENT_STATE.md + exact Target
applicable_fact_owner_bindings:
  - CURRENT_STATE / BASELINE_INDEX / DECISION_INDEX / ACTIVE_TASKS
  - approved design contracts / C03-owned implementation and validation evidence
upstream_roles: [C00]
downstream_roles: [C00]
inputs: [72-file frozen Target, IO-01..IO-07, DEC-022, DEC-024]
outputs: [this advisory rereview record]
allowed_process_starts: [INFORMAL_INDEPENDENT delta rereview]
allowed_tools_and_actions: [read Target, hash Target, offline fmt/check/test/tree, write this record]
forbidden_actions_and_side_effects: [Target mutation, implementation/remediation, dependency install, network, Git mutation, Baseline, Formal C04, C05, Release, Seal]
authority_source: TASK018_INFORMAL_IMPLEMENTATION_REREVIEW_TARGET_R1.md
authorized_until: rereview record complete
human_escalation_triggers: [Current Truth conflict, Target change, retained human decision]
interaction_contract_refs: [INDEPENDENT_REVIEW / INFORMAL_INDEPENDENT]
model_binding: CURRENT_CODEX_FRONTIER_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: task018_rereview_r1 / isolated
enforcement_mode: PROCEDURAL_FALLBACK
validity: exact Target digest and this review only
invalidation_conditions: [Target digest mismatch, scope/authority/Gate change]
```

`KNOWLEDGE_MANIFEST` 使用最小充分加载：完整读取 `AI_START_HERE.md`、岗位政策/机器合同、C04 Role Brief、当前状态/基线/决定/任务、保障节奏与测试治理；任务特定加载 Target、上一份 IO 定义、`HDP-APLS-019 / DEC-024`、七个 Delta 文件及 IO 直接引用的语言/IR/资源/诊断/CLI 契约。Target 集合摘要见第 2 节；未读取实现者私有推理、实现 HANDOFF 或无关历史内容。`on_demand_governance_search: allowed`，`unresolved_rule_gap: NONE`。

## 2. Target 完整性

摘要严格按 Target 第 1 节算法独立复算：每个普通文件形成 `<sha256><two ASCII spaces><relative path><LF>`，按 C Locale 整行 Byte 升序排序后，对完整 72 行记录流计算 SHA-256。

| 时点 | 文件数 | 缺失 | Target Set SHA-256 | 结果 |
|---|---:|---:|---|---|
| 开始 | 72 | 0 | `c3ef44330dd560569149a065fe8623de4741e7c7161bf038e5553aa831b6ced9` | MATCH |
| 报告写入前 | 72 | 0 | `c3ef44330dd560569149a065fe8623de4741e7c7161bf038e5553aa831b6ced9` | MATCH |
| 结束 | 72 | 0 | `c3ef44330dd560569149a065fe8623de4741e7c7161bf038e5553aa831b6ced9` | MATCH |

本报告不属于 Target Set；`07_src/target/` 也不属于 Target Set。

## 3. Delta、直接影响闭包与继承项

本轮语义复审只覆盖 Target 第 2.1 节的七个实际修改文件：Unicode Profile、CNL Lexer、CNL Pipeline、Resource Ledger、CLI Library/Main 和 C03 Retry 1 验证记录。直接影响闭包只覆盖 `IO-01～IO-07`、Cargo/Lock/Feature、固定资源值、Unicode 版本、四条公共命令与旧 DSL 隔离。

| 继承组 | 状态 |
|---|---|
| `IIR-APLS-TASK019-REREVIEW-003` 的 BF-02～BF-10 | `INHERITED_CLOSED` |
| `IIR-APLS-TASK019-REREVIEW-003` 的 NF-01～NF-05 | `INHERITED_CLOSED` |
| 上一轮中与 IO-01～IO-07 无关且未被 Retry 触及的实现结论 | `INHERITED_CLOSED` |

未发现 Retry 变化破坏上述继承前提；未扩大为全量复审。

## 4. IO-01～IO-07 关闭判定

| IO | 状态 | 独立直接证据 |
|---|---|---|
| IO-01 | `CLOSED` | `APLS_0.1_CNL_UNICODE_NFC_PROFILE.md:44-62` 已按 `HDP-APLS-019 Option A / DEC-024` 明示真实闭包；`07_src/Cargo.toml:26` 仍是 `unicode-normalization =0.1.25, default-features=false`；`Cargo.lock:1084-1096,1147-1150` 固定版本与三项获批 Checksum。独立离线 `cargo tree -p apls-compiler -i tinyvec -e features --locked` 只显示 `tinyvec/alloc + default + tinyvec_macros`，未显示 `std`；本地锁定 `tinyvec 1.6.0` Manifest 机械确认 `default=[]`、`alloc=["tinyvec_macros"]`、`std=["alloc"]`。 |
| IO-02 | `PARTIAL` | `cnl_lexer.rs:34-102` 已删除状态列表的非 Grammar 字面量 `且`，Profile `ws1` 也在 `cnl_pipeline.rs:291-367` 前置验证；但 `literal_spacing_is_valid` 按解析后的内建 Unit ID 覆盖了候选实际 Production 的 `ws1`，使合法的 `number-with-declared-unit` Parse Candidate 在公开 Parse Gate 被错误淘汰。详见第 5 节 `NB-01`。 |
| IO-03 | `CLOSED` | `cnl_pipeline.rs:206-271` 按不含 Provenance 的 Canonical Frame Byte 建等价类，并用 `BTreeSet<Provenance>` 合并每个成功候选；`:1700-1797` 再按 Sentence 聚合、去重并稳定输出全部 Role Span，不保留获胜 Parse。`:3690-3723` 的真实多 Tokenization 用例直接断言 `behavior_action={启动,启动水}`、`behavior_target={泵,水泵}` 同时进入唯一 Source Map Entry。 |
| IO-04 | `CLOSED` | `cnl_pipeline.rs:1668-1697` 对实际待交付 Canonical Byte 重新解析、Schema 验证并在 `VerifiedArtifact` 构造前调用 `cross_validate`；`:1799-2373` 重新执行 ID/Ref、Property/Comparison/Literal、Action Target、Transition 局部/冲突/可达、正 Time Deadline、Unit Materialization Closure、Source Map/Span/Pointer/排序/覆盖、Semantic Hash 和 Condition 顺序不变量。`:3424-3557` 的定向篡改矩阵覆盖本 IO 新增语义闭包；原有 Source Map 篡改检查继续直接覆盖 Pointer 与 Role 顺序。 |
| IO-05 | `CLOSED` | `cnl_pipeline.rs:841-877` 的生产 `process_candidate` 明确先完成 `bind_candidate` 才登记 Bound，完成 `type_candidate` 才登记 Typed，完成 Normalize 才登记 Canonical；`:880-1089` 显示绑定与类型阶段边界。`:3896-3981` 通过同一生产函数证明 Owner 绑定失败为 `[0,0,0]`，类型失败为 `[1,0,0]`；`resource.rs:68-97` 仍使用同一登记函数和首次超限失败语义。 |
| IO-06 | `CLOSED` | `cnl_pipeline.rs:1503-1601` 的 E1402 Primary 从对应 State Source Entry 的 `state` Role Span 取得，Related 为该模型全部 Transition Sentence Span；E1403 对冲突组 Span 稳定排序后取第一条为 Primary、其余为 Related。`:1076-1084,1171-1180` 的 E1307 Primary 为 Deadline Span。`:3253-3305` 对两个稳定排序等价类收集全部 Differing Role Provenance 并取最小包含区间。`:3778-3880` 分别断言 E1402/E1403、E1307、E1310 Span，且 E1307/E1310 Envelope 重复执行 Byte 相同；生产 BTree 集合与统一诊断归一化也保持 Related/Envelope 稳定。 |
| IO-07 | `CLOSED` | `apls-cli/src/lib.rs:108-125` 的统一物理输出函数把非空 stdout 的 `write_all` 或 `flush` 失败都收口为 Exit 2，并尽力向 stderr 写入且 flush 固定 `APLS-T0005`；`main.rs:5-14` 已接入该唯一出口。`lib.rs:457-491` 的可控中途写失败用例证明已暴露前缀、Exit 2 与精确 stderr Byte；同一条件表达式直接覆盖 flush 失败分支。 |

## 5. 新阻断观察

### NB-01 — Parse Gate 把 `number-with-declared-unit` 的 `ws1` 错按内建单位 `opt-space` 拒绝

- 直接代码证据：`07_src/crates/apls-compiler/src/cnl_pipeline.rs:1393-1416` 对 `RawLiteralKind::Quantity` 先查看解析后的 `unit.id`；只要 ID 是 `%/摄氏度/毫秒/秒/分钟` 就要求 `gap == " "`。因此一个由 `number-with-declared-unit` 产生、使用两个空格等合法 `ws1` 的 Candidate，会因 Unit 恰为内建单位而在 `compile_inner:160-170` 的 Parse 前置检查中被淘汰。`build_graph:729-733` 明确把这些内建单位加入 Terminology Graph；LALRPOP `apls_grammar.lalrpop:123-140,149-154` 允许 `SignedNumber + UnitRefRule` 形成 `RawLiteralKind::Quantity`。
- 违反的精确契约：`APLS_0.1_ZH_CN_GRAMMAR.ebnf:79-93,104,167-169` 将 `number-with-declared-unit = signed-number, ws1, unit-reference` 与专用 `percentage-value/temperature-value/duration` 的 `opt-space` 分开定义；`ws1` 接受一个或多个结构空白，而 `opt-space` 只接受零或一个 ASCII 空格。`APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md:69-72` 又明确承认专用 Production 与 `number-with-declared-unit` 可对同一 Source 形成候选，不能靠 Production 优先级选择。即使该 Quantity Candidate 随后因 Comparison 封闭矩阵在 Check 阶段被淘汰，Parse Gate 也不能先把 Grammar 合法 Candidate 当成空白违规删除。
- 可复现证据：`apls-cli/src/lib.rs:522-543` 的 Retry 测试把 `20  %`（两个 ASCII 空格）断言为四命令均 Exit 1；其中 `parse`/`diagnose --through parse` 的拒绝正好固化了上述偏差。按正式 EBNF，Fixed `%` 的专用 `percentage-value` Candidate 因 `opt-space` 失败，但内建 UnitRef `%` 的 `number-with-declared-unit` Candidate 满足 `ws1`，所以 Parse 阶段应成功；后续 Check 是否满足 Percentage 的专用 Source 语义是独立问题。
- 影响：Retry 为关闭 IO-02 而缩窄了公开 Grammar Gate，并使测试把一个语义阶段约束写成 Parse 阶段接受条件；不满足“所有 `ws1/opt-space` 约束按各自 Production 在 Parse Gate 执行”，也不满足“未产生新的公开接受集偏差”。
- 关闭条件：间距验证必须绑定 Candidate 的实际 Grammar Production：`Percentage/Temperature/Duration` 执行 `opt-space`，`Quantity(number-with-declared-unit)` 执行 `ws1`，不得按解析后的 Unit ID 改写 Production 约束；定向测试至少分别证明 `20  %` 在 Parse Gate 保留合法 `number-with-declared-unit` Candidate、在适用的 Check 语义矩阵中确定处理，并证明专用 Production 的真正 `opt-space` 违规不能作为该专用 Candidate 通过。

## 6. 直接影响闭包与必答问题

| 检查 | 判定 | 证据与结论 |
|---|---|---|
| 七个 Delta 是否只实现批准关闭语义 | FAIL | IO-01、IO-03～IO-07 的修改落点与批准关闭语义一致；IO-02 的生产间距检查按语义 Unit ID 改写 Grammar Production 约束，缩窄公开 Parse 接受集。 |
| Cargo/Lock/依赖 | PASS | `Cargo.toml`、锁定版本和 Checksum 与 `DEC-024` 一致；Feature 树为批准闭包且无 `unicode-normalization/std` 或 `tinyvec/std`。 |
| 固定资源数值 | PASS | `resource.rs:6-10` 仍为 `1,000,000 / 4,096 / 1,000,000 / 1,000,000 / 1,000,000`；既有 `limits.rs` 数值与批准 Profile 保持一致。 |
| Unicode | PASS | Compiler 常量、IR Header 与 Profile 均为 NFC / Unicode `17.0.0`；本轮未重跑已忽略的官方 20,034 行昂贵证据。 |
| 公共 CLI 与旧 DSL 隔离 | PASS | 命令仍仅为 `parse/check/emit-ir/diagnose` 与 `--version`；公共入口调用 CNL `compile`，CNL IR Schema 常量仍只指向 `apls-cnl-ir-0.1.schema.json`；旧 DSL 输入回归仍拒绝。 |
| 新增最小测试是否足以支持关闭 | PARTIAL | IO-01、IO-03～IO-07 的普通测试和源码/契约证据充分；IO-02 测试把 `20  %` 的 Grammar 合法通用 UnitRef Candidate 错误断言为 Parse 失败，不能支持完整关闭。按 Target 未重跑两项昂贵用例。 |
| 是否仍有阻止回到 `READY_FOR_REVIEW` 或后续 C05 的实现缺陷 | YES | `NB-01 / IO-02 PARTIAL` 直接违反唯一 EBNF 的 Parse Gate 接受边界；应形成精确 Retry 和新 Target 后再复审。 |

## 7. 独立机械核验

使用本地工具链：`CARGO_HOME=/private/tmp/apls-cargo`、`RUSTUP_HOME=/private/tmp/apls-rustup`、`PATH=/private/tmp/apls-cargo/bin:$PATH`。

| 检查 | 结果 |
|---|---|
| `cargo fmt --all --check` | PASS |
| `cargo check --workspace --locked --offline` / Rust 1.98.0 | PASS |
| `cargo +1.86.0 check --workspace --locked --offline` | PASS |
| `cargo test -p apls-compiler --locked --offline` | PASS：23 passed，1 ignored |
| `cargo test -p apls-cli --locked --offline` | PASS：7 passed，1 ignored |
| `cargo tree -p apls-compiler -i tinyvec -e features --locked --offline` | PASS：`alloc + default + tinyvec_macros`，未出现 `std` |
| Target Digest（开始/写入前/结束） | 三次 MATCH |

未执行百万级资源边界用例与 Unicode 20,034 行官方用例；两项均按 Target 要求保持 ignored，不被本 Reviewer 冒充为本轮独立重跑。普通工具输出中的 Homebrew `/bin/ps: Operation not permitted` 环境提示不影响各命令 Exit 0 或验证结果。

## 8. 结论与边界

`IO-01、IO-03～IO-07 = CLOSED`；`IO-02 = PARTIAL`；存在新阻断观察 `NB-01`。结论：`CHANGES_RECOMMENDED`。

本结论仅是绑定上述精确 72 文件 Target 的非正式独立 Advisory，不是正式 C04 Gate Decision，不自行把 TASK-018 改为 `READY_FOR_REVIEW`，也不授权 C05、Commit、Push、Baseline Adoption、Release 或 Formal Seal。报告完成后停止，不参与整改。
