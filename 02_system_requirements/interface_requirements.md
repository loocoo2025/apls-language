# 接口需求

- 状态：`CONFIRMED`
- 建立依据：`DEC-031`（`HDP-APLS-024` Q4=A，F-08 整改）；IF-001～IF-003 派生自 PRD-006～008 已批准公开契约，不新增 PRD 未表述的接口。

## IF-001 — CLI 命令表面与进程边界

- 名称：APLS 0.1 CLI 命令表面（`apls`）
- 对端：人类用户与 AI Agent 的本地进程调用方
- 协议：本地进程调用（argv 参数、stdout/stderr、进程退出码）；无网络协议
- 数据格式：命令为 `apls parse <entry>` / `apls check <entry>` / `apls emit-ir <entry> --output <path>` / `apls diagnose <entry> [--through parse|check|emit] [--format json|human]` 与 `apls --version`；stdout 仅承载有效 Artifact；诊断经 stderr（人读）或 Diagnostic Envelope（机读，见 IF-002）
- 超时：不适用；单次编译事务受固定资源上限约束（超限为 `APLS-T0007` / Exit 2）
- 重试：同一编译事务不自动重试；Source 竞态、I/O 或发布失败后由调用者发起新事务
- 兼容性：命令集合、退出码语义（0=成功；1=Source 诊断拒绝；2=Tool Failure（含参数错误 `APLS-T0001` 与资源超限 `APLS-T0007`）；3=Compiler Internal Failure `APLS-T0006`）与 `--version` 固定行 `apls 0.1.0 language=0.1 ir=0.1` 为冻结契约
- 错误处理：失败关闭；`emit-ir` 发布前复读 Source 并原子发布；符号链接、路径穿越与非普通文件拒绝；仅 Exit 0 且完整可验证时构成有效 IR 交付
- 安全：不执行 Source 中任何内容；不在未授权时修改源文件
- Traces-From（正式上游追溯）：
  - PRD-008
  - AC-008
- 状态：CONFIRMED

## IF-002 — Diagnostic Envelope 机器契约

- 名称：Diagnostic Envelope（`apls-cnl-diagnostic-0.1`）
- 对端：人类用户（经人读渲染）与 AI Agent / 工具（机读 JSON）
- 协议：随 CLI 诊断输出（见 IF-001）承载；无独立传输协议
- 数据格式：JSON，符合 `04_design/diagnostics/apls-cnl-diagnostic-0.1.schema.json`；每条诊断含稳定代码、严重度、文件、起止 Byte Span、根因与类型化 Payload；Envelope `status` 区分 `accepted` / `rejected` / `tool_failure` / `internal_failure`；公共总排序与第 1001 条截断按 `DES-APLS-CNL-DIAG-001` §1.2
- 超时：不适用
- 重试：不适用
- 兼容性：诊断 Code 的根因身份在 0.1 内不得漂移；Schema 版本化
- 错误处理：零有效候选时按封闭根因聚合输出根因组；歧义以 `APLS-E1310` 附至少两个分歧见证
- 安全：诊断不暴露 Verified IR 内容；可修复建议不自动改写 Source
- Traces-From（正式上游追溯）：
  - PRD-006
  - AC-006
- 状态：CONFIRMED

## IF-003 — Verified Canonical IR 消费契约

- 名称：Verified Canonical IR（`apls-cnl-ir-0.1`）
- 对端：AI Agent 与下游工具消费者（Graph / Verification / Backend Consumer）
- 协议：经 `apls emit-ir` 发布（见 IF-001）；无独立传输协议
- 数据格式：JSON，符合 `04_design/ir/apls-cnl-ir-0.1.schema.json`；发布前对输出 Byte 重新解析、Schema 复验与跨节点复验；Header 记录 `language_version`、`ir_schema_version`、编译器身份、源摘要与生成状态 `verified`；机械区分 normative 与 informative；unknown/open 内容不进入 IR（编译期以稳定诊断拒绝，见 SYS-007）
- 超时：不适用
- 重试：不适用
- 兼容性：Schema 显式版本化（`apls-cnl-ir-0.1`）；匿名节点 ID 与 `semantic_hash` 为版本化 SHA-256，跨执行稳定
- 错误处理：复验失败即失败关闭，不产出部分 IR
- 安全：每个规范节点可溯源回源文件与源码范围；未经验证的源文本不构成规范事实来源
- Traces-From（正式上游追溯）：
  - PRD-007
  - AC-007
- 状态：CONFIRMED
