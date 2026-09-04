# HDP-APLS-011 — APLS 0.1 首个规范性自然语言 Profile

```yaml
determination_id: HDP-APLS-011
status: APPROVED
option: A
decision_date: 2026-09-03
authority_owner: HUMAN_PROJECT_OWNER
what_must_be_decided: APLS_0_1_FIRST_NORMATIVE_HUMAN_LANGUAGE_PROFILE
why_human_authority_is_required: 该决定改变公开语言接受集、Grammar、诊断和用户范围
```

## 已确认事实

- `HDP-APLS-010 / Option A` 已决定受控自然语言是唯一公开 Surface Syntax；
- 用户默认不接触 DSL、AST 或 IR；
- 所有被接受的句子必须唯一映射为 Semantic Frame；
- LLM 可帮助改写，但不能替规范编译器裁决含义；
- 当前说明性目标样例使用简体中文。

## 待决定选项

### Option A — 简体中文单语 Profile（推荐）

- APLS 0.1 只冻结一个规范性简体中文 Profile；
- 术语声明使用中文定界符建立精确名称；
- 规范句优先保持自然中文，只在术语切分或引用多解时要求显式定界；
- 初版禁止未冻结指代、跨句省略和多义务并列；
- 其他自然语言以后作为独立、版本化 Profile，不与中文 Grammar 混合判定。

优点：范围可控，能快速建立唯一分词、句法、诊断和 Conformance 套件。

代价：0.1 首版不直接接受英文或混合自然句子。

### Option B — 中英双语 Profile

- 0.1 同时冻结中文和英文句式；
- 两套 Surface 必须映射到相同 Semantic Frame 和 Canonical Meaning；
- 混合语句的词法、术语和单位边界必须另行冻结。

优点：首版覆盖更多用户和 Agent 输入。

代价：Grammar、等价性证明、诊断和测试面显著扩大，更容易在早期引入语义不一致。

### Option C — 首版多语言通用 Profile

- 试图用单一通用规则接受多种自然语言；
- 依赖语言检测、多语分词和跨语言同义映射。

优点：理论用户面最广。

代价：与当前“首先证明唯一性”目标不匹配，实现和 Conformance 风险最高。

## 推荐

`OPTION A`。

理由：APLS 首要目标是拒绝歧义，不是首版覆盖所有自然语言。先在简体中文上建立可证明的受控句式、术语系统和失败诊断，再以独立 Profile 扩展其他语言，不会污染规范内核。

## 项目负责人决定

```text
HDP-APLS-011: APPROVED
OPTION: A
```

决定证据：项目负责人明确回复上述批准格式，并确认采纳“首版只支持简体中文，先证明分词、句法和语义唯一性，再扩展英文”的建议。

## 决定后果

- `APPROVED / OPTION A`：允许 C02 建立简体中文 CNL 精确词法、Grammar、Semantic Frame 与诊断设计工作包；
- `APPROVED / OPTION B`：相同工作包必须同时包含中英等价性证明；
- `APPROVED / OPTION C`：必须先建立多语唯一性可行性研究，不直接进入 Grammar；
- `CHANGES_REQUESTED`：修订本 Package，当前架构候选保持；
- `DEFERRED`：不设计具体 CNL Grammar；
- `REJECTED`：重新打开公开语言 Profile 方向。

## 明确未授权

- 不授权代码重写、依赖变更、Commit、Push、Baseline Adoption、C04、Formal Seal 或 Release。

## 权威证据

- `HDP-APLS-010`
- `DEC-014`
- `ARCH-APLS-CNL-001`
- `DES-APLS-CNL-AMB-001`

## 可复制回复格式

```text
HDP-APLS-011: APPROVED
OPTION: A
```
