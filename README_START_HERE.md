# 长程智构：从这里开始

本文档是项目采用“长程智构｜AI 软件工程治理系统”时面向操作者的入口。

如需五分钟概览，请阅读 [README.md](README.md)；如需具体执行步骤，请阅读 [docs/QUICK_START.md](docs/QUICK_START.md)。

## 选择采用方式

### Full Template

如果项目周期较长、对安全或质量敏感、涉及多学科、属于旧项目，或要求正式追溯和发布证据，请使用完整仓库。

1. 复制或解压完整模板。
2. 替换项目占位符。
3. 初始化或核验 Git。
4. 配置 `00_project/ai_context/CURRENT_STATE.md`。
5. 使用 `PROJECT_START_PROMPT.md` 启动 Agent。

### Lite

对于需要治理、但不需要完整生命周期目录树的小型或低风险项目，可使用 Lite 文件集。准确的文件清单和升级触发条件见 [docs/FULL_VS_LITE.md](docs/FULL_VS_LITE.md)。

Lite 保留以下不可弱化的原则：

- 当前事实（Current Truth）；
- 一个事实，一个所有者（One Fact, One Owner）；
- Role != Model != Runtime != Harness != Session != Tool；
- 独立 C04 评审；
- 明确的人类权限边界；
- Dynamic Role Profile、标准 Interaction 与精确 Authorization；
- 上下文、Model、Runtime 或 Harness 替换后的 Knowledge Continuation / Baseline Relearn。

## Agent 首次必读入口

首先完整阅读 `AI_START_HERE.md`，再严格遵循它维护的权威启动顺序。本说明不维护另一份可能漂移的阅读清单。

新项目从 C00/C01 开始。已有项目先按 `AI_LEGACY_PROJECT_STANDARDIZATION_GUIDE.md` 盘点，再移动或改写文件。

重要决策必须写入其正式事实所有者。聊天历史不是持久的项目事实来源。
