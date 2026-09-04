# 可选的 Codex Harness 启动配置

本目录为通过 Codex Harness 使用长程智构的用户提供保守的本地默认配置。它是可选的 Harness 适配层，不是第二套项目模板或治理来源。

长程智构的治理基线位于仓库根目录。始终以根目录的 `AI_START_HERE.md`、工程规则、项目文件和角色简报作为项目权威来源。

## 可选的一次性本地设置

复制前请先检查文件：

- `global_codex_home/AGENTS.md` → `~/.codex/AGENTS.md`
- `global_codex_home/config.toml` → `~/.codex/config.toml`

随附配置把写入范围限制在当前工作区内，默认禁用网络访问，并在执行更广泛的操作前请求批准。

不要直接用这些文件覆盖现有的个人 Codex 配置。只合并你明确希望采用的设置。

## 项目设置

安装 Full 或 Lite 项目时，请返回根目录的 [README](../README.md) 和[快速开始](../docs/QUICK_START.md)。选择何种 Model 或 Harness 不会改变 Current Truth，也不会改变 C00～C06 的角色权限。
