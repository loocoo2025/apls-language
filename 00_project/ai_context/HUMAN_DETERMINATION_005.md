# HUMAN_DETERMINATION_PACKAGE

```text
DETERMINATION_ID: HDP-APLS-005
TARGET: DES-APLS-COMPILER-001
DECISION: APPROVED
DECISION_DATE: 2026-09-02
DECISION_EVIDENCE: 项目负责人“HDP-APLS-005: APPROVED”
```

## WHAT_MUST_BE_DECIDED

是否批准 `04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md` 作为后续实现技术决策和 Conformance 测试设计的受控输入，并冻结其中的 MVP 公共契约候选：

- `apls parse/check/emit-ir/diagnose` 四个命令；
- Exit Code `0/1/2/3`；
- `apls-diagnostics 0.1` 机器 Envelope；
- `APLS-T0001`～`APLS-T0007` Tool Diagnostic；
- Verified IR MIME 身份与失败关闭发布语义。

## WHY_HUMAN_AUTHORITY_IS_REQUIRED

这些命令、退出码、诊断格式和 Artifact 身份将直接被人类、CI 和 AI Agent 依赖，属于新的长期公共接口。

## CONFIRMED_FACTS

- Compiler 事务只产生 `REJECTED + diagnostics` 或 `ACCEPTED + verified IR` 两种规范结果；
- Stage 从 Source 加载到 IR 验证单向前进，任一 Error 均失败关闭；
- `parse` 不公开 AST，避免形成第二机器契约；
- Import 只支持 Entry Root 下的精确本地路径，拒绝远程 Import、Symbolic Link 和路径猜测；
- Publisher 只接受从实际 JSON Byte 重新解析并验证的 `VerifiedArtifact`；
- PRD-001～PRD-008 均有明确追溯。

## OPEN_QUESTIONS

- `Q-002` 实现语言、Parser 技术、`Q-003` 参考后端和 `Q-007` 语义哈希仍未冻结；
- 具体资源上限将随 Compiler Version 在实现设计时给出；
- 本包不要求同时选择实现语言或开始编码。

## OPTIONS_AND_DIFFERENCES

- `APPROVED`：冻结当前 MVP 公共契约，允许进入实现语言/Parser 技术决策与 Conformance 测试设计；
- `CHANGES_REQUESTED`：指定命令、Stage、Import、Diagnostic 或 Publisher 需修改的条目；
- `DEFERRED`：保留候选，不进入实现技术决策；
- `REJECTED`：关闭当前 Compiler MVP 设计路线并重新建立候选。

## RISKS_AND_TRADEOFFS

- 不公开 AST 降低了调试便利性，但避免 Agent 误把内部或部分结果当作规范事实；
- 拒绝 Symbolic Link 和远程 Import 缩小了项目组织方式，但消除了主机解析和网络状态差异；
- MVP 无缓存和自动重试，性能与便利性有限，但编译事务身份清晰；
- Tool Diagnostic 允许 `primary_source_span: null`，与 Source Error 在同一 Envelope 内使用不同适用语义。

## RECOMMENDED_OPTION_AND_REASON

推荐 `APPROVED`。该设计把“拒绝歧义”落实到每个 Compiler Stage、Import 路径、命令输出、诊断顺序和 Artifact 发布，同时不把实现语言或后端偏好写入公共契约。

## CONSEQUENCES

- `APPROVED`：`DES-APLS-COMPILER-001` 标记为 `APPROVED_DESIGN_INPUT_NOT_BASELINED`，`TASK-004` 完成；
- `CHANGES_REQUESTED`：`TASK-004` 返回 `IN_PROGRESS`，只修改被指定的条目；
- `DEFERRED`：保持当前候选和评审状态；
- `REJECTED`：将当前候选标记为不采用并重新设计。

## EXPLICITLY_NOT_AUTHORIZED

本决定不授权选择实现语言或 Parser 技术、安装依赖、编写 Compiler 代码、冻结参考后端或哈希算法、Commit、Tag、Baseline Adoption、Formal C04 Dispatch、Push、Release、远程修改或 Formal Seal。

## AUTHORITATIVE_SOURCES

- `01_product_requirements/PRD.md`；
- `03_architecture/system_architecture.md`；
- `04_design/language/APLS_0.1_LANGUAGE_DESIGN.md`；
- `04_design/ir/APLS_0.1_CANONICAL_IR.md`；
- `04_design/diagnostics/APLS_0.1_DIAGNOSTICS.md`；
- `04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md`。

## COPYABLE_RESPONSE_FORMAT

```text
HDP-APLS-005: APPROVED
```

或：

```text
HDP-APLS-005: CHANGES_REQUESTED
修改：<具体命令、Stage、Import、Diagnostic 或 Publisher 条目>
```
