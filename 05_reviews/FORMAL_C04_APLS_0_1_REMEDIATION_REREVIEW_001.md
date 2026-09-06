# FORMAL C04 — APLS 0.1 整改候选 Finding 关闭复审

```yaml
review_id: FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001
review_line: FORMAL_C04
reason_code: FORMAL_C04_REREVIEW
review_purpose: FINDING_CLOSURE_REREVIEW_OF_FORMAL_C04_APLS_0_1_CANDIDATE_001
review_scope: DELTA_ONLY + 九条关闭条件必查项
physical_session_id: APLS-C04-Independent-Review-v03
review_target_commit: fd8b59536fcdfdff2f3b199b882c15d97edb1993
review_target_tree: c672fe950c4ef9a75af0fe4430254982a5ab175b
review_target_file_count: 256
base_commit: 3289037bee1aab64dfa2d58188379a68dcfa601e
review_date: 2026-09-06
gate_decision: PASS
```

## 1. Dispatch 凭据摘要

- 授权依据：`DEC-033`（项目负责人 2026-09-06 明确批准，承接 `HDP-APLS-024` / `DEC-031`）；上游评审 `FORMAL_C04_APLS_0_1_CANDIDATE_001 = CHANGES_REQUESTED`（9 项 Open Finding：S1×2、S2×5、S3×2）。
- 整改工作包：`04_design/compiler/WP-APLS-C04-REMEDIATION-001.md`；裁决方向已在 Target 内 `HUMAN_DETERMINATION_024.md` 与 `DECISION_INDEX.md`（DEC-031/032/033）核实：Q1=A（E1405 直接冲突拒绝）、Q2=B（PRD-007 措辞修订）、Q3=A（§1.1 封闭根因聚合 + 七码处置）、Q4=A（最小追溯层）、Q4-a=否。
- 本 Session 即 `DEC-033` 授权发起的全新独立 C04 复审 Session，只读 Target，唯一写入路径为本文件。

## 2. 独立性证据块（formal_c04_independence_evidence）

```yaml
review_id: FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001
reviewer_role: C04
new_session_id: APLS-C04-Independent-Review-v03
new_session_evidence: >-
  项目负责人按 DEC-033 手动新建的干净会话；本会话未读取任何实现/整改 Session 的
  HANDOFF 或私有推理，全部评审事实从精确 Commit 副本与主仓库正式治理文件重建；
  对 F-01～F-09 的每条判定均以本评审在 Target fd8b595 上独立读取代码/文档与
  独立执行的机械验证为证据，未继承整改方自检结论。
execution_or_remediation_session_ids_excluded:
  - 全部 C02 整改设计 / C03 整改实现 / C01 需求层落盘 Session（TASK-024 批次 1/2）
  - APLS-C04-Independent-Review-v02（上游首次评审，仅其正式 Review Record 作为 Finding 清单与关闭条件来源）
private_context_inherited: false
context_package_manifest:
  - 主仓库：AI_START_HERE.md、00_project/ai_context/DECISION_INDEX.md（DEC-031/032/033）
  - Target 副本（/private/tmp/apls-c04-rereview-fd8b595，checkout 自精确 Commit fd8b595）：
    AI_ENGINEERING_RULES_V2.md §38.7、HUMAN_DETERMINATION_024.md、
    WP-APLS-C04-REMEDIATION-001.md、FORMAL_C04_APLS_0_1_CANDIDATE_001.md（第 7 节关闭条件）、
    04_design 全套契约、02_system_requirements 全套、01_product_requirements、
    03_architecture、06_test_design、07_src 全部源码与内嵌测试、09_quality 追溯脚本
exact_immutable_target: fd8b59536fcdfdff2f3b199b882c15d97edb1993
target_binding_evidence: 见第 9 节三个时点的 cat-file/rev-parse/ls-tree 核验输出
review_target_access: READ_ONLY
allowed_write_paths:
  - /Users/luchuang/Documents/ai编程语言/05_reviews/FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001.md  # 本文件
  - /private/tmp/apls-c04-rereview-fd8b595  # Target 只读审查副本及构建/黑盒输出目录
git_write_allowed: false
remote_mutation_allowed: false
pre_review_target_status: UNCHANGED（时点 1/2 核验一致，见第 9 节）
post_review_target_status: UNCHANGED（时点 3 核验结果见第 9 节，于本文件写入后立即执行并回填）
review_record: 05_reviews/FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001.md
```

注：历史残留 `/private/tmp/apls-c04-review-3289037` 与 `/private/tmp/apls-c04-review-v2` 未读取、未使用，不作为任何证据。

## 3. Review Readiness（AI_ENGINEERING_RULES_V2.md §38.7.1）

```text
REVIEW_ID: FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001
REVIEW_TARGET: APLS 0.1 整改后实现候选（本地精确 Commit）
EXACT_GIT_COMMIT_OR_CONTROLLED_VERSION: fd8b59536fcdfdff2f3b199b882c15d97edb1993
TARGET_FROZEN: YES（本地不可变 Commit；克隆副本 checkout 后 HEAD/Tree/文件数复核一致）
INDEPENDENT_REVIEW_SESSION: APLS-C04-Independent-Review-v03（DEC-033 授权的干净会话）
FORMAL_REVIEW_RECORD_LOCATION_DEFINED: YES
FORMAL_REVIEW_RECORD: 05_reviews/FORMAL_C04_APLS_0_1_REMEDIATION_REREVIEW_001.md
REVIEW_READINESS: READY
```

逐项核验：复审类型（FORMAL_C04_REREVIEW）、精确不可变 Target（完整 40 位 Hash，可读取可复现）、独立 Session、Review Record 位置预定义均满足；必要输入（上游 Finding 清单与关闭条件、整改工作包、HDP-024 裁决、DEC-031/032/033）齐备；未发现权威 Current Truth 来源之间导致无法确定判定标准的实质冲突。适用 Baseline 仍为 `APLS_BASELINE_NOT_YET_ESTABLISHED_CANDIDATE_PREPARATION`；本复审结论只适用于该精确 Commit。

## 4. 范围、Delta 与继承项清单（DELTA_ONLY）

Delta 确定命令：`git diff --stat 3289037bee1aab64dfa2d58188379a68dcfa601e fd8b59536fcdfdff2f3b199b882c15d97edb1993`（37 文件，+2614/−477）。

实际变化集分组：

1. **代码**：`07_src/crates/apls-compiler/src/cnl_pipeline.rs`（F-01/F-02/F-03/F-04/F-05/F-07 实现 + 12 项定向测试）；`07_src/crates/apls-compiler/src/cnl_lexer.rs`（`lex_and_enumerate` 拆出 `build_edges` + 新增 `pub(crate) lexical_coverage`，属 F-04 证据采集的直接影响闭包；词法候选生成语义不变，公开签名不变）。
2. **设计契约**：`APLS_0.1_CNL_DIAGNOSTICS.md`（E1404/E1405 新增、七码处置标注、A-01/F-05 措辞、状态字段）；`APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md`（§6.4 Rule 直接冲突 + 状态字段）；`APLS_0.1_ZH_CN_GRAMMAR.ebnf`（仅第 3 行状态注释，DEC-033② 授权）；其余 6 份设计文档仅状态字段（F-09）。
3. **需求/产品/架构**：`PRD.md`（PRD-007 验收措辞，Q2=B）；`acceptance_criteria.md`（AC-001～009）；`02_system_requirements/` 五文件（SYS-001～009、NFR-001～006、IF-001～003、FORMAL_TRACE 矩阵）；`system_architecture.md`（§5.6/§6.3/§8 Agent Consumer，Q2=B + DEC-033①）。
4. **测试设计**：`APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md`（CNL-C003/C016 预期修订、新增 CNL-C017～C020）。
5. **治理落盘**：`00_project/ai_context/` 13 文件（HDP-024、DEC-031/032/033 等回执）；`05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md`（上游评审记录随 Anchor 首次入库）；`WP-APLS-C04-REMEDIATION-001.md`（工作包入库）；`07_src/README.md`（F-09 重写）。

继承项（`INHERITED_CLOSED`，未变化且不受 Delta 直接影响的既有结论，源自上游评审第 6 节正面核验）：词法 Lattice 全候选枚举与零最长匹配/概率/LLM 路径；行为句收敛 Gate 的 Canonical Bytes 去重与 E1310 前两类见证截断；`process_candidate` bind→type→normalize 阶段顺序与短路；Transition 文档级 E1402/E1403；IR 发布前复读 + Schema + 跨节点复验与 `units[]` 物化闭包机制本身；CLI 退出码 0/1/2/3、`--version` 固定行、原子发布、符号链接/路径穿越拒绝；资源 Ledger 登记语义与 T0007 Payload 合同；公共诊断 §1.2 总排序键与第 1001 条截断位置；诊断 Envelope Schema 复验。上述各项对应代码/契约文件（`apls_grammar.lalrpop`、`diagnostic.rs`、`resource.rs`、`limits.rs`、`apls-cli` 全部、两个 Schema、Cargo Manifest/Lock）均不在 Delta 内或仅经不受影响的路径被调用。

## 5. 机械验证命令与结果

| 命令 | 执行位置 | 结果 |
|---|---|---|
| `python3 09_quality/traceability/validate_traceability.py` | Target 副本根 | **PASS（exit 0）**：Expected 18 / Covered 18 / Missing 0 / Unexpected 0；Detailed 与 Matrix Edges 各 30、交集 30；Detailed-only/Matrix-only/Duplicate/Invalid 全 0 |
| `cargo test --locked --offline` | Target 副本 `07_src/` | **PASS**：Compiler 36 passed / 0 failed / 1 ignored（Unicode 17.0.0 NormalizationTest，需联网，保持 ignored）；CLI 7 passed / 0 failed / 1 ignored；与 Dispatch 预期一致；12 项新增定向测试（T-F01-1/2、T-F02-1/2、T-F03-1、T-F04-1～4、T-F05-1、T-F07-1 对应实现）全部实际存在且通过（逐名过滤核实） |
| `cargo +1.86.0 check --locked --offline` | Target 副本 `07_src/` | **PASS**（`Finished dev profile`，MSRV 1.86 锁定离线核验通过） |
| CLI 黑盒 F-01：`说明：『unit:% 是内建百分比单位』。` | 副本构建产物 `apls` | `check` Exit 0；`emit-ir --output` Exit 0；输出 IR `units[]` 为空（未注入 `unit:%`） |
| CLI 黑盒 F-02：同条件同行为 `必须` + `禁止` | 同上 | `check` Exit 1 + 唯一 `APLS-E1405`（Primary 第 5 句）；`emit-ir` Exit 1 + E1405，不产出 IR |
| CLI 黑盒 F-07：源状态=目标状态 Transition | 同上 | `check` Exit 1 + 唯一 `APLS-E1404`，Span 指向状态引用短语 |

证据受限项：

- E-01：Compiler 1 项 ignored 用例（Unicode 17.0.0 官方 `NormalizationTest.txt` 一致性）与 CLI 1 项 ignored 用例（T0 资源边界黑盒）未重跑——前者需联网下载（评审禁止），后者沿上游记录为注入型 ignored；两者均不在本次 Delta 内，其既有结论按 `INHERITED_CLOSED` 继承。除上述外本轮无证据受限项（工具链已按 DEC-032 安装，全部机械验证由本评审在 Target 副本上独立执行）。

## 6. 逐 Finding 关闭判定（证据均为本评审在 Target fd8b595 上独立复核）

### F-01（S1）`needs_percent` 子串扫描误判 — **CLOSED**

- 代码核验：`cnl_pipeline.rs:1711-1715` 已删除 `canonical_bytes(v).windows(6)` 子串扫描，改为 `graph.properties` 的 `unit_ref` 精确 JSON 相等 + `frame_references_unit`（行 2621-2631）对 Frame 规范载荷的确定性递归遍历，仅匹配冻结字段 `unit_ref`/`canonical_unit_ref` 的字符串精确相等，不匹配任何 `text` 字段；`needs_time`/`needs_temp` 未动。
- 定向测试：`unit_percent_in_text_does_not_trigger_builtin_injection`（说明文本与文本比较右值双用例、check+emit 双路径、断言 `units[]` 不含 `unit:%`）与 `real_percentage_reference_injects_exactly_unit_percent`（防过杀正向）均通过。
- 黑盒复核：含 `unit:%` 子串的合法说明文本 check/emit-ir 均 Exit 0 且 `units[]` 为空——check 与 emit-ir 行为一致，T0006 误报路径消除。

### F-02（S1）REQUIRE×PROHIBIT 规则冲突无检测 — **CLOSED**

- 裁决核验：HDP-APLS-024 Q1=A（最小直接冲突模型 + 新码 E1405），DEC-031 落盘，与 Dispatch 一致。
- 契约核验：`APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md` §6.4 冻结冲突键 `(canonical_condition_payload_bytes, actor_ref, action_ref, target_ref)` 与 require×prohibit 判定、Span/角色规则，并显式记录 Invariant×Rule、Safety 优先级等更一般冲突 0.1 不定义的边界；验证顺序清单增补"Rule 文档级冲突检查"；诊断目录新增 E1405 行；Conformance 新增 CNL-C018。
- 代码核验：`validate_rule_conflicts`（`cnl_pipeline.rs:1645-1699`）按上述键分组，modality 集合多于一类即发一条 E1405（Primary=稳定排序首句、其余 Related、`missing_or_ambiguous_roles=["modality"]`）；check 路径（行 237）与 emit-ir 路径（行 1710）均调用。`cnl_ast.rs` Modality 仅 `Require`/`Prohibit` 两值，`modalities.len() > 1` 与"两类同时出现"等价，无过杀空间。
- 定向测试：`require_prohibit_direct_conflict_is_rejected`（双路径、唯一码、双 Span、Envelope Schema 复验）与 `non_conflicting_rule_combinations_do_not_report_e1405`（同模态合并、异条件、异行为三组防过杀）均通过。
- 黑盒复核：同条件同行为 必须/禁止 → Exit 1 + E1405（check 与 emit-ir 一致拒绝，无 IR 产出）。

### F-03（S2）候选循环掩盖 APLS-T0007 — **CLOSED**

- 代码核验：`converge_candidates` 行 872 `Err(error) if error.code.starts_with("APLS-T") => return Err(one(error))`——封闭 Tool Failure 码族立即终止整个编译事务，不进入 `failures` 聚合；普通候选淘汰仍走聚合。判据与 `Diagnostic::tool` 构造路径一致，未引入新类型。
- 定向测试：`tool_failure_in_candidate_processing_terminates_the_transaction` 以别名构造 ≥2 候选，注入 Ledger 使后处理候选在 bind 阶段首超限，断言整个事务以单一 T0007 终止（`stage=bind`、`resource=bound_frame_candidates`）、兄弟候选不掩盖；通过。

### F-04（S2）零候选根因聚合未按 §1.1 封闭算法实现 — **CLOSED**

- 结构核验：行为句循环（行 156-206）对每条 Complete Token Stream 收集 Grammar Terminal Finding（`grammar_terminal_findings`，含 UnrecognizedToken 位置 + 引用类别不匹配 E1204 / 条件外并列 E1305 的机械归类）；`parsed` 为空且无 Stream 证据时才回退 `lattice_terminal_findings`；有 Parse 但全部候选淘汰时 `converge_candidates` 聚合各候选首个失败 Stage 的 Terminal Finding（`process_candidate` 短路满足"失败后不运行下游 Stage"）。`aggregate_terminal_findings`（行 3147-3195）实现并集 → 按 `(code,primary_span,related_spans,missing_or_ambiguous_roles,candidate_symbols,payload)` Canonical JSON Byte 去重（BTreeMap，与发现顺序无关）→ 封闭抑制 → 输出；旧 `diagnose_unparsed` 已不存在。
- 抑制表核验（行 3103-3140）：精确实现 §1.1 第 4 条三规则且仅三规则——`E1103..E1106`/`E1201..E1403` 抑制同 Span `E1101`；`E1201/E1204/E1206/E1207` 抑制同 Span `E1203`；`E1401` 抑制同 Span `E1308`；均要求同一 Primary Span。
- 原缺陷逐项核对：E1303 不再硬编码 `["温度","速度"]`，改由 Lattice 首个停滞位置证据产生；词级检测（E1301/E1302/E1304/E1306/E1307）仅在"无 Complete Token Stream"分支作为 §1.1 第 2 条"Lattice/Sentence Validator 直接 Finding"存在，且以 `lexical_coverage`（Lattice Edge 覆盖事实）与引号区间双重机械排除——已声明术语名（如"最高水位"）与固定 Token 内部不再命中；E1203 由聚合对同 Span E1204/E1207 派生并被封闭表抑制（目录发射点标注一致）；E1308 在 Type/Unit 阶段有真实发射点（行 1384，属性声明要求单位而右值为无单位数值）；E1305 由 Grammar Terminal Finding 落在条件外 `并且` Token 产生。
- 七码处置核验（HDP Q3=A）：E1203/E1305/E1308 补实现并标注发射点；E1105/E1106/E1205 标记 `0.1_RESERVED_NOT_EMITTED`；E1309 自目录删除并注明版本边界——目录处置说明（HDP-APLS-024 Q3=A / DEC-031）逐字落实。
- 定向测试：T-F04-1（目录样例句产 E1301+E1303 根因组且顺序无关）、T-F04-2（`“最高水位”` 不误报 E1301）、T-F04-3（封闭抑制表精确性）、T-F04-4（E1308 负向）、E1305 负向均通过；Conformance CNL-C016 预期改写 + 新增 CNL-C019/C020。
- 评审说明：WP 行文称"词表启发式整体删除"，实现保留了经 Lattice 覆盖/引号双重证据防护的词级检测作为上述词法根因身份的直接 Finding 来源。本评审判定该残留与 §1.1 第 2 条及原关闭条件（"实现 §1.1 的封闭根因聚合"）一致：原 Finding 的实质缺陷（无证据诊断凌驾候选证据、样例词表硬编码、命中已声明术语、抑制表不符、死码）均已消除，词级检测的触发边界已由机械证据界定并有定向测试锁定。

### F-05（S2）声明路径 E1310 Witness 不符合冻结机器契约 — **CLOSED**

- 代码核验：新增 `DeclarationWitness`（行 3436-3573）将声明候选确定性映射为声明语义载荷（字段集与 Graph 声明值一致，含派生 `id`，不含 Provenance/Span）；声明收敛（行 107-135）按 `canonical_bytes(witness.payload)` 分组等价类、取字节序前两类；`ambiguity_with`（行 3575-3635）以 `domain_hash("APLS-CNL-FRAME-WITNESS-0.1", canonical_bytes(payload))` 计算 `semantic_fingerprint`（冻结 Preimage），`frame_kind` 取实际声明类别，`differing_roles`/`role_fingerprints` 取真实分歧规范角色（`APLS-CNL-ROLE-WITNESS-0.1` + 角色名 + Canonical 值字节），Primary Span 取分歧 Role Span 最小包含区间、无法定位时回退整句；旧 Rust `Debug` 串 Preimage、硬编码 `entity_declaration` 与 `["meaning"]` 均已删除。
- 诊断目录 §5 措辞同步：等价类排序与见证选择明确含派生 `id` 的完整 Canonical Byte（A-01 一并澄清，见第 7 节），并增补声明路径同规则句。
- 定向测试：`declaration_ambiguity_uses_frozen_witness_contract` 模块级注入两个不等价 Property 声明候选，断言指纹可按冻结 Preimage 独立复算、`frame_kind=property_declaration`、分歧角色与最小 Span 正确、Envelope Schema 复验通过，并验证语义等价声明（秒/毫秒规范 unit_ref 相同）合并为一个等价类；通过。

### F-06（S2）PRD-007 验收不能机械区分 unknown 与 open — **CLOSED**

- 裁决核验：HDP-APLS-024 Q2=B（修订 PRD-007 验收措辞，纯文档变更），DEC-031 落盘。
- 文档核验：`PRD.md` PRD-007 验收改为"IR Schema 机械区分 normative 与 informative；unknown 与 open 内容不得进入 Verified IR，由稳定诊断在编译期拒绝并定位"并注明裁决来源；`system_architecture.md` §5.6、§6.3（unknown/open 行改为"不进入 IR；编译期以稳定诊断拒绝并定位"）、§8 Agent Consumer 同步修订（DEC-033①）；`acceptance_criteria.md` AC-007 与 `SRS.md` SYS-007 措辞一致。无代码、无 Schema、无接受集变化（已核对 IR Schema 不在 Delta 内）。修订后验收措辞与失败关闭设计实际行为一致，验收项可机械判定。

### F-07（S3）源状态等于目标状态复用 E1403 — **CLOSED**

- 契约核验：诊断目录新增 `APLS-E1404`，根因身份"Transition 源状态与目标状态必须彼此不同（`DES-APLS-CNL-FRAME-001` §5.3）"，最小主要范围为状态引用短语；与 E1405 编号无冲突；Conformance 新增 CNL-C017。
- 代码核验：`type_candidate`（行 1081）与 `normalize_candidate`（行 1164）两处发射点均改为 `APLS-E1404`，Span 为两状态引用的覆盖区间；E1403 文档级冲突发射点（`validate_transitions`）未动；`cross_validate` 的 T0006 内部检查如关闭条件豁免未动。
- 定向测试 `transition_with_identical_source_and_target_state_is_e1404` 通过（唯一码、Span 精确）；黑盒复核 Exit 1 + E1404。

### F-08（S2）正式需求追溯层缺失 — **CLOSED**

- 裁决核验：HDP-APLS-024 Q4=A、Q4-a=否，DEC-031 落盘。
- 内容核验：`acceptance_criteria.md` 建立 AC-001～009（提炼自 PRD-001～009 既有验收段，AC-007 随 Q2=B 同步）；`functional_requirements.md` 建立 SYS-001～009（一对一派生，每条含 `Traces-From: PRD-00X / AC-00X`）；`nonfunctional_requirements.md` 建立 NFR-001～006（Traces-From 用最直接相关 P0 ID 并注明源自 PRD §6，与 Q4-a=否一致）；`interface_requirements.md` 建立 IF-001～003（CLI 表面/诊断 Envelope/Verified IR 消费契约）；`SRS.md` 同步登记；`requirements_traceability.md` §2 落盘 30 条 `FORMAL_TRACE` 边并与详细元数据一致。未发现 PRD 未表述的新需求（抽查 SYS/NFR/AC 各行语义均为派生）。
- 机械核验：`validate_traceability.py` **PASS**（18/18 节点、30/30 边、六项错误类全 0）——完成条件达成。

### F-09（S3）受控文档状态与 07_src/README — **CLOSED**

- 逐文件核验：8 份设计文档头部状态均为 `ADOPTED_AS_IMPLEMENTATION_INPUT_BY_DEC-023`；`detailed_design.md` 状态段已改为 TASK-019/TASK-018 完成的当前事实；`07_src/README.md` 整篇重写为 CNL Compiler 垂直切片实现描述，过期表述（`PRE_DEC_014_LEGACY_DSL_PROTOTYPE`、"不是 CNL Compiler 符合性实现"、"未实现命令统一 T0006/Exit 3"）均已删除；全库检索确认上述过期状态串仅残留在工作包对自身修改内容的描述中（可接受）。
- EBNF 第 3 行状态注释修正为 `ADOPTED_AS_IMPLEMENTATION_INPUT_BY_DEC-023`，与 8 份文档措辞一致（DEC-033② 授权范围内）。

## 7. Advisory 顺带核验（不阻断）

- **A-01（已落实）**：诊断目录 E1310 段落明确等价类排序与 Witness 选择使用"不含 Provenance、含派生 `id` 字段的完整 Canonical Semantic Payload Byte 升序"，与实现一致；纯措辞澄清，行为不变。
- **A-02（已落实）**：`07_src/README.md` 设"Legacy DSL 迁移参考（非现行代码）"专节，逐名列出 10 个旧 DSL 文件并声明未挂入模块树。
- A-03/A-04：按工作包仅记录，不需改动；本评审确认其性质说明仍然成立（`expression_depth` 上限当前 Grammar 不可达；CNL-C010 以重复执行代替扰动注入在当前全序确定性实现下充分）。

## 8. 越权检查结果 — **未发现越权**

- Grammar 产生式：`apls_grammar.lalrpop` 不在 Delta 内；`APLS_0.1_ZH_CN_GRAMMAR.ebnf` 仅第 3 行状态注释差异（DEC-033② 显式授权）。
- IR Schema 与 Diagnostic Schema：均不在 Delta 内。
- Cargo Manifest/Lock：均不在 Delta 内（`cargo test --locked --offline` 成功亦佐证 Lock 完整一致）。
- CLI 命令表面/退出码：`apls-cli` crate 不在 Delta 内；黑盒实测 Exit 0/1 行为符合既有契约。
- 资源数值：`limits.rs`、`resource.rs` 不在 Delta 内；T0007 Payload 合同不变（T-F03-1 断言复核）。
- 诊断目录变更范围：仅限 E1404/E1405 新增、七码处置标注（含 E1309 删除说明）、A-01/F-05 §5 措辞与状态字段——未超出授权边界。
- 公共语言接受集：唯一变化为经批准的收窄（E1405 矛盾规则拒绝）与码值归位（E1404），无扩大。
- `cnl_lexer.rs` 改动（`build_edges` 抽取 + `pub(crate) lexical_coverage`）属 F-04 证据采集的直接影响闭包，不改变词法候选生成语义与公开签名，判定为在范围内。

## 9. Target 身份核验（三个时点）

时点 1（评审开始，主仓库）：

```text
git cat-file -t fd8b59536fcdfdff2f3b199b882c15d97edb1993  -> commit
git rev-parse fd8b595...^{tree}                           -> c672fe950c4ef9a75af0fe4430254982a5ab175b
git ls-tree -r --name-only fd8b595... | wc -l             -> 256
```

副本核验：`git clone --no-checkout` 至 `/private/tmp/apls-c04-rereview-fd8b595` 并 checkout 后，`git rev-parse HEAD` = `fd8b59536fcdfdff2f3b199b882c15d97edb1993`，`git rev-parse HEAD^{tree}` = `c672fe950c4ef9a75af0fe4430254982a5ab175b`，`git ls-files | wc -l` = 256，与 Dispatch 期望值全部一致。

时点 2（写入本文件前，主仓库）：上述三条命令重复执行，输出与时点 1 完全一致（commit / c672fe950c4ef9a75af0fe4430254982a5ab175b / 256）。

时点 3（本文件写入后，主仓库）：

```text
git cat-file -t fd8b59536fcdfdff2f3b199b882c15d97edb1993  -> commit
git rev-parse fd8b595...^{tree}                           -> c672fe950c4ef9a75af0fe4430254982a5ab175b
git ls-tree -r --name-only fd8b595... | wc -l             -> 256
```

三个时点输出完全一致，评审期间 Target 未被改动。主仓库工作区 HEAD 之后的未提交治理回执不属于 Target；本评审除本文件外未修改任何项目文件，未执行任何 Git 写操作或远程操作。

## 10. Open Finding 数与 Gate Decision

- Finding 判定：F-01 CLOSED；F-02 CLOSED；F-03 CLOSED；F-04 CLOSED；F-05 CLOSED；F-06 CLOSED；F-07 CLOSED；F-08 CLOSED；F-09 CLOSED。
- Open Findings：**0**（S0：0；S1：0；S2：0；S3：0）
- Advisory：4（A-01/A-02 已落实，A-03/A-04 仅记录；均不阻断）
- 证据受限项：E-01（2 项 ignored 用例未重跑，均不在 Delta 内，既有结论按 INHERITED_CLOSED 继承；不影响下述 Decision 的充分性）

**唯一 Gate Decision：`PASS`**

依据 §38.7.4 机械判定：`REVIEW_READINESS = READY`，全部适用强制检查完成，证据充分，`OPEN_FINDINGS = 0`，无待批准 Exception → `PASS`。

本结论只适用于精确 Commit `fd8b59536fcdfdff2f3b199b882c15d97edb1993`（Tree `c672fe950c4ef9a75af0fe4430254982a5ab175b`，256 文件），不外推到任何后代 Commit 或当前 `HEAD`。Baseline Adoption 仍须另行 HDP 裁决，不随本评审发生。
