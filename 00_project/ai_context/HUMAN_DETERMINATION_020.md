# HDP-APLS-020 — TASK-020 C05 最小风险驱动验证授权

```yaml
determination_id: HDP-APLS-020
status: APPROVED
selected_option: A
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-04
```

## 1. 必须决定什么

是否授权 `TASK-020` 由 C05 对已经进入 `READY_FOR_REVIEW` 的 TASK-018 实现候选执行一次最小、风险驱动的验证，并允许仅为 Unicode 17.0.0 一致性测试从 Unicode 官方站点取得对应数据文件。

## 2. 已确认事实

- TASK-018 的三轮实现增量复审已经关闭全部 `IO-01～IO-07` 和 `NB-01`；最后一轮 21 文件 Target 摘要三次匹配，无新阻断观察。
- C03 普通锁定测试已通过：Compiler 24 passed/1 ignored，CLI 7 passed/1 ignored；Retry 2 独立复审又运行了两个定向测试、`fmt --check` 和 workspace locked/offline check。
- 两个 ignored 用例不是普通回归：一个验证公开候选资源的真实边界/首次超限，另一个使用 Unicode 17.0.0 官方 `NormalizationTest.txt` 验证 NFC 数据实现。
- 当前结果仍只是实现与非正式独立复审证据，不是 C05、正式 C04、Baseline 或 Release 结论。

## 3. 选项

### Option A — 执行最小完整 C05 验证（推荐）

C05 仅执行以下工作：

1. 按 `CNL-C001～C016` 建立简短 Traceability，并把用例分成 T0/T1/T2/T3；
2. 运行 `cargo fmt --all --check`、Rust 1.98 和 MSRV 1.86 的 workspace `check --locked --offline`；
3. 运行 Compiler 与 CLI 普通 `test --locked --offline`；
4. 显式运行公共候选资源边界 ignored 用例；
5. 只从 Unicode 官方 HTTPS 地址取得 Unicode 17.0.0 `NormalizationTest.txt` 到 `/private/tmp`，记录来源与 SHA-256，并显式运行官方 NFC 一致性 ignored 用例；
6. 输出精简验证计划和 C05 验证报告，记录通过项、失败项、未执行项和剩余风险。

如任一验证失败或官方数据身份无法确认，TASK-020 立即报告 `BLOCKED/FAIL` 返回 C00；本授权不允许 C05 修改 Compiler Source、测试、设计、Cargo 或 Lock 来消除失败。

优点：用最低额外成本补齐目前明确缺少的两个高风险证据，同时避免 Fuzz、性能、跨平台和组合爆炸。

### Option B — 只做完全离线普通验证

运行普通锁定构建和测试，但不获取 Unicode 官方数据，也不执行两个昂贵 ignored 用例。

缺点：只能形成部分 C05 证据；Unicode 17.0.0 完整一致性和公开资源边界仍是已知验证缺口，不足以推荐后续正式 C04/Baseline。

### Option C — 暂缓

TASK-018 保持 `READY_FOR_REVIEW`，TASK-020 保持 `READY`，不启动 C05。

## 4. 推荐

推荐 `Option A`。当前最大的剩余风险正是资源上限真实公共路径和 Unicode NFC 数据版本；两项都有现成定向用例，不需要新增测试基础设施或扩大实现范围。

## 5. Option A 的授权边界

允许：

- 读取 TASK-018 当前实现、批准契约、CNL-C001～C016 和现有验证证据；
- 运行第 3 节列出的锁定构建与测试；
- 从 Unicode 官方 HTTPS 地址只读下载 Unicode 17.0.0 `NormalizationTest.txt` 到 `/private/tmp` 并计算摘要；
- 写入 `06_test_design/APLS_0.1_CNL_IMPLEMENTATION_VERIFICATION_PLAN.md`、`11_validation/validation_reports/TASK020_C05_CNL_VERIFICATION.md` 及对应最小 Current Truth 状态。

不允许：

- 修改 Compiler/CLI Source、现有测试、Grammar、设计、Schema、Cargo Manifest 或 Lock；
- 新增/升级依赖，改变 Acceptance Threshold、资源数值、Unicode 版本或公共行为；
- 执行性能、Fuzz、跨平台、完整发布测试或与 T0/T1 无关的额外测试；
- Commit、Push、Baseline Adoption、Formal C04、Release 或 Formal Seal。

## 6. 完成条件

- 计划对 `CNL-C001～C016` 给出测试映射和 T0/T1/T2/T3 分类；
- Option A 的普通与两项高风险用例均有真实执行结果；
- 官方 Unicode 数据来源和 SHA-256 可追溯；
- C05 报告不把未执行或失败项表述为通过；
- 完成后回到 C00，另行决定初始 Commit、正式 C04 和 Baseline Adoption。

## 7. 可复制回复

```text
HDP-APLS-020: APPROVED
OPTION: A
```

## 8. 裁决结果

项目负责人于 `2026-09-04` 明确批准 Option A。本批准形成 `DEC-025`，授权 TASK-020 按本文件第 3、5、6 节执行最小风险驱动 C05 验证，并只从 Unicode 官方 HTTPS 地址取得 Unicode 17.0.0 `NormalizationTest.txt` 到 `/private/tmp`。未列出的副作用仍不授权。
