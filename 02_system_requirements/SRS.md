# SRS — 系统需求规格说明书

状态：CONFIRMED
版本：0.1
建立依据：`DEC-031`（`HDP-APLS-024` Q4=A，F-08 整改）；本 SRS 登记最小完备需求集，详细条目见 `functional_requirements.md`、`nonfunctional_requirements.md`、`interface_requirements.md`，追溯边见 `requirements_traceability.md`。

## 系统范围
- APLS 0.1 编译器式工具链：以受控自然语言（`apls-zh-CN-0.1`）为唯一公开 Source，经候选分析、唯一 Canonical Frame 收敛、检查与复验，交付 Verified Canonical IR（`apls-cnl-ir-0.1`）或稳定诊断。
## 系统上下文
- 用户（架构师、AI Coding Agent、验证工程师、工具开发者）经 CLI 调用；AI 可在编译事务外辅助，但不得替编译器确定规范语义。
## 功能需求

| ID | 来源 | 要求 | 验收标准 |
|---|---|---|---|
| SYS-001 | PRD-001 | 全部有效分析候选完整、确定地收敛为恰好一个 Canonical Frame 等价类 | 等价类基数为 1，重复执行结构相同 |
| SYS-002 | PRD-002 | 最终语义多解必须拒绝 | 非零结果、`AMBIGUOUS` 稳定码、精确范围、至少两个分歧见证；中间多候选最终收敛者必须成功 |
| SYS-003 | PRD-003 | 通过检查的程序生成唯一规范化 IR | 语义等价输入生成字节级稳定或规范化比较相等的 IR |
| SYS-004 | PRD-004 | 完整名称、类型、单位与循环依赖检查 | 负向样例不能进入 IR 生成阶段 |
| SYS-005 | PRD-005 | 语义冲突检测，未定义解决规则时拒绝 | 状态转换冲突和规则冲突均有稳定诊断 |
| SYS-006 | PRD-006 | 可操作诊断，建议不擅自改源文件 | 所有 Conformance 负向样例定位到最小相关源码范围 |
| SYS-007 | PRD-007 | Agent 默认消费已验证 Canonical IR | IR Schema 机械区分 normative 与 informative；unknown/open 编译期以稳定诊断拒绝、不进入 IR |
| SYS-008 | PRD-008 | 编译器式最小工作流 Parse/Check/Emit IR/Diagnose | 四项能力经 CLI 可用，命令表面与退出码符合已批准契约 |
| SYS-009 | PRD-009 | 受控自然语言唯一公开界面（0.1 简体中文单语 Profile） | 合法句子唯一收敛；同义句组同一 Canonical Meaning；模糊/未定义/多解样例全部失败关闭 |

各条目详细字段（前置、正常/异常流程、边界）见 `functional_requirements.md`。

## 非功能需求

| ID | 类别 | 可量化要求 | 验证方法 |
|---|---|---|---|
| NFR-001 | 确定性 | 输出不依赖随机数、模型温度或执行顺序 | CNL-C010/C015/C016 重复执行与顺序扰动 |
| NFR-002 | 可复现 | 相同工具版本和输入产生相同规范化结果 | 重复执行比较 + 候选 Commit 复现 |
| NFR-003 | 可演进 | 语言与 IR 均具显式版本 | 版本字段核验（CNL-C014、`--version`） |
| NFR-004 | 可诊断性 | 失败关闭，不产生部分可信结果 | Conformance 负向样例 + 发布契约核验 |
| NFR-005 | 可移植性 | 核心语义不绑定特定 LLM 或实现语言 | 契约一致性评审 |
| NFR-006 | 可测试性 | 每条规范性规则可映射到正向或负向一致性样例 | CNL-C001～C020 追溯核对 |

## 接口需求
见 `interface_requirements.md`（IF-001 CLI 命令表面、IF-002 Diagnostic Envelope、IF-003 Verified IR 消费契约）。
## 故障与恢复
- 任一 Error/Tool Failure 存在即失败关闭，不发布 Verified IR；同一编译事务不自动重试。
## 数据与持久化
- Source 为唯一持久化输入；IR 交付经原子发布；无其他持久化状态。
## 安全
- 不执行 Source 内容；不在未授权时修改源文件；符号链接与路径穿越拒绝。
## 可观测性
- 诊断经稳定代码 + 源码范围输出（Diagnostic Envelope / 人读渲染）。
## 未决问题
- 无阻断项；产品 Baseline Adoption 仍须另行裁决（不随需求层建立自动发生）。
