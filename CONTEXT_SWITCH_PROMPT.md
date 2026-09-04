# 上下文切换指令

```text
当前对话准备切换到下一版本。停止接受新的大型任务。

1. 重要正式决策落盘；
2. 更新 CURRENT_STATE；
3. 更新 ACTIVE_TASKS；
4. 更新 OPEN_QUESTIONS；
5. 更新 DECISION_INDEX；
6. 更新 CONVERSATION_MAP；
7. 创建 HANDOFF；
8. 记录 Git 分支/Commit/未提交修改；
9. 记录测试结果；
10. 记录未完成、风险、workaround；
11. 明确新对话第一项任务；
12. 当前对话进入 READ ONLY。
```
