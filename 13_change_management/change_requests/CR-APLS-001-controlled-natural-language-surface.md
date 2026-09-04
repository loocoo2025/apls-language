# CR-APLS-001：将 APLS 公开表面语法改为受控自然语言

- 提出人：Human Project Owner
- 日期：2026-09-03
- 优先级：P1
- 变更分类：`REQUIREMENT_CHANGE + ARCHITECTURE_CHANGE + DESIGN_CHANGE`
- 决定：`APPROVED`
- 决定依据：`HDP-APLS-010 / OPTION A`

## 原因

现有花括号 DSL 仍要求用户学习计算机语法。项目负责人明确要求 APLS 成为“可编译的自然语言”，且用户默认不接触 DSL、AST 或 IR。

## 原决定

- `DEC-007`：单一自定义 `.apls` DSL 是唯一规范性源语言。
- `DEC-012`：为该 DSL 冻结 ASCII 词法行为。
- `DEC-013`：为该 DSL 冻结限定名源码形式和 Action 边界。

## 新决定

APLS 的唯一公开 Surface Syntax 改为版本化的受控自然语言。按后续 `DEC-017` 精炼，每个句子可以保留多个中间分析候选；被接受前，全部有效候选必须收敛为一个类型化 Canonical Frame 等价类，再进入语义冲突和 Canonical IR 检查。

```text
多个已批准表达 -> 一个 Canonical Meaning    允许
一个表达 -> 多个中间分析 -> 一个 Canonical Meaning 允许
一个表达 -> 多个可能 Meaning             拒绝
```

## 影响范围

- 需求：公开编程界面从 DSL 改为受控自然语言；
- 架构：新增确定性 CNL Frontend 和 Semantic Frame 阶段；
- 设计：需重建词法、句法、术语、指代、模态、数量与句间上下文边界；
- 代码：现有 Lexer/Parser/Surface AST 不再能作为新公开语法的符合性证据；
- 可复用：Canonical IR 方向、失败关闭流水线、Rust 工程骨架、诊断边界、Source Graph 概念和 Index/Resolver 语义职责。

## 兼容与迁移

当前没有已发布 APLS 产品 Baseline，因此不承诺旧 DSL Source 兼容性。旧代码保留为迁移参考和可复用核心候选，不立即删除，不对外发布。

## 风险

- 受控自然语言可能“看起来自然、实际暗含语法陷阱”；
- 中文分词、同义表达、否定、并列、指代和时间范围会创建多解；
- 若 LLM 直接决定 AST，将破坏 `DEC-004`；
- 旧实现输出的完成度不能沿用为新方向完成度。

## 明确边界

- 本变更不授权立即重写代码；
- 本变更不授权删除旧 DSL；
- 本变更不建立 Baseline，不签发 Formal Seal；
- 首个规范性自然语言 Profile 仍需后续精确裁决。

## 替代关系

```text
OLD_DECISION: DEC-007
OLD_STATUS: SUPERSEDED
SUPERSEDED_BY: DEC-014

NEW_DECISION: DEC-014
NEW_STATUS: APPROVED
SUPERSEDES: DEC-007_AND_PUBLIC_SURFACE_PORTIONS_OF_DEC-012_DEC-013
```
