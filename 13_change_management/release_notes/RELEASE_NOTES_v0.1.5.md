# AI Software Engineering Governance Framework v0.1.5

# AI 软件工程治理框架 v0.1.5

面向长周期 AI 软件开发的模型无关工程治理框架。

## 本版本是什么

`v0.1.5` 是继 `v0.1.3` 后的下一稳定版本，包含 `v0.1.4-beta.1` 已验证的治理能力，并完成岗位交互与可执行治理合同的稳定化。

## 主要新增

- 固定 C00～C06 标准岗位，并通过 Dynamic Role Profile 收窄每次 Session 的任务、Gate、事实 Owner、工具和授权边界；
- 增加 Knowledge Manifest、Rule Gap、Interaction Contract / Operation 和 Authorization Contract 生命周期；
- 建立 `INDEPENDENT_REVIEW / CONTEXTUAL_REVIEW / SELF_REVIEW / HUMAN_DETERMINATION` 四条审核与裁决运行线；
- 增加正式 Task State Machine、C04 独立性证据和 `PROCEDURAL_FALLBACK / TOOL_ENFORCED` 双执行模式；
- 增加保障节奏、不可关闭控制项、外部 AI 调用配置与独立 Session 自动创建协议；
- 支持当前或历史精确 Commit 作为正式 C04 Review Target。

## 核心规则

```text
Role != Model != Runtime != Harness != Session != Tool
```

- Model、Runtime、Harness、Session 或 Tool 的技术可用性不产生治理角色或批准权；
- Session 创建、正式 C04 Dispatch、真实 Model 调用、Commit、Tag、Push、Baseline Adoption、Formal Seal 与 Release 是独立 Action Class；
- Dynamic Role Profile 必须绑定当前或适用 Gate 与适用事实 Owner，并在缺失、过期或冲突时失败关闭；
- 正式 C04 必须使用全新独立 Session、精确不可变 Target 和正式 Review Record。

## 本版本修复

- 独立 Session 请求可分别表达 Session 创建、正式 C04 Dispatch 与真实 Model 调用授权；
- 为 create-only、Dispatch 不触发新 Model 调用以及三类动作全部执行定义机械适用规则；
- Requirement、Architecture、Design 与 Code Review Template 统一支持 Finding 的 `Default Route`；
- Dynamic Role Profile 机器合同补齐 Gate 与事实 Owner 绑定及失败关闭条件。

## 升级说明

- `v0.1.4-beta.1` 项目可以使用 `13_change_management/templates/治理模板升级-GOVERNANCE_TEMPLATE_UPGRADE_TEMPLATE.md` 直接升级；
- `v0.1.3` 及更早版本也使用同一升级协议，并按协议逐版本读取迁移信息；
- 升级必须锁定 `v0.1.5` Tag 和完整 Commit，不得使用浮动 `main` 或 `latest`；
- 本版本改变启动、权限、Interaction、Review Line、Task 和 Session 连续性语义，采用后 `BASELINE_RELEARN: REQUIRED`；
- 升级治理框架不会自动改变项目当前里程碑、产品需求、架构、接口、代码或测试结论。

## 已知限制

- `GOVERNANCE_EXECUTION_CONTRACTS.yaml` 是机器可读合同目录，不是某个具体 Runtime 的完整执行器；
- `TOOL_ENFORCED` 需要 Agent Runtime 或治理工具实际消费合同并提供机械证据；
- 没有专用工具时使用 `PROCEDURAL_FALLBACK`，仍需依靠受控文件、Git、独立 Session 和检查表完成闭环。

## 验证

- 正式独立 C04：`PASS`；
- Open Finding：0；
- YAML、Markdown、本地链接、Template File Index、Git whitespace 与敏感信息扫描：通过；
- 产品构建和产品测试：不适用，本版本不包含产品代码变更。

## 许可证

Apache License 2.0。
