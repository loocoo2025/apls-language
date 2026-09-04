# APLS 当前项目状态

> 本文件是项目阶段、授权、执行焦点和下一步的唯一权威来源。

## 1. 当前快照

- 项目：APLS（Agent Programming Language Specification）
- 当前产品版本：`0.1.0-draft`
- 当前开发阶段：`BASELINE_PREPARATION`
- 当前里程碑：`M28 — 当前 C05 PASS 候选的本地初始 Commit 已获授权`
- 最后更新时间：`2026-09-04`
- 当前项目状态：`TASK021_INITIAL_COMMIT_IN_PROGRESS`
- 当前 Baseline：见 `BASELINE_INDEX.md`
- 当前有效决定：见 `DECISION_INDEX.md`

## 2. 当前执行状态

- 当前 Gate：`TASK021_LOCAL_INITIAL_COMMIT_IN_PROGRESS`
- 当前执行焦点：排除构建缓存，冻结最终 staged set，创建一个本地初始 Commit
- 当前责任角色：`C00`
- 持续逻辑控制通道：`C00`
- 当前任务：`TASK-021 = IN_PROGRESS`

## 3. 当前授权边界与运行路由

```text
AUTONOMY_MODE: SUPERVISED_AUTO
AUTHORIZED_UNTIL: TASK021_COMMIT_CREATED_OR_FAILED
PREAUTHORIZED_GATES: NONE
ASSURANCE_CADENCE_PROFILE: LEAN
DEFAULT_REVIEW_SCOPE: DELTA_ONLY
ENFORCEMENT_MODE: PROCEDURAL_FALLBACK

CURRENT_ROLE_ASSIGNMENT: C00_CONTROL_CHANNEL
CURRENT_DYNAMIC_ROLE_PROFILE: C00_PROJECT_CONTROL_CURRENT
CURRENT_KNOWLEDGE_MANIFEST: KM-APLS-C00-021
CURRENT_INTERACTION: INT-APLS-C00-GIT-021
CURRENT_AUTHORIZATION: AUTH-APLS-C00-021 ISSUED_UNUSED

PRIMARY_EXECUTOR:
MODEL: CURRENT_CODEX_MODEL
RUNTIME: CODEX_DESKTOP_LOCAL
HARNESS: CODEX

EXPERT_ESCALATION_PRIMARY:
MODEL: CURRENT_CODEX_FRONTIER_MODEL
RUNTIME: CODEX_DESKTOP_LOCAL
HARNESS: CODEX

INDEPENDENT_REVIEWER_PRIMARY:
MODEL: CURRENT_CODEX_FRONTIER_MODEL
RUNTIME: NEW_INDEPENDENT_SESSION
HARNESS: CODEX

HUMAN_PROJECT_OWNER: PROJECT_OWNER_USER
```

### 当前允许

- 读取并盘点当前全部非忽略项目文件；
- 补充 `/07_src/target/` 构建缓存忽略规则及本次 Commit 授权/执行记录；
- 将最终非忽略文件加入 Git Index，核对 staged set 后创建一个本地初始 Commit；
- 读取并记录精确 Commit Hash。

### 当前禁止

- 改变已批准 Grammar、Frame、IR、Schema、诊断、CLI、资源数值或 Unicode 版本；
- 建立临时公开 CNL 子集、第二 Parser、旧 DSL/CNL 双入口或隐式顺序消歧；
- 使用最长匹配、概率、LLM、候选排名或第一个成功路径制造最终唯一性；
- 把资源耗尽误报为 `AMBIGUOUS`，或在未完整证明时产生 Frame/IR；
- 修改旧 `apls-ir-0.1.schema.json` 或把它宣称为当前 CNL IR；
- 未获新授权不得修改 Compiler/CLI Source、测试、Grammar、设计、Schema、Cargo Manifest 或 Lock；
- 不追加性能、Fuzz、跨平台或完整发布测试；
- 只允许本次授权的一个本地初始 Commit；
- 未经授权执行 Push、Tag、Branch、PR、Baseline Adoption、Formal C04、Release 或 Formal Seal。

## 4. 当前阻塞与风险

- 当前是否阻塞：`NO`
- P0/P1 未决问题：`NONE`
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
- Commit 授权：`HDP-APLS-021 Option A / DEC-026` 已签发，尚未消费。
- Residual Risk：`INITIAL_COMMIT_NOT_YET_CREATED_NO_FORMAL_C04_NO_PRODUCT_BASELINE`
- 已知实现偏差：`NONE_OPEN_IN_IMPLEMENTATION_REREVIEWS`

## 5. 当前下一步

1. 冻结并核验最终 staged set，创建授权的一个本地初始 Commit；
2. Commit 成功后返回 C00，记录精确 Hash；
3. Formal C04 与 Baseline Adoption 仍需后续独立授权。

## 6. 上下文健康

- 当前状态：`HEALTHY`
- 上一次 Baseline Relearn：`NEVER — PROJECT_INIT`
- HANDOFF 次数：`0`
