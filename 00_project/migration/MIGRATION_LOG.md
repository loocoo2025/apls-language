# MIGRATION_LOG.md
## 迁移/治理历史事件日志（Append Only）

> 新项目通常为空；接管旧模块、治理迁移或重大规范化事件时使用。
>
> **本文件是历史事件日志，不是当前状态文件。**
>
> 只能追加，不得为了同步“现在”而回头改写历史记录。

---

# 使用规则

1. 每条记录描述“某个时点发生了什么”；
2. 历史状态即使后来失效，也保留原文并通过后续新记录说明变化；
3. 不得在日志顶部维护“当前 R04 状态”“当前 OPEN finding”“当前下一步”等动态摘要；
4. 当前项目状态统一见 `00_project/ai_context/CURRENT_STATE.md`；
5. 当前 Baseline 统一见 `BASELINE_INDEX.md`；
6. 当前决定统一见 `DECISION_INDEX.md`。

---

# 记录模板

```text
## {{DATE}} / {{EVENT_ID}}

Event Type:
MIGRATION / GOVERNANCE / BASELINE / REVIEW / OTHER

Git Commit:
{{COMMIT_OR_NA}}

事实：
- 

影响：
- 

后续：
- 
```

> 历史日志中的“当时状态”不得被解释为当前状态；新 AI 必须先读 `CURRENT_STATE.md`。
