# HDP-APLS-024 — FORMAL_C04_APLS_0_1_CANDIDATE_001 整改工作包批准与关键方向裁决

```yaml
determination_id: HDP-APLS-024
status: APPROVED
selected_option: "WORK_PACKAGE: WP-APLS-C04-REMEDIATION-001；Q1: A；Q2: B；Q3: A；Q4: A；Q4-a: NO"
decision_owner: HUMAN_PROJECT_OWNER
prepared_date: 2026-09-06
approved_date: 2026-09-06
```

## 1. 必须决定什么

是否批准 `04_design/compiler/WP-APLS-C04-REMEDIATION-001.md`（整改工作包候选）作为首次正式 C04（`FORMAL_C04_APLS_0_1_CANDIDATE_001`，`CHANGES_REQUESTED`，9 项 Open Finding）的整改依据，并对其中四个必须由有权层级裁决的方向作出选择：

- **Q1（F-02，S1）**：PRD-005 规则冲突语义在 0.1 的方向；
- **Q2（F-06，S2）**：PRD-007 验收中 unknown/open 机械区分的方向；
- **Q3（F-04，S2）**：零候选根因聚合按目录 §1.1 补齐实现，以及目录中从不产生的七个代码（E1105/E1106/E1203/E1205/E1305/E1308/E1309）的处置范围；
- **Q4（F-08，S2）**：建立正式 SYS/NFR/IF/AC 需求层并落盘追溯边的最小范围（含 PRD §6 是否先补编号的子项）。

F-01、F-03、F-05、F-07、F-09 根因均确认成立且方案为最小实现/文档修正，随工作包整体批准即生效，无需单独方向裁决（F-05 备选方案 B 仅在主方案被证明成本过高时才需另行裁决）。

## 2. 确认事实

- 首次正式 C04 已完成：`FORMAL_C04_APLS_0_1_CANDIDATE_001 = CHANGES_REQUESTED`；Open Findings 9（S1：F-01、F-02；S2：F-03、F-04、F-05、F-06、F-08；S3：F-07、F-09）；Advisory 4（A-01～A-04，不阻断）；证据受限项 E-01～E-03（本机无 Rust 工具链等，不构成 Finding）。
- Review Target 为不可变 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e`，三个时点核验一致，评审期间未被改动。
- `DEC-030` 已授权 C00 组织整改：由全新 C02 Session 形成整改工作包候选与本 HDP；**本授权不含代码修改**。
- 工作包候选 `WP-APLS-C04-REMEDIATION-001` 已由 C02 Session `APLS-C02-Architecture-Design-v02` 形成，逐项覆盖 F-01～F-09 与 A-01～A-04，无 Finding 反驳项。
- 当前产品 Baseline 未建立；Baseline Adoption 不随整改或复审自动发生。
- `validate_traceability.py` 当前 FAIL（模板占位），按保障节奏该门对本 C04 Gate 不构成机械阻断，但在 Baseline Adoption 前必须关闭（F-08）。

## 3. 待裁决选项

### Q1 — F-02：规则冲突语义方向

- **Option A（推荐）**：0.1 冻结最小直接冲突模型——相同 Canonical Condition + 相同行为三元组（actor/action/target）的 REQUIRE×PROHIBIT 直接冲突拒绝，分配新稳定诊断码 `APLS-E1405`；Invariant×Rule、Safety 优先级等更一般冲突 0.1 不定义，按既有"未定义即拒绝"原则处理并显式记录边界。
  - 后果：满足 PRD-005 验收与 `system_architecture.md` §5.5；接受集收窄（矛盾输入由错误接受改为拒绝，失败关闭方向）；诊断目录扩展一码（契约变更，随本 HDP 批准）。
- **Option B**：冻结较完整冲突模型（含 Invariant×Rule、Safety 显式优先级）。
  - 后果：语义形式化与测试面显著扩大；Safety 优先级本身是产品级裁决，0.1 阶段仓促冻结风险高。
- **Option C**：Descope——C01 修订 PRD-005 验收措辞，规则冲突检测推迟到 0.2。
  - 后果：削弱 DEC-004 首要质量目标与 P0 承诺；Baseline 就绪主张受损。

### Q2 — F-06：unknown/open 机械区分方向

- **Option A**：IR Schema 增加 unknown/open 机械表达（如新数组或 classification 枚举扩展）。
  - 后果：Schema 契约变更；失败关闭设计下该两类永远无法进入 Verified IR，新增类别恒为空数组（机械区分成立但语义空转）；需同步梳理 `system_architecture.md` §5.6/§6.3 措辞；实现、Cross-validation、Conformance 同步。
- **Option B（推荐）**：C01 修订 PRD-007 验收措辞，与失败关闭设计一致（IR 机械区分 normative/informative；unknown/open 编译期以稳定诊断拒绝、不进入 IR）；同步修订 `system_architecture.md` §6.3/§5.6。
  - 后果：仅文档变更，无代码/Schema/接受集变化；产品行为不变。
- **Option C**：Descope 到 0.2（验收原文挂起 + 版本边界注记）。
  - 后果：P0 验收挂起，阻碍 Baseline 就绪判定。

### Q3 — F-04：根因聚合与七码处置

- **Option A（推荐）**：实现目录 §1.1 封闭根因聚合（候选路径首个失败 Stage Terminal Finding 并集、Canonical JSON 去重、封闭抑制表、§1.2 总排序），删除词表启发式；七码处置按工作包 §2 F-04 建议表：E1308/E1203 补实现，E1305/E1309 补负向样例（不可机械识别者申请删除并注明版本边界），E1105/E1106/E1205 标记 `0.1_RESERVED_NOT_EMITTED`（根因身份保留不复用）。
  - 后果：诊断目录内容变更（七码处置部分属契约变更，随本 HDP 批准）；同一非法 Source 的根因码集合可能变化（缺陷修复方向）；公共诊断可靠性对样例外输入成立。
- **Option B**：修订目录以匹配实现能力（删除 §1.1 聚合要求，词表启发式显式冻结为合同）。
  - 后果：契约降级；违背 PRD-006 与"根因身份不得漂移"立场；词表方案已被 C04 证明对样例外输入不可靠。不推荐。

### Q4 — F-08：需求追溯层最小范围

- **Option A（推荐）**：按工作包 §2 F-08 建立最小完备集——AC-001～009（提炼 PRD 既有验收段）、SYS-001～009（一对一派生自 PRD-001～009）、NFR-001～006（派生自 PRD §6）、IF-001～003（CLI/Diagnostic Envelope/Verified IR 消费契约），落盘 `Traces-From` 与 `requirements_traceability.md` 的 `FORMAL_TRACE` 边，`validate_traceability.py` PASS 为完成条件；不发明任何 PRD 未表述的新需求。
  - 子项 Q4-a：NFR 的 Traces-From 是否要求先为 PRD §6 六条非功能要求补独立编号（是 / 否，选"否"则 NFR 行 Traces-From 使用最直接相关 P0 需求 ID 并注明源自 PRD §6）。
  - 后果：需求基线层新事实落盘；触发 Traceability Gate 必须 PASS（本方案即以其为完成条件）；主导角色 C01。
- **Option B**：由有权 Owner 正式批准追溯例外并记录范围与理由。
  - 后果：Baseline 就绪主张持续缺少机械可追溯证据；仅在明确接受该风险时可选。不推荐。

## 4. 风险与取舍

- F-02 Option A 收窄接受集：曾被错误接受的矛盾规则输入将被拒绝；这是失败关闭方向的缺陷修复，但属公共行为变化，故必须经本 HDP 显式批准而非默认执行。
- F-04 Option A 是九项中实现量最大者（证据采集 + 聚合 + 抑制表重写）；其收益是 PRD-006 可操作诊断对任意输入成立，而非仅对已见样例成立。
- F-06 Option B 以文档修订替代 Schema 扩展，避免制造恒空的机械类别；代价是 PRD-007 验收原文改写，需 C01 正式修订流程。
- F-08 建立的 ID 集为最小完备集；若未来需要 CON/SCN 等更完整需求建模，属另立任务，不在本次范围。
- 本机无 Rust 工具链：整改实施与验证必须在具备锁定离线工具链的环境执行；该证据受限性质须写入后续实施 Dispatch。

## 5. 推荐

- 整体：**批准** `WP-APLS-C04-REMEDIATION-001` 作为整改依据。
- Q1：Option A；Q2：Option B；Q3：Option A；Q4：Option A（子项 Q4-a 推荐"否"）。
- 整改分两批执行（批次 1 代码+契约，批次 2 需求追溯+文档货币性），全部完成后经单独授权创建新精确 Git Anchor，由全新独立 C04 按 `DELTA_ONLY` + 九条关闭条件必查项复审。

## 6. 各结论后果

- **全部批准**：DEC-030 进入第二阶段；C00 Dispatch 批次 1/2；完成后形成新 Review Target 与 Finding 关闭复审；全部 Finding 关闭且新 C04 PASS 前，Baseline Adoption 不可发起。
- **任一 Q 选项与推荐不同**：工作包对应条款按所选选项修订后执行；F-02/F-06 选 Descope 类选项将留下 Baseline 就绪缺口，需在 `CURRENT_STATE.md` 与 OPEN_QUESTIONS 中显式记录。
- **不批准工作包**：9 项 Open Finding 保持 Open，`CHANGES_REQUESTED` 状态不变，项目维持无产品 Baseline。

## 7. 明确不授权事项

即使本 HDP 全部批准，也**不授权**：

- Commit、Push、Tag、Branch、PR（新 Git Anchor 须参照 DEC-026 模式单独授权）；
- Baseline Adoption、Release、Formal Seal；
- 改变资源数值上限、Token Kind Rank、节点 Taxonomy、T0007 Payload 合同；
- 改变依赖闭包、版本、Checksum、Cargo Manifest/Lock；
- 改变 CLI 命令表面、退出码、`--version` 行为；
- 修改 Grammar（`apls_grammar.lalrpop` / `APLS_0.1_ZH_CN_GRAMMAR.ebnf`）；
- 除 Q2 Option A 外修改 IR Schema 或 Diagnostic Schema；
- 扩大公开语言接受集（仅 F-02 Option A 的收窄与 F-07 的码值归位经批准）；
- 整改参与方替自己关闭任何 Finding；Finding 关闭只能由全新独立 C04 复审判定。

## 8. 可复制回复

```text
HDP-APLS-024: APPROVED
WORK_PACKAGE: WP-APLS-C04-REMEDIATION-001
Q1 (F-02): A | B | C
Q2 (F-06): A | B | C
Q3 (F-04): A | B
Q4 (F-08): A | B
Q4-a: YES | NO
```

（如需部分批准或附加条件，请逐条注明；任何未填写的 Q 项视为该项不批准、对应 Finding 保持 Open。）
