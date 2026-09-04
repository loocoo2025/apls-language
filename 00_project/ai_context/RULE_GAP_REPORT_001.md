# RULE GAP REPORT 001 — CNL 唯一性判定层级冲突

```yaml
report_id: RULE-GAP-APLS-001
status: CLOSED_BY_DEC_017
classification: RULE_CONFLICT
priority: P1
owner_required: HUMAN_PROJECT_OWNER
blocks: NONE
```

## 问题

同一句 Source 存在多个词法切分或多个 Parse，但这些候选在类型检查后产生同一个 Semantic Frame 时，是否仍必须拒绝？

## 冲突证据

裁决前的严格阶段唯一规则（现已由 `DEC-017` 失效）：

- `PRD-001` 要求 Parser 对合法 Source 产生唯一 AST；
- `DES-APLS-CNL-AMB-001` 要求 `lexical_tokenization_count == 1`、`grammar_parse_count == 1`、`semantic_frame_count == 1`；
- `DES-APLS-ZH-CNL-001` 第 3.3 节要求只有一个完整 Token 序列和一个完整 Frame 才接受；
- `GRAM-APLS-ZH-CNL-001` 附加约束 8 声明 Parse/Tokenization Tie 是 Error。

裁决前另一组允许 Frame 层归一的规则（现已成为 `DEC-017` 基础）：

- `DES-APLS-ZH-CNL-001` 第 10 节要求枚举 Token 序列、构造候选 Frame、按完整结构去重，并只按“剩余 Frame 数量恰好为 1”决定接受；
- `GRAM-APLS-ZH-CNL-001` 的术语说明同样使用“恰好一个完整 Typed Frame”作为最终门槛；
- `DES-APLS-CNL-FRAME-001` 第 7 节定义候选 Frame 的语义去重条件。

两组规则对同一 Source 可能给出不同接受结果，无法同时实现。

## 已检索范围

- `PRD-001/002/009`；
- `ARCH-APLS-CNL-001`；
- `DES-APLS-CNL-AMB-001`；
- `DES-APLS-ZH-CNL-001`；
- `GRAM-APLS-ZH-CNL-001`；
- `DES-APLS-CNL-FRAME-001`；
- `DES-APLS-CNL-DIAG-001`；
- `DEC-004`、`DEC-014`、`DEC-016`。

## 受影响动作

- 无法确定 Terminology Indexer 是输出唯一 Token Stream，还是输出 Tokenization Lattice；
- 无法评估现有 LALRPOP 是单次解析、候选流多次解析，还是需要其他 Parser；
- 无法冻结 `APLS-E1102` 与 `APLS-E1203` 的触发边界；
- 无法建立准确的 CNL-to-IR Conformance 接受集。

## 当前阻断

历史上，`TASK-016` 停止在技术适配与代码迁移之前。该阻断已由 `HDP-APLS-013 Option B` / `DEC-017` 关闭。

## 原建议

建议采用严格阶段唯一：Tokenization、Parse 和 Typed Frame 三层分别必须恰好一个。它直接符合 `PRD-001` 和“拒绝歧义”首要目标；当裸术语切分不唯一时，用户只需给该引用加中文引号，不需要查看 AST 或 IR。

该原建议未被采用，仅作为裁决前分析记录保留。

## 裁决结果

Human Project Owner 采用 `HDP-APLS-013 Option B`：中间 Tokenization、Parse Tree 和分析路径允许多候选；全部有效候选经绑定、检查与规范化后，必须只形成一个 `Canonical Frame` 等价类。零等价类是 `INVALID`，两个及以上不等价类是 `AMBIGUOUS`。实现不得选择最高概率、第一个或任意优先候选，也不得让 LLM 代替证明。

所有冲突条款已按 `DEC-017` 统一；`TASK-016` 可以恢复设计工作。
