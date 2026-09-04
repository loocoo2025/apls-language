# HDP-APLS-013 — CNL 唯一性判定层级

```yaml
determination_id: HDP-APLS-013
status: APPROVED
selected_option: B
decision_date: 2026-09-03
authority: HUMAN_PROJECT_OWNER
what_must_be_decided: 多个词法切分或 Parse 最终归一为同一个 Semantic Frame 时是否允许接受 Source
why_human_authority_is_required: 该选择会改变公开语言接受集、PRD-001、Parser 架构和用户何时必须显式引用
```

## 已确认事实

- APLS 的首要目标是拒绝歧义；
- 合法 Source 最终不得对应两个不同规范含义；
- 裁决前的 `PRD-001` 曾要求合法 Source 产生唯一 AST；该条款现已同步修正；
- 用户允许直接写裸中文术语，但多解时可以使用 `“精确术语”`；
- `HDP-APLS-012` 已批准中文语言契约作为设计输入，但没有裁定其中新发现的内部冲突；
- 具体冲突证据记录在 `RULE-GAP-APLS-001`。

## 已裁决问题

唯一性应在每个语法阶段分别成立，还是只要求最后的 Typed Semantic Frame 唯一？

## 选项与差异

### Option A — 每层分别唯一（未采用）

```text
Tokenization Count == 1
Parse Count        == 1
Typed Frame Count  == 1
```

任何一层出现多个完整候选都拒绝。裸术语切分多解时，用户使用 `“精确术语”` 消歧。

优点：最符合“拒绝歧义”和 `PRD-001`；Parser 边界清楚；诊断稳定；不允许类型检查器掩盖语法多解。

代价：少数语义上最终相同的句子仍会被拒绝，需要用户补一对引号。

### Option B — 只要求最终 Canonical Frame 唯一（已采用）

允许多个 Tokenization/Parse 继续到类型化阶段；只要全部有效候选按完整语义去重后剩一个 Frame 就接受。

优点：可接受更多自然表达，用户更少需要引号。

代价：合法 Source 可以有多个 AST，与裁决前的 `PRD-001` 冲突；Parser、资源上限和诊断复杂度显著增加；PRD 和歧义边界现已同步修改。

### Option C — 行为句所有术语都必须加引号

不允许裸术语引用，从语法上消除中文分词多解。

优点：实现最简单、最确定。

代价：用户会反复书写引号，明显削弱“表面就是自然语言”的核心体验，不推荐。

## 风险与权衡

- Option A 优先形式唯一性，保留自然书写并提供局部显式消歧；
- Option B 优先接受范围，但把歧义处理推迟到类型语义阶段；
- Option C 最保守，却最偏离产品定位；
- 无论选择哪项，AI 都不能代替用户选择术语含义。

## 原建议与最终裁决

本 Package 原建议为 `OPTION A`。Human Project Owner 已明确裁决采用 `OPTION B`；该裁决优先于原建议，并成为语言合法 Source 集合的规范性契约。

最终规则是：表层分析允许多个词法切分、Parse Tree 或其他候选路径；所有仍有效候选经过语义绑定、消歧和规范化后，必须收敛到唯一的 `Canonical Frame` 等价类。中间候选多解本身不是错误。

```text
Canonical Frame 等价类数量 == 0  -> INVALID
Canonical Frame 等价类数量 == 1  -> ACCEPTED
Canonical Frame 等价类数量 >= 2  -> AMBIGUOUS / REJECTED
```

不得通过任意优先级、概率最高候选、LLM 猜测或选择第一个 Parse 制造唯一性。唯一性必须来自全部有效分析路径按规范规则可证明地语义收敛；无法证明时由人补充 Source。

## 决定后果

- `APPROVED / OPTION A`：修正 Profile 第 10 节和 Frame 去重边界，使每层候选数量必须为 1；允许继续单 Token Stream Parser 技术适配；
- `APPROVED / OPTION B`：同步修改 `PRD-001`、歧义边界、Profile、Grammar、诊断和 Parser 资源模型，再重新形成语言契约 Target；
- `APPROVED / OPTION C`：移除裸术语引用并修改目标体验与 Grammar；
- `CHANGES_REQUESTED`：按项目负责人明确意见修订本 Package；
- `DEFERRED`：`TASK-016` 保持阻塞；
- `REJECTED`：重新提出不超出已批准产品目标的唯一性方案。

## 明确未授权

- 不授权 Compiler Source、测试或依赖修改；
- 不授权改变 Rust/LALRPOP 决定；
- 不授权 Commit、Push、Baseline Adoption、Formal C04、Formal Seal 或 Release；
- 不授权 AI 自动给多解术语加引号或选择候选。

## 批准记录

Human Project Owner 于 `2026-09-03` 作出以下高精度裁决：

> 采用“最终语义唯一”，不采用“各中间阶段均必须唯一”。表层分析允许多候选，规范语义必须唯一。若多个候选最终得到结构和语义等价的同一 Canonical Frame，则源码合法；若最终存在两个或以上不等价 Canonical Frame，则判定为 `AMBIGUOUS` 并拒绝。不得以任意优先级、最高概率、LLM 猜测或第一个 Parse 制造唯一性。

## 权威来源

- `DEC-004`、`DEC-014`、`DEC-016`；
- `PRD-001`、`PRD-002`、`PRD-009`；
- `RULE-GAP-APLS-001`；
- `DES-APLS-CNL-AMB-001`、`DES-APLS-ZH-CNL-001`、`GRAM-APLS-ZH-CNL-001`、`DES-APLS-CNL-FRAME-001`。

## 可复制回复格式

```text
HDP-APLS-013: APPROVED
OPTION: B
```
