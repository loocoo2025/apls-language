# HDP-APLS-010 — 受控自然语言公开表面语法决定记录

```yaml
DETERMINATION_ID: HDP-APLS-010
STATUS: APPROVED
OPTION: A
DECISION_DATE: 2026-09-03
AUTHORITY_OWNER: HUMAN_PROJECT_OWNER
DECISION_EVIDENCE:
  - "用户一定不要碰这个，除非他主动想看"
  - "就这么干吧"
  - "就是这个定位"
  - "同意你的建议"
```

## 本次决定

APLS 采用方案 A：

> 以受控自然语言作为唯一公开 Surface Syntax，以严格 Canonical IR 作为底层契约，建立面向人类与 AI 的可编译自然语言。

用户默认只编写和评审受控自然语言。花括号 DSL、Surface AST 和 Canonical IR 不是用户必学界面；只在用户主动请求解释、检查或调试时展示。

## 强制语义边界

- 允许多个已明确规定的自然表达归一为同一语义；
- 禁止一个合法输入对应多个可能语义；
- 不能证明唯一解释时必须拒绝，不得由 AI 选择“最可能”含义；
- AI 可依据结构化诊断建议改写，但修改后的 Source 必须重新进入确定性编译路径。

## 正式后果

- `DEC-014` 成为公开表面语法的当前唯一决定；
- `DEC-007` 被替代；`DEC-012/DEC-013` 中依赖旧 DSL 源码形式的部分不再是当前公开语法规范；
- 建立 `TASK-014` 设计受控自然语言架构、歧义边界和旧产物迁移路由；
- 在新 Grammar 获批前，不重写 Lexer/Parser/AST，不继续 Type/Unit 实现。

## 明确未授权

- 未授权 Commit、Push、Release、Baseline Adoption、Formal C04 或 Formal Seal；
- 未授权使用 LLM 作为规范编译器的语义裁决器；
- 未授权删除旧 DSL 实现；
- 未决定首个规范性自然语言 Profile 是仅简体中文还是首版多语言。

## 可复制历史决定格式

```text
HDP-APLS-010: APPROVED
OPTION: A
```
