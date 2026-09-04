# AI 中途变更模板使用说明

## 推荐位置

```text
13_change_management/
└── templates/
    ├── CHANGE_CLASSIFICATION_HEADER_TEMPLATE.md
    ├── DISCUSSION_ONLY_TEMPLATE.md
    ├── FORMAL_CHANGE_DECISION_TEMPLATE.md
    ├── IMPLEMENTATION_BUG_FIX_TEMPLATE.md
    └── GOVERNANCE_TEMPLATE_UPGRADE_TEMPLATE.md
```

实际变更记录建议放：

```text
13_change_management/
├── changes/
├── decisions/
├── impact_analysis/
├── baselines/
└── templates/
```

Bug 仍放 `12_issues/`。

## 使用顺序

```text
任何中途变化
↓
先用 CHANGE_CLASSIFICATION_HEADER_TEMPLATE
↓
只是讨论？
→ DISCUSSION_ONLY_TEMPLATE

已经正式改变需求/架构/设计/接口？
→ FORMAL_CHANGE_DECISION_TEMPLATE

只是代码没按当前设计实现？
→ IMPLEMENTATION_BUG_FIX_TEMPLATE

只是工程治理模板升级？
→ GOVERNANCE_TEMPLATE_UPGRADE_TEMPLATE
```

## 统一规则

```text
讨论
→ 不改变 Current Truth

正式决定
→ 新决定落盘
→ 旧决定 SUPERSEDED
→ 从正确层级向下修改

实现 Bug
→ 修代码 + 回归测试
→ 不改变正式需求/架构

治理模板升级
→ 只迁移工程治理
→ 不自动改变产品行为
```

这些模板必须服从现有的：
- `AI_ENGINEERING_RULES_V2.md`
- `DECISION_INDEX.md`
- `CURRENT_STATE.md`
- `BASELINE_INDEX.md`
- `AI_CONTEXT_RESET_AND_BASELINE_RELEARN_RULES.md`
- `AI_TESTING_GOVERNANCE_RULES.md`

模板只负责把“中途变化”执行得机械、明确、低歧义，不覆盖上层治理规则。
