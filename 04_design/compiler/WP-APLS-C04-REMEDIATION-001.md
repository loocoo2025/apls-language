# WP-APLS-C04-REMEDIATION-001 — FORMAL_C04_APLS_0_1_CANDIDATE_001 整改工作包候选

- 工作包 ID：`WP-APLS-C04-REMEDIATION-001`
- 状态：`CANDIDATE_AWAITING_HDP_APLS_024`
- 日期：`2026-09-06`
- 责任角色：`C02`（架构与详细设计）
- 物理 Session：`APLS-C02-Architecture-Design-v02`
- 授权依据：`DEC-030`（TASK-024 第一阶段：仅设计，禁止改代码）
- 整改依据（唯一）：`05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md`（`CHANGES_REQUESTED`，9 项 Open Finding + 4 项 Advisory）
- 被评审 Target：Commit `3289037bee1aab64dfa2d58188379a68dcfa601e`（Tree `51edf42da73237bcb3408e234d2cbb2095758fa1`）

> 本文件是**候选**，不是已批准方案。F-02、F-06 的方向必须由项目负责人在 `HDP-APLS-024` 中裁决；在 HDP 批准前，不得修改任何 Compiler/CLI Source、测试、设计、Schema、Grammar、Cargo Manifest/Lock。

---

## 1. 总体整改策略

1. 本工作包逐项覆盖 F-01～F-09，每项给出：根因确认/反驳、最小整改方案、精确位置、契约影响、定向测试、验证方式。
2. 争议裁决项集中在 `HDP-APLS-024`：**F-02**（规则冲突语义方向）、**F-06**（unknown/open 机械表达方向）、**F-04**（目录七码处置的范围确认）、**F-08**（需求层建立的范围确认）。其余 Finding 根因均确认成立，方案为最小实现/文档修正。
3. 建议整改实施分两批（均在 HDP-024 批准后方可动工）：
   - 批次 1（代码+契约）：F-01、F-03、F-04、F-05、F-07，以及按裁决结果的 F-02、F-06；
   - 批次 2（需求追溯+文档货币性）：F-08（C01 主导）、F-09。
4. 全部完成后形成**新的精确 Git Anchor**作为新 Review Target，由全新独立 C04 Session 复审关闭 Finding（步骤见 §7）。
5. 本机无 Rust 工具链，本工作包全部为设计层候选；实施 Session 必须在具备锁定离线工具链的环境执行构建与测试（继承 C04 记录证据受限项 E-01～E-03 的性质说明）。

---

## 2. 逐项整改方案

### F-01（S1）`needs_percent` 子串扫描误判

- **根因确认：成立。** `07_src/crates/apls-compiler/src/cnl_pipeline.rs:1619-1623`，`build_ir` 用 `canonical_bytes(v).windows(6).any(|w| w == b"unit:%")` 扫描全部 Frame 序列化字节。任意合法说明文本（`informative_items[].text`，如 `说明：『unit:% 是内建百分比单位』。`）或文本比较右值均命中子串，注入无人引用的 `unit:%`，随后 Unit Materialization 闭包检查失败，事务以 `APLS-T0006`/Exit 3 结束；而 `needs_time`/`needs_temp` 用精确字段匹配。
- **最小整改方案：** 删除字节子串扫描，改为与另外两个内建单位一致的**精确语义字段判定**：
  - 实现接缝：新增私有辅助函数（建议名 `frame_references_unit(value: &serde_json::Value, unit: &str) -> bool`），对 Frame 规范载荷 JSON 做确定性递归遍历，仅匹配对象字段 `unit_ref`（Property 声明）与 `canonical_unit_ref`（Quantity/Deadline 规范值）的**字符串精确相等**；不匹配任何 `text`/说明字段。`unit_ref`/`canonical_unit_ref` 均为 IR Schema 已冻结字段名（`apls-cnl-ir-0.1.schema.json` 第 282/289、391/397 行区域）。
  - 修改点：`cnl_pipeline.rs:1619-1623` 的 `needs_percent` 表达式替换为：
    `graph.properties.values().any(|v| v["unit_ref"] == "unit:%") || frames.values().flatten().any(|(_, (v, _))| frame_references_unit(v, "unit:%"))`。
  - 不改动 `needs_time`/`needs_temp`，不改动 `builtin_unit` 注入逻辑其余部分。
- **契约影响：** 无。公开语言接受集不窄化（仅修复错误拒绝）；Schema/诊断目录/资源数值/依赖闭包均不变。修复后 `apls check` 与 `apls emit-ir` 对同一 Source 行为一致。
- **定向测试（T0，历史 Bug 回归）：**
  - `T-F01-1`：Source 含 `说明：『unit:% 是内建百分比单位』。`（及一条文本比较右值含 `unit:%` 子串的用例），断言 `check` Exit 0 且 `emit-ir` 成功、`units[]` 闭包精确且**不含** `unit:%`（除非被真实引用）。
  - `T-F01-2`：Source 含真实百分比 Quantity（如 `百分之 5` 对应的 Percentage 右值）或 `unit_ref=unit:%` 的 Property 声明，断言 `unit:%` 被正确注入且闭包检查通过（正向防回归，与 CNL-C007 互补）。
- **验证方式：** 两个定向单元测试 + 现有 Compiler 24 项测试全量回归。

### F-02（S1）REQUIRE×PROHIBIT 规则冲突无检测 —【HDP 裁决项，本工作包不冻结方向】

- **根因确认：成立。** 当前仅 Transition 冲突（E1402/E1403）有检测；同一实体、相同 Canonical Condition 下针对同一 Action 的 `必须` 与 `禁止/不得` 两条 Rule 各自收敛并同时进入 `rules[]` 与 Verified IR。PRD-005 与 `system_architecture.md` §5.5 均要求显式组合否则报错；DECISION_INDEX 无 Descope 记录。
- **候选方案（供 HDP-024 裁决）：**

  **Option A — 0.1 冻结最小直接冲突模型（推荐）。**
  - 语义：两条 Rule 的 Canonical Condition Byte 相等、且 `behavior`（actor_ref/action_ref/target_ref）三元组相等、modality 分属 REQUIRE（`必须`）与 PROHIBIT（`禁止`/`不得`）两类时，构成直接冲突，编译拒绝。
  - 诊断：分配新稳定代码（建议 `APLS-E1405`，见 F-07 编号协调），Payload 含两条冲突 Rule 的匿名 ID 与双方 Sentence Span（Primary Span 取稳定排序后第一条，另一条入 `related_source_spans`，模式同 E1403 现有实现 `cnl_pipeline.rs:1529-1550`）。
  - 文档级检查位置：`build_ir` 前、与 `validate_transitions` 同级的 `validate_rule_conflicts`（复用 `frame_groups`/`rules` 的 Canonical 值分组键：`(canonical_bytes(condition), actor_ref, action_ref, target_ref)` → modality 集合含两类即冲突）。
  - 影响面：诊断目录新增一码（契约变更，需 HDP 显式批准）；`DES-APLS-CNL-SEMVAL-001` 增补冲突规则；Conformance 增补负向样例；**接受集收窄**（此前被错误接受的矛盾输入将被拒绝，失败关闭方向）。Invariant×Rule、Safety 优先级等更一般冲突仍按"未定义即拒绝"原则不在 0.1 定义，但须在设计文档中显式记录此边界。
  - 成本：小。检测是确定性分组比较，无新依赖、无资源数值变化。

  **Option B — 0.1 冻结较完整冲突模型。** 在 A 基础上增加 Invariant×Rule 冲突、Safety（`安全要求：` 前缀 Rule）与普通 Rule 的显式优先级语义。影响面：语义 Profile 与诊断目录多码扩展、Conflict 组合语义需完整形式化、测试面显著扩大；**风险：0.1 阶段成本过高，且 Safety 优先级语义本身是产品级裁决，仓促冻结易错。** 不推荐本次采用。

  **Option C — Descope。** 由 C01 正式修订 PRD-005 验收措辞，将"规则冲突"检测推迟到 0.2，0.1 仅保留 Transition 冲突，并在 PRD/架构中显式记录该缺口。**风险：直接削弱 DEC-004 首要质量目标与 PRD-005 P0 承诺，Baseline 就绪主张受损。** 不推荐。
- **定向测试（按 Option A）：** `T-F02-1` 相同条件+相同行为、modality 一为 `必须` 一为 `禁止` → 拒绝，稳定代码与双 Span；`T-F02-2` 相同 modality/不同条件/不同目标的 Rule 组合**不**误报（防过杀）。
- **验证方式：** 定向测试 + Conformance 负向样例 + 新 C04 复核。

### F-03（S2）候选循环掩盖 APLS-T0007

- **根因确认：成立。** `cnl_pipeline.rs:208-229`：`Err(error) => failures.push(error)` 把 `process_candidate`（行 841-878，内部三次 `ledger.add` 可产生 T0007）返回的 Tool Failure 与普通候选淘汰同等待遇；兄弟候选收敛即静默丢弃。违反 `DES-APLS-CNL-RESOURCE-001` §7"资源耗尽立即终止当前编译事务"与 `WP-APLS-CNL-C03-001` §3。
- **最小整改方案：** 候选循环区分两类错误，改动仅一处：
  - 修改点：`cnl_pipeline.rs:227` 的 `Err` 分支改为——若 `error` 属于封闭 Tool Failure 码族（机械判据：`error.code` 以 `"APLS-T"` 为前缀；0.1 内即 T0006/T0007，与 `Diagnostic::tool` 构造路径一致），立即 `return Err(one(error))` 终止整个编译事务，不进入 `failures` 聚合；否则维持 `failures.push(error)`。
  - 该判据不引入新类型、不改变 `Diagnostic` 结构；Tool 码族前缀是现有公共契约（Envelope `status=tool_failure`）的天然机械边界。
- **契约影响：** 无公共契约变化；仅是内部错误分类语义与已批准合同对齐。资源数值、退出码、Envelope 不变。
- **定向测试（T0，故障注入、私有 `#[cfg(test)]`，沿用 CNL-C012B 既有注入模式）：**
  - `T-F03-1`：构造一句存在两个 Parse 候选的输入（模块级），注入 Ledger 使**后处理候选**在 Bind/Type/Normalize 某 Stage 首次超限产生 T0007，而另一候选可正常收敛；断言整个编译事务以单一 T0007 终止、不产出 Frame/IR，且失败聚合中不出现该 T0007 被掩盖的行为。
- **验证方式：** 注入测试 + 现有资源边界测试（含 CNL-C012A/B/C 对应用例）回归。

### F-04（S2）零候选根因聚合未按 §1.1 封闭算法实现 —【七码处置需 HDP 确认范围】

- **根因确认：成立。** `diagnose_unparsed`（`cnl_pipeline.rs:3008-3235`）不使用任何候选失败证据，对句子文本做词表子串扫描：`E1303` 仅对硬编码 `["温度","速度"]` 触发；`E1301` 的 `高/低` 子串可命中已声明术语名内部；`E1302/E1304/E1306/E1307` 同为词表匹配。`suppress_derived_diagnostics`（行 2994-3007）的抑制规则与 §1.1 封闭表不一致（实现中任意具体诊断抑制同 Span 的 E1308；合同只允许 E1401 抑制 E1308）。`E1105/E1106/E1203/E1205/E1305/E1308/E1309` 在生产代码中从不产生（已用全库检索核实：`cnl_pipeline.rs` 中 E1308 仅出现在抑制函数匹配表，无发射点；E1105/E1106/E1203/E1205 无任何出现）。
- **最小整改方案（主方案 A，推荐）：按 §1.1 实现封闭根因聚合。**
  - 证据采集：改造行为句处理段（`cnl_pipeline.rs:152-242`）——每条 Complete Token Stream 的 Grammar 失败记录为 Grammar Terminal Finding（首个无法继续位置）；`parsed` 非空时，`process_candidate` 失败即该候选路径首个失败 Stage 的 Terminal Finding（现有 bind→type→normalize 短路返回已满足"失败后不运行下游 Stage"）；无 Complete Token Stream 时使用 Lattice/Sentence Validator 直接 Finding。
  - 聚合：零候选时，对全部 Terminal Finding 取并集 → 按 `(code,primary_span,related_spans,missing_or_ambiguous_roles,candidate_symbols,payload)` Canonical JSON Byte 去重 → 应用 §1.1 第 4 条**封闭抑制表**（重写 `suppress_derived_diagnostics`：具体 `E1103/E1104/E1105/E1106/E1201..E1403` 抑制同 Primary Span 的 `E1101`；`E1201/E1204/E1206/E1207` 抑制同 Span `E1203`；`E1401` 抑制同 Span `E1308`；无其他规则）→ 按 §1.2 总排序输出。
  - `diagnose_unparsed` 的词表启发式整体删除，由证据驱动路径取代。
  - **七码处置（需 HDP-024 确认，逐项建议）：**
    | 代码 | 建议处置 | 理由 |
    |---|---|---|
    | E1308（数值缺唯一类型/必需单位） | **补实现 + 负向样例** | 聚合实现后，Type/Unit Stage 对无单位数值的 Terminal Finding 即自然落在该码；属当前 Grammar 可达根因 |
    | E1203（引用候选无法继续绑定） | **补实现**（绑定阶段零候选继续路径的根因归码）或随聚合证据自然产生 | 目录抑制链依赖其存在 |
    | E1305（不支持的否定/并列作用域） | **补负向样例**（Grammar 拒绝的否定/并列结构句，根因归码由聚合证据映射） | 词表扫描删除后须有证据来源 |
    | E1309（跨句省略/上下文依赖） | **补负向样例**或**删除**（0.1 Grammar 下该根因是否可由候选失败证据机械识别，需实施时验证；若不可识别则申请从目录删除并注明版本边界） | 避免目录承诺超过实现能力 |
    | E1105（英文/混合句式）、E1106（多主 Frame/并列行为）、E1205（未声明同义词/近似名） | **建议删除或在目录中标记 0.1 不产生**（标记方式：`0.1_RESERVED_NOT_EMITTED`，根因身份保留不复用） | 当前 Grammar/绑定规则下无对应 Terminal Finding 来源；保留空承诺即重蹈本 Finding |
  - 备选方案 B：**修订诊断目录以匹配实现能力**（删除 §1.1 聚合要求，将词表启发式显式冻结为合同）。**不推荐**：违背 PRD-006 与目录自身"根因身份不得漂移"的立场，且词表方案已对样例外输入证明不可靠。若负责人选择 B，属契约降级裁决，须记录后果。
- **契约影响：** 主方案 A 下——诊断目录文本不变（实现补齐合同），若删除/标记五码则目录变更为契约变更，需 HDP 显式列出；`suppress_derived_diagnostics` 行为变化影响公共诊断输出内容（同一非法 Source 的根因码集合可能变化，属缺陷修复方向）。Schema 不变。
- **定向测试（T0/T1，沿用并扩展 CNL-C010）：**
  - `T-F04-1`：`温度高的时候适当降低一点速度。`（目录 §6 样例）在聚合实现下产生 E1301+E1303 根因组且顺序无关（CNL-C010 扩展）。
  - `T-F04-2`：已声明术语名含 `高`（如 `“最高水位”`）的非法句**不**误报 E1301 于该术语 Span。
  - `T-F04-3`：封闭抑制表定向用例——同 Span 具体码抑制 E1101；E1401 抑制 E1308；非 E1401 不抑制 E1308。
  - `T-F04-4`：E1308 负向样例（若采纳补实现）。
- **验证方式：** 定向测试 + Conformance 同步 + 新 C04 复核。

### F-05（S2）声明路径 E1310 Witness 不符合冻结机器契约

- **根因确认：成立。** 声明句多候选路径（`cnl_pipeline.rs:127-133`）调用 `ambiguity()`（行 3236-3257）：Witness 指纹 Preimage 为 Rust `Debug` 串（`format!("{v:?}")`，含 Span、rustc 相关），违反 `DES-APLS-CNL-DIAG-001` §5 冻结 Preimage（`APLS-CNL-FRAME-WITNESS-0.1` + Canonical Payload Bytes）；`frame_kind` 硬编码 `entity_declaration`；`differing_roles` 硬编码 `["meaning"]`（`meaning` 不是规范 Role 名）；Primary Span 为整句。
- **最小整改方案（主方案 A，推荐）：按冻结 Preimage 改造声明路径。**
  - 声明候选 Canonical 化接缝：声明 AST（`Declaration`）尚无规范 JSON 表示；新增私有纯函数将声明候选确定性地映射为其**声明语义载荷**（与 `build_graph` 产出的 Graph 声明值同一字段集：kind、显示名 NFC、类别特有角色值——Property 的 value_type/unit_ref、State 的所属实体等；不含 Provenance/Span），经 `canonical_bytes` 得 Canonical Payload Bytes。等价类分组键改用该 Canonical Bytes（替代当前 `BTreeSet<Declaration>` 的派生序），与行为句路径的收敛 Gate 语义对齐。
  - Witness：`semantic_fingerprint = sha256(APLS-CNL-FRAME-WITNESS-0.1 + canonical_payload_bytes)`；`frame_kind` 取实际声明类别（`entity_declaration`/`unit_declaration`/`property_declaration`/`action_declaration`/`event_declaration`/`alias_declaration`/`state_declaration`，均在 Schema `ambiguityWitness.frame_kind` 枚举内）；`role_fingerprints`/`differing_roles` 取两见证间值不同的**真实规范角色名**（如 `display_name`/`value_type`/`unit_ref`，按 Role 名字节升序）；Primary Span 取最小分歧范围——声明句的 Role 级 Provenance Span 需随上述映射一并机械生成（候选角色的 Source Span），取分歧 Role Span 的最小包含区间；无法定位时回退整句并在设计中注明（回退不得成为常态）。
  - `ambiguity()` 函数签名改为接收 Canonical 化后的见证结构，与 `frame_ambiguity()`（行 3259-3312，已是正确实现）复用同一 Witness 构造逻辑。
- **备选方案 B：** 按关闭条件第二种允许，将该防御路径的根因与 Payload 合同**显式冻结为另一受批准形态**（如声明路径 E1310 使用独立 Payload 类型并在目录/Schema 中显式定义）。不推荐：同一 E1310 两种机器形态增加消费复杂度；仅当主方案实施成本被证明过高时由 HDP 选择。
- **契约影响：** 无 Schema 变更（现有枚举已覆盖）；诊断目录 §5 措辞不变（实现补齐合同）；等价类排序键变化属内部实现（A-01 措辞澄清一并处理，见 §3）。
- **定向测试（T1，模块级注入——公开 Source 在当前 Grammar 下难以触发声明多候选）：**
  - `T-F05-1`：模块级构造两个不等价声明候选（如同名不同 `value_type` 的 Property 声明）注入收敛 Gate；断言 `frame_kind=property_declaration`、指纹可对冻结 Preimage 独立复算、`differing_roles` 为真实分歧角色、Primary Span 为最小分歧范围。
- **验证方式：** 注入测试 + Envelope Schema 复验 + 新 C04 复核。

### F-06（S2）PRD-007 验收：IR Schema 不能机械区分 unknown 与 open —【HDP 裁决项】

- **根因确认：成立。** `apls-cnl-ir-0.1.schema.json` 仅 `informative_items[].classification = "informative"`（第 531-534 行区域）+ 规范节点隐含分区；无 `unknown`/`open` 机械标记；无 Descope 决定。
- **候选方案（供 HDP-024 裁决）：**

  **Option A — Schema 增加 unknown/open 机械表达。** 在 IR Schema/契约中增加机械类别（如 `knowledge_items[]`/`open_items[]` 数组，或将 `classification` 枚举扩展为 `informative|unknown|open`）。**关键后果：失败关闭设计下 unknown/open 永远无法进入 Verified IR，新增类别恒为空数组——机械区分成立但语义上空转**；同时 `system_architecture.md` §6.3 的四类表与 §5.6"自由自然语言只进入 informative/rationale/open/unknown 字段"的措辞与失败关闭现实的张力需要一并梳理。契约变更（Schema 变更）需精确批准；实现与 Cross-validation、Conformance 同步。
  
  **Option B — 修订 PRD-007 验收措辞（推荐）。** 由 C01 将验收改为与失败关闭设计一致的形式，例如："IR Schema 机械区分 normative 与 informative；unknown 与 open 内容不得进入 Verified IR，由稳定诊断在编译期拒绝并定位"。同步修订 `system_architecture.md` §6.3（unknown/open 行改为"编译期拒绝，不进入 IR"）与 §5.6 相关措辞。**后果：PRD/架构文档变更（C01/C02 有权层 + HDP 批准）；无代码、无 Schema、无接受集变化；产品行为不变。**
  
  **Option C — Descope 到 0.2。** PRD-007 增加版本边界注记，unknown/open 机械表达推迟。实质同 B 但保留验收原文挂起；不推荐（挂起的 P0 验收阻碍 Baseline 就绪判定）。
- **推荐 Option B**：与既有失败关闭架构一致，成本最小，不制造空转契约。
- **定向测试：** Option B 无需新代码测试（文档裁决 + Conformance 措辞核对）；Option A 需 Schema 正负向样例各一。
- **验证方式：** 按所选方案由 C01/C02 修订文档或 Schema，新 C04 复核验收措辞与实际契约一致。

### F-07（S3）源状态等于目标状态复用 E1403

- **根因确认：成立。** `type_candidate`（`cnl_pipeline.rs:1058-1066`）与 `normalize_candidate`（行 1141-1149）对 `source_state.id == target_state.id` 发 `APLS-E1403`；而目录中 E1403 根因身份是"相同实体、源状态与规范 Trigger 要求进入多个不同目标状态"（文档级冲突，正确发射点为 `validate_transitions` 行 1529-1550）。"两状态必须彼此不同"是 `DES-APLS-CNL-FRAME-001` §5.3 的独立 Frame 契约（第 191 行），目录无对应码。
- **最小整改方案：** 诊断目录**新增稳定码** `APLS-E1404`，根因身份："Transition 源状态与目标状态必须彼此不同（`DES-APLS-CNL-FRAME-001` §5.3）"，最小主要范围为该 Transition 句的状态引用短语。实现侧将上述两处发射点的码改为 `APLS-E1404`（消息不变）。
  - **编号协调：** 若 F-02 Option A 获批，规则冲突码建议取 `APLS-E1405`，避免与本码冲突；两码的目录扩展同属一次契约变更，一并在 HDP-024 批准。
  - 复验路径说明：`cross_validate` 的 T0006 内部检查不受影响（Finding 关闭条件已豁免）。
- **契约影响：** 诊断目录扩展一码（契约变更，需 HDP 显式列出）；诊断 Schema 无需变更（code 为通用字符串模式）；公共诊断输出内容变化属缺陷修复方向（根因身份归位）。
- **定向测试（T1，负向样例）：** `T-F07-1`：`当…时，“设备”从“运行”进入“运行”。` → 拒绝，唯一码 `APLS-E1404`，Span 指向状态引用；既有 E1403 文档级冲突测试（`cnl_pipeline.rs:3812` 区域）回归不变。
- **验证方式：** 定向测试 + 目录/实现一致性核对 + 新 C04 复核。

### F-08（S2）正式需求追溯层缺失 —【需求层范围需 HDP 确认】

- **根因确认：成立。** `02_system_requirements/` 四文件全为模板占位（含 `<SOURCE_ID>`），`01_product_requirements/acceptance_criteria.md` 仅一行空 DRAFT；`validate_traceability.py` FAIL（Missing SYS-001/NFR-001/IF-001，双边集为 0）。
- **最小整改方案（仅派生，不发明新需求；主导角色 C01，C02 配合 IF 层）：**
  1. **AC 层**：将 PRD-001～009 各条既有"验收"段落原义提炼为 `AC-001`～`AC-009`，写入 `01_product_requirements/acceptance_criteria.md`（AC ID 属校验脚本合法 Source 前缀，落盘后自动满足 `declared_product_ids` 检查）。F-06 若选 Option B，AC-007 措辞随裁决同步。
  2. **SYS 层**：从 PRD-001～009 一对一派生 `SYS-001`～`SYS-009`（如 SYS-001←PRD-001 最终规范语义唯一、SYS-005←PRD-005 语义冲突检测、SYS-008←PRD-008 编译器式最小工作流），写入 `02_system_requirements/functional_requirements.md`，每条含 `Traces-From（正式上游追溯）：PRD-00X`、`AC-00X`。`SRS.md` 同步登记。
  3. **NFR 层**：从 PRD §6 六条非功能要求派生 `NFR-001`～`NFR-006`（确定性/可复现/可演进/可诊断/可移植/可测试），写入 `nonfunctional_requirements.md` 表格，Traces-From 列填对应 PRD（NFR 来源统一为 `PRD-001～009` 中最直接者；PRD §6 无独立编号，建议 Traces-From 使用最接近的 P0 需求 ID 并在 NFR 行注明源自 PRD §6——**此点是否需要 PRD §6 先补编号，列为 HDP-024 需求层裁决子项**）。
  4. **IF 层**：派生 `IF-001`（CLI 命令表面/退出码/stdout 边界 ← PRD-008）、`IF-002`（Diagnostic Envelope 机器契约 ← PRD-006）、`IF-003`（Verified IR `apls-cnl-ir-0.1` 消费契约 ← PRD-007），写入 `interface_requirements.md`。
  5. **矩阵落盘**：`requirements_traceability.md` §2 填写全部 `FORMAL_TRACE` 边（PRD/AC → SYS/NFR/IF），与详细元数据逐边一致；§3 下游追溯表填架构/设计/代码/测试映射（DES-APLS-*、`07_src/crates/*`、CNL-C001～C016）。
  6. 机械验证：`python3 09_quality/traceability/validate_traceability.py` 必须 PASS（Missing/Unexpected/Detailed-only/Matrix-only/Duplicate/Invalid 全 0）。
- **契约影响：** 不触碰代码/Schema/语言契约；属需求基线层建立（新事实落盘）。按 `PROJECT_ASSURANCE_CADENCE_POLICY.md` 第 4 节，此动作本身触发 Traceability Gate 必须通过——本方案即以 PASS 为完成条件。
- **定向测试：** 无需代码测试；验证即脚本 PASS + 逐边人工核对记录。
- **范围控制：** 以上 ID 数量（9 SYS + 6 NFR + 3 IF + 9 AC）为覆盖 PRD-001～009 与 PRD §6 的**最小完备集**；不新增任何 PRD 未表述的需求。若负责人认为需求分层需更完整建模（如 CON 层、SCN 层），超出本整改最小范围，应另立任务。

### F-09（S3）受控文档状态字段与 07_src/README 修正

- **根因确认：成立。** 逐文件核实如下，修正清单精确到字段：
  1. `07_src/README.md`：**整篇重写**为当前 CNL 实现描述——删除 `PRE_DEC_014_LEGACY_DSL_PROTOTYPE` 方向状态与"不是 CNL Compiler 符合性实现""已声明但未实现的命令统一以 APLS-T0006 / Exit 3 失败"等过期表述；改写为：本目录是 `DEC-023/DEC-024` 批准契约的 CNL Compiler 垂直切片实现（`apls-compiler` CNL Pipeline + `apls-cli` 已批准命令表面）；`src/` 下旧 DSL 文件（`ast.rs/lexer.rs/parser.rs/resolve.rs/source.rs/index.rs/canonical.rs/ir.rs/semantic.rs/types.rs`）标注为未挂入模块树的 **Legacy DSL 迁移参考**（`ARCH-APLS-CNL-001` §8，同 A-02）；最小验证命令段保留。
  2. 以下 8 份文档头部状态字段 `TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018` → 按 DEC-023 事实改为"已被 DEC-023（HDP-APLS-018 Option A）采用为实现输入"的现行状态（精确措辞建议 `ADOPTED_AS_IMPLEMENTATION_INPUT_BY_DEC-023`，最终措辞由文档事实 Owner 规则确认）：
     - `04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md`
     - `04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md`
     - `04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md`
     - `04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md`（注：F-04/F-05/F-07 获批整改时本文件另有内容变更，状态修正与其合并一次提交）
     - `04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md`
     - `04_design/compiler/APLS_0.1_CNL_RESOURCE_AND_ORDERING_PROFILE.md`
     - `04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md`
     - `04_design/compiler/APLS_0.1_CNL_FRONTEND_ADAPTATION_DESIGN.md`
  3. `04_design/detailed_design.md`：头部状态 `TASK019_THIRD_REMEDIATION_CANDIDATE_AWAITING_THIRD_REREVIEW` 与"原 CNL 实现包继续阻塞"表述 → 按 DEC-023/DEC-024 改为 TASK-019 已完成、TASK-018 已实施的当前事实。
- **契约影响：** 仅文档状态/自述货币性修正，不改变任何文档的规范性内容（诊断目录与 IR 文档的**内容**变更只属于 F-04/F-05/F-07 整改）。
- **定向测试：** 无代码测试；验证为逐文件 diff 核对 + 新 C04 复核。
- **验证方式：** 上述清单逐项核对。

---

## 3. Advisory 处置（A-01～A-04）

| Advisory | 处置 | 说明 |
|---|---|---|
| A-01（等价类排序键是否含 `id` 的措辞歧义） | **纳入本次整改** | 随 F-05 一并处理：在诊断目录 §5 明确等价类排序与 Witness 选择的排序键是否包含 `id` 字段，使措辞与实现（含 `id` 的完整 Canonical Bytes）一致；纯措辞澄清，不改变实现行为 |
| A-02（旧 DSL 文件身份标注） | **纳入本次整改** | 随 F-09 第 1 项在 `07_src/README.md` 中一并标注 Legacy 参考身份 |
| A-03（`expression_depth` 上限当前不可达） | **仅记录** | 不构成缺陷；Grammar 扩展时复核，不在本次改动 |
| A-04（CNL-C010 以重复执行代替扰动注入） | **仅记录** | 当前实现下证据充分；不新增测试 |

---

## 4. 整改精确文件清单与修改边界

**代码（批次 1，HDP 批准后由 C03 执行）：**
- `07_src/crates/apls-compiler/src/cnl_pipeline.rs`：F-01（行 1619-1623 + 新增私有辅助函数）、F-03（行 227 分支）、F-04（行 152-242 证据采集、`diagnose_unparsed` 行 3008-3235 重写、`suppress_derived_diagnostics` 行 2994-3007 重写）、F-05（行 126-134、行 3236-3257 重写 + 声明 Canonical 化接缝）、F-07（行 1058-1066、1141-1149 码值）、F-02 Option A（新增 `validate_rule_conflicts` 及调用点）。
- `07_src/crates/apls-compiler/src/` 内 `#[cfg(test)]`：§6 定向测试。
- **不改动**：`diagnostic.rs`（结构不变）、`resource.rs`、`limits.rs`、Grammar（`apls_grammar.lalrpop`）、CLI crate、Cargo.toml/Cargo.lock。

**设计契约（批次 1，C02）：**
- `04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md`：F-07 新增 E1404；F-02 Option A 新增 E1405；F-04 七码处置（补实现说明/删除/标记）；F-05/A-01 §5 措辞澄清；状态字段（F-09）。
- `04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md`：F-02 Option A 冲突规则条目。
- `04_design/ir/apls-cnl-ir-0.1.schema.json` + `APLS_0.1_CNL_CANONICAL_IR.md`：**仅当 F-06 选 Option A**。
- `01_product_requirements/PRD.md`、`03_architecture/system_architecture.md`（§5.6/§6.3）：**仅当 F-06 选 Option B/C 或 F-02 选 Option C**（C01 主导）。

**需求追溯（批次 2，C01 主导）：**
- `01_product_requirements/acceptance_criteria.md`、`02_system_requirements/`（SRS.md、functional_requirements.md、nonfunctional_requirements.md、interface_requirements.md、requirements_traceability.md）。

**文档货币性（批次 2）：** F-09 清单所列 10 个文件。

**测试设计同步：** `06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md` 增补/修订对应条目（F-02/F-04/F-07 负向样例登记）。

---

## 5. 什么不得改变（除非 HDP-024 显式批准）

1. 公开语言接受集不因整改**扩大**；唯二接受集变化是收窄方向且必须经 HDP 显式批准：F-02 Option A（矛盾规则由错误接受改为拒绝）、F-07（码值归位，拒绝行为本身不变）。
2. 资源数值上限（`DEC-020` 固定值）、Token Kind Rank、节点 Taxonomy、T0007 Payload 合同：不变。
3. 依赖闭包、版本、Checksum、Cargo Manifest/Lock：不变。
4. CLI 命令表面、退出码 0/1/2/3、`--version` 固定行：不变。
5. IR Schema、Diagnostic Schema：除 F-06 Option A 经批准外不变。
6. Grammar（`apls_grammar.lalrpop`/`APLS_0.1_ZH_CN_GRAMMAR.ebnf`）：不变。
7. Git：整改实施前不得 Commit/Push/Tag；新 Anchor 的创建须单独授权（参照 DEC-026 模式）。

---

## 6. 拟新增定向测试清单（最小测试原则）

| 测试 ID | 对应 Finding | 级别 | 来源 | 类型 |
|---|---|---|---|---|
| T-F01-1 | F-01 | T0 | 历史 Bug（C04 实证） | 单元（公共 Source，check+emit-ir 双路径） |
| T-F01-2 | F-01 | T0 | 历史 Bug 防过杀回归 | 单元 |
| T-F02-1 / T-F02-2 | F-02（若 Option A） | T0 | PRD-005 P0 需求 | 单元负向 + 防过杀 |
| T-F03-1 | F-03 | T0 | 已批准资源合同违规（防御路径） | 模块级 Ledger 注入（`#[cfg(test)]` 私有） |
| T-F04-1～T-F04-4 | F-04 | T0/T1 | PRD-006 + 目录 §1.1 冻结合同 | 单元（含 CNL-C010 扩展） |
| T-F05-1 | F-05 | T1 | 诊断机器契约（防御路径，公开 Source 不可达） | 模块级注入 |
| T-F07-1 | F-07 | T1 | 目录根因身份合同 | 单元负向 |
| F-06 | — | — | Option A 时 Schema 正负向各一；Option B 无新测试 | — |
| F-08 | — | — | 机械门禁 PASS 即验证，无代码测试 | — |
| F-09 | — | — | 文档 diff 核对，无测试 | — |

合计约 10 个高价值定向用例，无 T2/T3，无组合爆炸，符合 `AI_TESTING_GOVERNANCE_RULES.md` 最小充分原则。

---

## 7. 整改后新 Review Target 与 Finding 关闭复审步骤

1. HDP-024 批准 → C00 按 DEC-030 后续授权 Dispatch 批次 1（C02 契约同步 + C03 实现）与批次 2（C01 需求层 + 文档）。
2. 每批完成后执行适用验证：批次 1 为锁定离线构建 + 全量单元测试 + 定向测试；批次 2 为 `validate_traceability.py` PASS + 文档 diff 核对。本机无工具链的环境限制须在实施 Dispatch 中显式处理（继承 E-01 性质，构建/测试证据由具备工具链的环境产生）。
3. 全部批次完成后，参照 DEC-026 模式经单独授权创建**新的精确本地 Commit**作为新 Review Target（记录 Commit/Tree/文件数）。
4. C00 发起 Finding 关闭复审：全新独立 C04 Session、只读新 Target；范围按 `DEC-022` 默认 `DELTA_ONLY`（本轮改动 + 直接影响闭包 + 最小 Target 完整性证据），但 F-01～F-09 九条关闭条件逐条核验为必查项；首次正式 C04 已建立的事实按 `INHERITED_CLOSED` 继承。
5. 新 C04 逐条判定 Finding 关闭/保持 Open；全部关闭且 Gate = PASS 后，Baseline Adoption 仍须另行 HDP 裁决，不自动发生。

---

## 8. 争议项声明

本工作包对 F-01～F-09 的**根因均确认成立，无反驳项**。无 `RULE_NOT_FOUND / RULE_CONFLICT` 阻断。需项目负责人裁决的事项集中于 `HDP-APLS-024`：F-02 方向、F-06 方向、F-04 七码处置范围、F-08 需求层范围（含 PRD §6 编号子项）、F-09 状态字段精确措辞确认。
