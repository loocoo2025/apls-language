# 详细设计总览

> 当前状态：`TASK-019` 已由 `DEC-023`（`HDP-APLS-018 Option A`）采用并完成；`TASK-018` CNL 垂直切片已实施（整改按 `DEC-024` 完成），通过 `TASK-020` C05 验证（`DEC-025`），候选冻结为 Commit `3289037bee1aab64dfa2d58188379a68dcfa601e`（`DEC-026`）。首次正式 C04 结论 `CHANGES_REQUESTED`，整改按 `WP-APLS-C04-REMEDIATION-001` / `DEC-031` 执行中。旧 DSL 不再定义当前公开语言。

## 模块列表

| 模块 | 职责 | 输入 | 输出 | 依赖 | 对应需求 |
|---|---|---|---|---|---|
| Controlled Natural Language | 唯一公开 CNL Profile、候选分析、句式和歧义边界 | CNL `.apls` Source | 唯一 Canonical Frame 等价类或稳定拒绝 | DEC-014、DEC-017、ARCH-APLS-CNL-001 | PRD-001、PRD-002、PRD-004、PRD-009 |
| Canonical IR | 表示已解析、已定型、已规范化的机器契约 | Checked Model | Verified JSON IR | ARCH-APLS-001 | PRD-003、PRD-007 |
| Diagnostics | 稳定编号、源码范围与失败关闭 | 各编译阶段错误 | 人类/机器可读诊断 | ARCH-APLS-001 | PRD-002、PRD-005、PRD-006 |
| Compiler MVP | 确定性阶段编排、命令契约和 Verified IR 发布 | Entry CNL `.apls` | Diagnostics 或 Verified IR | DEC-008、DEC-014 | PRD-001～PRD-009 |

## 公共接口

- `../03_architecture/APLS_0.1_CONTROLLED_NATURAL_LANGUAGE_ARCHITECTURE.md`（当前架构候选）
- `language/APLS_0.1_CNL_AMBIGUITY_BOUNDARY.md`（当前歧义边界候选）
- `language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md`（当前简体中文 Profile 候选）
- `language/APLS_0.1_ZH_CN_GRAMMAR.ebnf`（当前 CNL Grammar 候选）
- `language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md`（当前 Frame 候选）
- `language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md`（TASK-019 类型、单位、Condition、Transition、Unit/Span 候选）
- `diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md`（当前 CNL 诊断候选）
- `diagnostics/apls-cnl-diagnostic-0.1.schema.json`（TASK-019 机器诊断 Schema 候选）
- `language/APLS_0.1_LANGUAGE_DESIGN.md`
- `language/APLS_0.1_GRAMMAR.ebnf`
- `ir/APLS_0.1_CANONICAL_IR.md`
- `ir/apls-ir-0.1.schema.json`
- `ir/APLS_0.1_CNL_CANONICAL_IR.md`（当前 CNL IR 候选）
- `ir/apls-cnl-ir-0.1.schema.json`（当前 CNL Schema 候选）
- `diagnostics/APLS_0.1_DIAGNOSTICS.md`
- `compiler/APLS_0.1_COMPILER_MVP_DESIGN.md`（已批准设计输入，未建立产品 Baseline）
- `compiler/APLS_0.1_IMPLEMENTATION_FOUNDATION_DESIGN.md`（已批准设计输入，未建立产品 Baseline）
- `compiler/APLS_0.1_CNL_RESOURCE_AND_ORDERING_PROFILE.md`（TASK-019 资源与稳定顺序候选）
- `compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md`（TASK-019 Unicode/NFC 与依赖候选）
- `compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md`（工作包已批准，`TASK-018` 已按其实施）
- `../06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md`（16 个实施前最小一致性 Case）

旧 Language Design、EBNF、Lexical Profile 及现有 Compiler Frontend 只作 `DEC-014` 前迁移参考，不是当前 CNL 公共接口。

旧 `apls-ir-0.1` 只作 Legacy DSL 迁移参考；`apls-cnl-ir-0.1` 已由 `HDP-APLS-015 Option A` / `DEC-019` 批准为 CNL 实现输入，但尚未成为产品 Baseline。

## 状态机

- `TASK-015` 中文语言契约已由 `HDP-APLS-012` 批准为设计输入；`Q-014` 已由 `HDP-APLS-013 Option B` / `DEC-017` 关闭，`TASK-016` 恢复并形成 Frontend 适配输出。

## 线程模型

- Compiler 实现线程模型尚未冻结；APLS 规范内的 Execution Unit 为语义模型，不是编译器实现线程。

## 生命周期/资源所有权

- Compiler 事务使用不可变 Stage Artifact；详见 Compiler MVP 设计候选。

## 错误处理

- 任一 Error 存在时失败关闭，不发布 Verified IR。

## 超时/重试

- 同一编译事务不自动重试；Source 竞态、I/O 或发布失败后由调用者发起新事务。

## 日志/诊断

- 诊断公共契约见 `diagnostics/APLS_0.1_DIAGNOSTICS.md`；MVP stdout/stderr 不混入非诊断运行日志。

## 配置

- Compiler 实现已选择 Rust 2024 + LALRPOP 默认 LR(1)；原工具链与依赖闭包已批准。语义哈希已由 `DEC-019` 冻结为版本化 SHA-256；完整 NFC 所需新增依赖已由 `DEC-023` 批准、实际 Feature 闭包由 `DEC-024` 接受。参考后端仍未冻结。

## 测试点

- Grammar 工具冲突为零、候选分析完整性与 Canonical Frame 收敛唯一性；
- Surface 到 IR 的确定映射；
- 名称、类型、单位、状态和规则冲突的负向样例；
- Canonicalization 字节级稳定性；
- 诊断 Code、排序和 Source Span 稳定性。
