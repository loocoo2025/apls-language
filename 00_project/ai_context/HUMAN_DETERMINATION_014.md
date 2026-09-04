# HDP-APLS-014 — CNL Frontend 适配与 IR Schema 闭合路线

```yaml
determination_id: HDP-APLS-014
status: APPROVED
selected_option: A
decision_date: 2026-09-03
authority: HUMAN_PROJECT_OWNER
what_must_be_decided: 是否批准保留 Rust/LALRPOP、增加候选分析与 Canonical Frame 收敛层，并在代码迁移前先修订不兼容的 Canonical IR Schema
why_human_authority_is_required: 该选择冻结 Frontend 组件边界和下一工作包顺序，并决定是否继续沿用 DEC-010 技术栈与现有 IR Schema
```

## 已确认事实

- `HDP-APLS-013 Option B` / `DEC-017` 已规定最终 Canonical Frame 唯一，中间分析可以多候选；
- LALRPOP 可以继续解析单条候选 Token Stream，零 Grammar 冲突 Gate 仍有效；
- 在 LALRPOP 前增加确定性 Candidate Lexical Lattice，并逐候选调用同一 Parser，不需要新增依赖；
- 现有 `apls-ir-0.1.schema.json` 无法无损表达 Entity Kind、Property、Action Target、Rule Modality、一般 Condition Trigger、Invariant Assertion、中文 Display Name 和派生节点稳定 ID；
- 因此当前不能把 CNL Canonical Frame 直接发布为 Verified IR。

## 选项

### Option A — 保留技术栈，新增候选层，先闭合 IR Schema（推荐）

- 保留 Rust + LALRPOP；
- LALRPOP 对每条候选 Token Stream 使用同一正式 Grammar；
- 新增 Candidate Lattice、Binder/Type Validator、Frame Normalizer 和 Convergence Gate；
- 下一工作包先修订 CNL Canonical IR 模型、Schema 和稳定 ID 契约；
- Schema 获批前，C03 最远只实现到唯一 Canonical Frame，不接 IR Publisher。

该方案满足 `DEC-010` 和 `DEC-017`，变化最小，且不允许有损 IR。

### Option B — 重新选择 Parser 技术

用支持共享 Parse Forest/歧义 Grammar 的其他 Parser 路线替代 LALRPOP，并重新评估依赖、构建、诊断和供应链。

该方案可能减少重复解析，但会替代 `DEC-010/DEC-011` 的部分决定；目前没有证据表明首版必须付出这项成本。

### Option C — 暂停 Frontend，先重新设计全部 CNL 与 IR

不进入迁移工作包，重新打开已经批准的 CNL Profile 边界。

该方案风险最低但范围最大；当前已识别问题可以通过 Option A 的受控 Schema 工作包关闭，无需重开全部语言设计。

## 推荐

推荐 `OPTION A`。它把“中间多候选”和“Parser Generator 冲突”明确分离，保留已批准技术栈，同时拒绝在不兼容 Schema 上发布有损 Verified IR。

## 批准记录

Human Project Owner 通过对“批准 `HDP-APLS-014`”的明确续行指令，于 `2026-09-03` 批准推荐的 `OPTION A`。

## 批准后果

- `APPROVED / OPTION A`：建立下一 C02 工作包，先冻结 CNL IR Schema、稳定 Semantic ID 和 Frame-to-IR 映射；之后再提交最小 C03 实现授权；
- `APPROVED / OPTION B`：创建替代 `DEC-010` 的新 ADR 和技术/依赖评估，代码继续暂停；
- `APPROVED / OPTION C`：重新打开 CNL/IR 架构范围，`TASK-016` 输出不进入后续设计；
- `CHANGES_REQUESTED`：按负责人意见修订；
- `DEFERRED`：Frontend 与 IR 迁移保持暂停。

## 明确未授权

- 不授权 Compiler Source、测试、依赖或 Schema 修改；
- 不授权 Commit、Push、Baseline Adoption、Formal C04、Release 或 Formal Seal；
- 不授权概率分词、LLM 语义裁决、最长匹配或第一成功候选策略。

## 可复制回复格式

```text
HDP-APLS-014: APPROVED
OPTION: A
```
