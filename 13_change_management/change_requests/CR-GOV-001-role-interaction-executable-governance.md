# CR-GOV-001：岗位交互与可执行治理语义升级

## 0. 文档控制

```text
CHANGE_TYPE: GOVERNANCE_CHANGE
SEMANTIC_LEVEL: SUBSTANTIVE
STATUS: APPROVED_FOR_CANDIDATE_IMPLEMENTATION
CURRENT_VERSION: v0.1.4-beta.1
SOURCE_COMMIT: 87d879c8c52d7f8ac045b5a197cd50f937a513c9
TARGET_IDENTITY: v0.1.5-candidate
HUMAN_PROJECT_OWNER: Project Owner
DATE: 2026-08-29
```

本记录只适用于治理框架源仓库。外部产品项目只作为只读问题证据，不成为本框架 Current Truth，也不得由本 Change 修改。

## 1. 负责人决定

项目负责人已批准：

1. C00～C06 保持固定标准岗位；
2. 正式 Baseline 仍允许在精确预授权条件满足时由 C00 采用；
3. 新系统边界、公共接口、跨系统依赖、安全/数据完整性设计和重大不可逆取舍属于重大架构裁定；
4. Formal Seal 纳入正式治理概念并保留给 Human Project Owner；
5. 逻辑 C00 在模型切换时保持连续，物理上下文按 Knowledge Continuation / Baseline Relearn 规则处理；
6. 当前只形成 `v0.1.5-candidate`，不创建 Tag。

授权执行 `WP-GOV-015-01～06`，终点为：

```text
V0.1.5_CANDIDATE_OUTPUT_READY
```

## 2. 问题

v0.1.4 已具备 Current Truth、C00～C06、Model/Harness 解耦、权限继承、保障节奏、独立 C04、负责人决策包和 Baseline Relearn，但尚未统一定义：

- 动态岗位责任说明书；
- 岗位最小知识加载与规则缺口上报；
- 版本化岗位交互合同；
- 通用授权生命周期与消费/对账；
- 独立评审、带上下文复审、自审和人类审定四条运行线；
- Formal Seal；
- Procedural Fallback 与 Tool Enforced 双模式；
- 任务状态转换和正式 C04 独立性证据。

## 3. 目标

把治理框架从“由 Markdown 指导岗位协作”升级为“同时支持程序性执行和工具机械执行的同一套治理语义”，且工具实现不成为治理制度成立的前提。

核心表达：

```text
Role != Model != Runtime != Harness != Session != Tool
```

## 4. 工作包

| Work Package | 目标 | 授权终点 |
|---|---|---|
| `WP-GOV-015-01` | CR、问题清单、影响分析和兼容边界 | `CHANGE_SCOPE_RECORDED` |
| `WP-GOV-015-02` | 固定 Role、动态 Profile、知识加载和规则缺口 | `ROLE_KNOWLEDGE_POLICY_READY` |
| `WP-GOV-015-03` | Interaction Contract 与通用授权生命周期 | `INTERACTION_AUTHORIZATION_READY` |
| `WP-GOV-015-04` | 四条运行线、Formal Seal 和 C04 证据 | `REVIEW_DETERMINATION_READY` |
| `WP-GOV-015-05` | Task State Machine 与双执行模式 | `EXECUTION_STATE_READY` |
| `WP-GOV-015-06` | 启动、Role Brief、模板、Lite、迁移与术语整合 | `V0.1.5_CANDIDATE_OUTPUT_READY` |

## 5. 兼容边界

保持不变：

- Current Truth / One Fact One Owner；
- C00～C06 固定角色结构与基本职责；
- V 模型、需求追溯、ADR、Testing Governance；
- C04 Finding、Severity、Decision Matrix；
- 正式 C04 强制触发事件；
- Baseline Relearn 和治理升级状态链；
- v0.1.4-beta.1 的 Tag、Commit、Release 与历史记录。

增强：

- 增加 Runtime / Session 分离；
- 增加动态岗位、知识、交互和授权合同；
- 增加 Review Line 路由和 Formal Seal；
- 增加程序性/工具执行模式与状态转换。

需要迁移：

- 项目启动必须生成或声明 Dynamic Role Profile；
- `CURRENT_STATE.md` 必须选择 Enforcement Mode 并引用合同；
- 已有项目需要把现有任务状态映射到正式状态机；
- Model/Runtime/Harness 切换按新连续性规则记录；
- 采用候选前必须进行独立 C04、Baseline Adoption 和 Baseline Relearn。

## 6. 当前授权

允许：

- 修改治理模板文件；
- 新增本 CR、影响分析、权威政策和合同目录；
- 执行文档、YAML、索引、敏感信息和一致性检查。

禁止：

- Commit；
- 正式 C04；
- Baseline Adoption；
- Tag、Push、PR、Release；
- 修改任何外部产品项目；
- 安装或升级依赖；
- 真实模型产品调用；
- 修改或暂存既有 `output/marketing/` 文件。

## 7. Candidate 停止条件

达到以下条件后停止：

```text
POLICY_AND_CONTRACTS_COMPLETE: YES
ENTRY_POINTS_ALIGNED: YES
ROLE_BRIEFS_ALIGNED: YES
REVIEW_AND_TASK_TEMPLATES_ALIGNED: YES
MIGRATION_PATH_DEFINED: YES
MECHANICAL_VALIDATION: PASS
COMMIT_CREATED: NO
FORMAL_C04_STARTED: NO
BASELINE_ADOPTED: NO
FINAL_STATUS: V0.1.5_CANDIDATE_OUTPUT_READY
```

## 8. 后续精确授权

候选输出达到第 7 节停止条件后，Human Project Owner 进一步明确授权：

```text
AUTHORIZATION_SEQUENCE:
1. CREATE_EXACT_CANDIDATE_COMMIT
2. START_FORMAL_C04_AGAINST_THAT_COMMIT

BASELINE_ADOPTION: NOT_AUTHORIZED
TAG: NOT_AUTHORIZED
PUSH: NOT_AUTHORIZED
PR: NOT_AUTHORIZED
RELEASE: NOT_AUTHORIZED
FORMAL_SEAL: NOT_AUTHORIZED
OUTPUT_MARKETING_FILES: EXCLUDED_AND_UNTOUCHED
```

正式 C04 必须使用新的独立 Session，冻结到步骤 1 形成的完整 Commit Hash，并只输出正式 Review Record 与 `PASS / CHANGES_REQUESTED`。当前 C00 / Primary Executor Session 不得冒充 C04。

## 9. 正式 C04 Finding 整改闭环

针对精确 Review Target `51a7a41cfe5ac22034d1388f010d01efecf0d03a` 的正式 C04 输出 `CHANGES_REQUESTED`，记录见 `05_reviews/C04-GOV-001-v0.1.5-candidate.md`。本轮继续授权唯一绑定以下关闭范围：

- `C04-GOV-001-F001`：拆分独立 Session 创建、正式 C04 Dispatch 与真实 Model 调用的授权表达和适用性校验；
- `C04-GOV-001-F002`：补齐 Dynamic Role Profile 的当前或适用 Gate、适用事实 Owner 绑定和失败关闭条件；
- `C04-GOV-001-F003`：统一四类 Review Template 的 Finding 默认路由字段；
- 记录本次正式 C04、同步必要入口/升级说明与 Template File Index；
- 形成新的精确 Commit，并针对该 Commit 发起全新独立 C04 复审。

本轮仍不授权 Baseline Adoption、Tag、Push、PR、Release 或 Formal Seal；不得修改 `output/marketing/` 中其他会话的文件。

## 10. v0.1.5 正式发布授权

在精确整改 Commit `70a1f11ca502f53f8f04debcd071b562e571e131` 获得全新独立 C04 `PASS` 后，Human Project Owner 明确要求“正式发布 v0.1.5”。本授权唯一绑定以下发布 Package：

```text
RELEASE_AUTHORIZATION_ID: HPO-RELEASE-v0.1.5-20260829
AUTHORITY_OWNER: Human Project Owner
TARGET_VERSION: v0.1.5
TARGET_REPOSITORY: loocoo2025/ai-engineering-governance
TARGET_REMOTE: origin
RELEASE_CHANNEL: STABLE

AUTHORIZED_ACTIONS:
- 将候选版本展示元数据收口为正式 v0.1.5
- 记录上一轮正式 C04 PASS
- 形成精确 Release Commit
- 对 Release Commit 执行全新独立正式 C04
- Push origin/main
- 创建并 Push v0.1.5 Tag
- 创建公开 GitHub Release v0.1.5
- 上传由该 Tag 生成的正式 Git archive

FORMAL_SEAL: NOT_AUTHORIZED / NOT_ISSUED
DOWNSTREAM_BASELINE_ADOPTION: NOT_AUTHORIZED / NOT_PERFORMED
GITEE_PUSH: NOT_AUTHORIZED
HISTORY_REWRITE: NOT_AUTHORIZED
FORCE_PUSH: NOT_AUTHORIZED
OUTPUT_MARKETING_FILES: EXCLUDED_AND_UNTOUCHED
```

Release、Tag、Push 和 GitHub Release 必须解析到同一精确 Commit。最终独立 C04 不是 Formal Seal，也不替任何下游项目执行 Governance Baseline Adoption。
