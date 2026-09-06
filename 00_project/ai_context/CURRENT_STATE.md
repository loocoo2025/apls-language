# APLS 当前项目状态

> 本文件是项目阶段、授权、执行焦点和下一步的唯一权威来源。

## 1. 当前快照

- 项目：APLS（Agent Programming Language Specification）
- 当前产品版本：`0.1.0-draft`
- 当前开发阶段：`BASELINE_PREPARATION`
- 当前里程碑：`M35 — TASK-024 批次 1/2 整改完成，追溯门禁 PASS，待新 Anchor 与关闭复审授权`
- 最后更新时间：`2026-09-06`
- 当前项目状态：`REMEDIATION_COMPLETE_ANCHOR_PENDING`
- 当前 Baseline：见 `BASELINE_INDEX.md`
- 当前有效决定：见 `DECISION_INDEX.md`

## 2. 当前执行状态

- 当前 Gate：`REMEDIATION_COMPLETE — NEW_ANCHOR_AND_REREVIEW_AUTHORIZATION_PENDING`
- 当前执行焦点：两批整改完成且复核通过；等待项目负责人裁决三件事——① C01 对 `system_architecture.md` §8 的扩展修正是否接受；② `APLS_0.1_ZH_CN_GRAMMAR.ebnf` 第 3 行过期状态注释修正授权；③ 新精确 Git Anchor 的 Commit 授权（之后才能发起 Finding 关闭复审）
- 当前责任角色：`C00`
- 持续逻辑控制通道：`C00`
- 当前任务：`TASK-022 = DONE（CHANGES_REQUESTED）`；`TASK-024 = IN_PROGRESS`（整改实施完成，待 Anchor 与关闭复审）

## 3. 当前授权边界与运行路由

```text
AUTONOMY_MODE: SUPERVISED_AUTO
AUTHORIZED_UNTIL: TASK023_DONE
PREAUTHORIZED_GATES: NONE
ASSURANCE_CADENCE_PROFILE: LEAN
DEFAULT_REVIEW_SCOPE: DELTA_ONLY
ENFORCEMENT_MODE: PROCEDURAL_FALLBACK

CURRENT_ROLE_ASSIGNMENT: C00_CONTROL_CHANNEL
CURRENT_DYNAMIC_ROLE_PROFILE: DRP-APLS-C00-023
CURRENT_KNOWLEDGE_MANIFEST: KM-APLS-C00-023
CURRENT_INTERACTION: INT-APLS-C00-TRANSFER-023
CURRENT_AUTHORIZATION: AUTH-APLS-C00-023-REMOTE/COMMIT/PUSH EXECUTION_COMPLETED
CURRENT_AUTHORIZATION: AUTH-APLS-C04-022-FORMAL-C04-DISPATCH CONSUMED（HDP-APLS-022 Option A / DEC-028；2026-09-05 Dispatch NISR-APLS-C04-022-001 已启动但在产出 Review Record 前被平台中止；对账完成：/private/tmp 副本已按精确 Commit 物化、Target 未改动、无 Gate Decision；NO_AUTOMATIC_RETRY，重新 Dispatch 需项目负责人新授权）
CURRENT_AUTHORIZATION: AUTH-APLS-C04-022-REDISPATCH-MANUAL EXECUTION_COMPLETED（DEC-029；APLS-C04-Independent-Review-v02 手动干净会话已完成评审，Review Record 与 Gate Decision 已产出）
CURRENT_AUTHORIZATION: AUTH-APLS-C00-030-REMEDIATION-ORGANIZATION EXECUTION_COMPLETED（DEC-030；C02-v02 已交付工作包与 HDP-APLS-024）
CURRENT_AUTHORIZATION: AUTH-APLS-C00-032-TOOLCHAIN-INSTALL EXECUTION_COMPLETED（DEC-032；rustup + Rust 1.98.0/1.86.0 已安装，cargo fetch/test --locked 基线核验通过）
CURRENT_AUTHORIZATION: AUTH-APLS-C03-024-BATCH1-IMPLEMENTATION EXECUTION_COMPLETED（HDP-APLS-024 / DEC-031；批次 1 完成：Compiler 36 PASS/1 ignored、CLI 7 PASS/1 ignored、MSRV 1.86 check 通过、fmt 通过，C00 复核一致）
CURRENT_AUTHORIZATION: AUTH-APLS-C01-024-BATCH2-IMPLEMENTATION EXECUTION_COMPLETED（HDP-APLS-024 / DEC-031；批次 2 完成：27 个新需求 ID、30 条 FORMAL_TRACE 边，validate_traceability.py PASS，C00 复核一致；遗留：§8 扩展修正待裁决、EBNF 第 3 行状态注释待授权）

PRIMARY_EXECUTOR:
MODEL: KIMI_K3
RUNTIME: OPENCODE_LOCAL
HARNESS: OPENCODE

EXPERT_ESCALATION_PRIMARY:
MODEL: KIMI_K3
RUNTIME: OPENCODE_LOCAL
HARNESS: OPENCODE

INDEPENDENT_REVIEWER_PRIMARY:
MODEL: KIMI_K3
RUNTIME: NEW_INDEPENDENT_SESSION
HARNESS: OPENCODE

HUMAN_PROJECT_OWNER: PROJECT_OWNER_USER
```

> 路由值已于 2026-09-05 按 `DEC-027`（逻辑 C00 移交 Kimi）和项目负责人对首次正式 C04 Reviewer 放置的明确裁决更新为当前实际 Harness；Reviewer 与 Codex 实现/整改 Session 构成 Provider 级隔离。

### 当前允许

- Kimi C00-v02 从私有仓库读取当前 `main`，执行 Baseline Relearn 和上下文校验；
- 依据 `HDP-APLS-022 Option A / DEC-028` 发起一次（且仅一次）全新独立 C04 Session：Reviewer 只读审查精确 Commit `3289037...`，可在 `/private/tmp` 物化只读副本并运行与首次正式 Gate 直接相关的锁定离线构建/测试和 Traceability Validator，仅写入 `05_reviews/FORMAL_C04_APLS_0_1_CANDIDATE_001.md`；
- 保持产品候选 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e` 不变。

### 当前禁止

- 改变已批准 Grammar、Frame、IR、Schema、诊断、CLI、资源数值或 Unicode 版本；
- 建立临时公开 CNL 子集、第二 Parser、旧 DSL/CNL 双入口或隐式顺序消歧；
- 使用最长匹配、概率、LLM、候选排名或第一个成功路径制造最终唯一性；
- 把资源耗尽误报为 `AMBIGUOUS`，或在未完整证明时产生 Frame/IR；
- 修改旧 `apls-ir-0.1.schema.json` 或把它宣称为当前 CNL IR；
- 未获新授权不得修改 Compiler/CLI Source、测试、Grammar、设计、Schema、Cargo Manifest 或 Lock；
- 不追加性能、Fuzz、跨平台或完整发布测试；
- TASK-023 的 Commit/Push 授权已经执行完成；未经新授权不得继续 Commit 或远程修改；
- 除 `HDP-APLS-022 / DEC-028` 授权的一次正式 C04 Dispatch 外，未经新授权不得执行 Tag、Branch、PR、Baseline Adoption、Release、Formal Seal、公开仓库或邀请协作者；C04 Session 不得修改 Target、Compiler/CLI Source、测试、设计、Schema、Cargo Manifest 或 Lock，不得 Commit/Push，不得参与整改或关闭自己的 Finding。

## 4. 当前阻塞与风险

- 当前是否阻塞：`NO — Rust 1.98.0（默认）/1.86.0 工具链已安装（DEC-032），锁定离线基线核验通过（Compiler 24 PASS/1 ignored、CLI 全 PASS）`
- P0/P1 未决问题：`NONE（Q-024 已由 HDP-APLS-024 APPROVED / DEC-031 关闭；Q-003 为 P2 保持 OPEN）`
- 第一次再复审：`IIR-APLS-TASK019-REREVIEW-001 = CHANGES_RECOMMENDED`；BF-02/03/04/05/08/09 Closed，BF-06/07/10 Partial，新增 NF-01～NF-04。
- 第二次再复审：`IIR-APLS-TASK019-REREVIEW-002 = CHANGES_RECOMMENDED`；BF-02/03/04/05/07/08/09 与 NF-01/02/04 Closed，BF-06/10 与 NF-03 Partial，新增 NF-05。
- 第三次增量再复审：`IIR-APLS-TASK019-REREVIEW-003 = READY_FOR_HUMAN_DETERMINATION`；BF-02～BF-10 与 NF-01～NF-05 全部 Closed，其中未变化项按 `INHERITED_CLOSED` 继承。
- TASK-018 实现增量复审：`IIR-APLS-TASK018-IMPLEMENTATION-001 = CHANGES_RECOMMENDED`；Target Digest 三次匹配；`IO-01～IO-07` Open。
- 实施恢复：`HDP-APLS-019 Option A / DEC-024` 已批准实际空 `tinyvec/default` Feature 闭包及 `IO-01～IO-07` 精确 Retry。
- Retry 1 实施：C03 报告 `IO-01～IO-07` 全部完成并通过普通锁定构建、测试和定向关闭探针；尚未获得独立关闭结论。
- Retry 1 独立再复审：`IIR-APLS-TASK018-IMPLEMENTATION-002 = CHANGES_RECOMMENDED`；`IO-01/03/04/05/06/07 = CLOSED`，`IO-02 = PARTIAL`，新增 `NB-01`。
- Retry 2 实施：C03 已按实际 Grammar Production 修正 `ws1/opt-space`，Compiler 24 PASS/1 ignored、CLI 7 PASS/1 ignored。
- Retry 2 独立增量再复审：`IIR-APLS-TASK018-IMPLEMENTATION-003 = READY_FOR_C00_DETERMINATION`；21 文件摘要三次匹配；`NB-01 / IO-02 = CLOSED`，`IO-01/03/04/05/06/07 = INHERITED_CLOSED`，无新阻断观察。
- C00 实现收口：TASK-018 曾从 `OUTPUT_READY → READY_FOR_REVIEW`，现已在 C05 PASS 后完成；这仍不等于正式 C04、Baseline 或 Release 通过。
- Enforcement Evidence：`IIR_TASK018_RETRY2_NB01_CLOSED_AND_VAL_APLS_C05_020_PASS`
- Mechanical Enforcement：`TASK018_AND_TASK020_DONE`
- C05 授权：`HDP-APLS-020 Option A / DEC-025` 已执行完成。
- C05 结果：`VAL-APLS-C05-020 = PASS`；Compiler 24、CLI 7 项普通测试通过；Rust 1.98/1.86 锁定检查通过；公共资源边界与 Unicode 17.0.0 官方全量一致性通过。
- 完整性：排除构建输出后的 74 个受保护设计/源码/测试文件前后摘要均为 `c52271e9e5f25eed993ab00d8af17d27fd3f6258ed151f5a35b1552aa29fb1f8`；Cargo/Lock 未变化。
- Commit 结果：`3289037bee1aab64dfa2d58188379a68dcfa601e`，Tree `51edf42da73237bcb3408e234d2cbb2095758fa1`，249 文件，Git 对象核验通过。
- 私有远程：`https://github.com/loocoo2025/apls-language` 已创建并确认 `Private`；首次同步 `origin/main = fa3e09f35eb1d7b5418cf73edba80a57aa5aa769`。
- Residual Risk：`FORMAL_C04_CHANGES_REQUESTED_NO_PRODUCT_BASELINE`
- 首次正式 C04：`FORMAL_C04_APLS_0_1_CANDIDATE_001 = CHANGES_REQUESTED`（Target `3289037...` 三时点身份核验一致、评审期间未改动）；Open Findings 9（S1：F-01/F-02；S2：F-03/F-04/F-05/F-06/F-08；S3：F-07/F-09），Advisory 4，证据受限 E-01～E-03（本机无 Rust 工具链，测试未独立重跑）
- 已知实现偏差：`NONE_OPEN_IN_IMPLEMENTATION_REREVIEWS`

## 5. 当前下一步

1. 项目负责人裁决：① `system_architecture.md` §8 扩展修正（接受/回退）；② `APLS_0.1_ZH_CN_GRAMMAR.ebnf` 第 3 行过期状态注释的修正授权；③ 参照 DEC-026 模式授权创建一个包含全部整改与治理落盘的新精确 Git Anchor；
2. Anchor 建立后，C00 发起全新独立 C04 Session 按 `DELTA_ONLY` + 九条关闭条件必查项复审 Finding 关闭；
3. 只有复审 PASS 且 Open Finding 为 0 后，才另行裁决产品 Baseline Adoption。

## 6. 上下文健康

- 当前状态：`HEALTHY`
- 上一次 Baseline Relearn：`2026-09-05 — Kimi C00-v02 接管完成（权威文件 + 精确 Target 只读核验通过）`
- HANDOFF 次数：`1（C00-v01 → C00-v02，交接包已同步）`
