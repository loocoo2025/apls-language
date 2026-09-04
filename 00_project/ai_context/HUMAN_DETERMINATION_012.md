# HDP-APLS-012 — 简体中文 CNL 首版语言契约批准

```yaml
determination_id: HDP-APLS-012
status: APPROVED
option: A
decision_date: 2026-09-03
authority_owner: HUMAN_PROJECT_OWNER
what_must_be_decided: 是否批准简体中文 CNL Profile、Grammar、Semantic Frame 和诊断候选作为 Compiler Frontend 设计输入
why_human_authority_is_required: 该决定冻结首版公开语言接受集、用户写法和拒绝边界
```

## 已确认事实

- `DEC-014`：APLS 是以受控自然语言为唯一公开 Surface、以 Canonical IR 为底层契约的可编译自然语言；
- `DEC-015`：APLS 0.1 采用简体中文单语 Profile；
- 用户默认不查看 DSL、AST 或 IR；
- 任一输入若不能形成唯一完整语义必须拒绝；
- AI 只可在编译事务外解释和建议改写。

## 本次批准 Target

1. `04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md`；
2. `04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf`；
3. `04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md`；
4. `04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md`。

## 候选契约摘要

- 首句显式声明 `APLS 简体中文语言版本 0.1`；
- 术语只在声明处必须使用中文双引号，行为句在唯一可切分时可直接自然书写；
- 多种已批准条件引导句式可以归一到同一个 `Rule` Frame；
- `必须` 表示义务，`不得/禁止` 表示禁止；
- 首版只接受平铺 `并且`，拒绝 `或者`、未冻结指代、跨句省略、一般否定和一个句子多个主行为；
- 模糊阈值、程度、对象、单位、时限和术语全部失败关闭；
- 编译器枚举全部合法切分/Parse/Typed Frame，最终数量不等于 1 就拒绝；禁止最长匹配、第一候选或概率裁决。

## 关键用户体验

以下三句被候选设计明确为等价：

```text
当水箱液位低于 20% 时，系统必须启动灌溉水泵。
如果水箱液位低于 20%，系统必须启动灌溉水泵。
水箱液位低于 20% 时，系统必须启动灌溉水泵。
```

以下输入必须拒绝：

```text
温度高的时候适当降低一点速度。
```

诊断必须指出阈值、变化量、属性所属和动作目标不完整，不得替用户猜测。

## 选项

### Option A — 批准当前四份候选（推荐）

把四份文档作为 Compiler Frontend 技术适配和实现计划的受控输入。随后先验证 Grammar 技术适配与 Source-to-IR 映射，再决定是否重写 Lexer/Parser/AST。

### Option B — 要求修改

项目负责人指出需要增加、删除或改变的用户句式、术语规则或拒绝边界。当前 Target 保持候选，不进入实现规划。

### Option C — 暂缓

保留 `DEC-015`，但不冻结具体句式，Compiler Frontend 迁移保持暂停。

## 风险与权衡

- 强制术语声明会增加少量前置书写，但使裸中文术语可以被机械消歧；
- 首版不支持 `或者`、指代和并列行为，表达范围较窄，但显著降低作用域歧义；
- Unicode NFC 与固定中文标点提高确定性，但需要后续实现评估 Unicode 依赖；
- 动态术语 Tokenization 未证明可直接由现有 LALRPOP Frontend 实现，因此批准语言契约不等于批准现有 Parser 直接复用。

## 推荐

推荐 `OPTION A`。它已实现用户要求的自然中文体验，同时将真正难以唯一解释的句子明确拒绝。后续可以用版本化 Profile 扩展更多句式，而不让 0.1 Compiler 猜测。

## 项目负责人决定

```text
HDP-APLS-012: APPROVED
OPTION: A
```

该决定精确批准“本次批准 Target”中的四份候选作为后续 Frontend 设计输入，不扩大到代码、依赖、Baseline 或发布动作。

## 决定后果

- `APPROVED / OPTION A`：形成新决定，允许 C02 评估 Parser 技术适配、建立 Source-to-IR 映射和 C03 迁移工作包候选；不自动授权代码修改；
- `APPROVED / OPTION B`：按明确修改意见修订同一 Target；
- `APPROVED / OPTION C`：保持设计暂停；
- `CHANGES_REQUESTED`：等同 Option B；
- `DEFERRED`：等同 Option C；
- `REJECTED`：重新打开具体中文语言契约，不推翻 `DEC-014/015`，除非项目负责人另行明确改变方向。

## 明确未授权

- 不授权 Compiler Source 或测试代码修改；
- 不授权依赖变更、安装或 Parser 技术替换；
- 不授权 Commit、Push、Baseline Adoption、Formal C04、Formal Seal 或 Release；
- 不批准旧 DSL 作为第二条公开 Source 路径。

## 权威来源

- `HDP-APLS-010` / `DEC-014`；
- `HDP-APLS-011` / `DEC-015`；
- `PRD-001`、`PRD-002`、`PRD-006`、`PRD-009`；
- `ARCH-APLS-CNL-001`、`DES-APLS-CNL-AMB-001`。

## 可复制回复格式

```text
HDP-APLS-012: APPROVED
OPTION: A
```
