# 非功能需求

- 状态：`CONFIRMED`
- 建立依据：`DEC-031`（`HDP-APLS-024` Q4=A、Q4-a=否，F-08 整改）；NFR-001～NFR-006 派生自 `PRD.md` §6 六条非功能要求。PRD §6 无独立编号，各行 Traces-From 使用最直接相关的 P0 需求 ID 并注明源自 PRD §6。

| ID | 类别 | 可量化要求/阈值 | 单位与适用环境 | 测量边界（开始/结束、包含/排除） | 实际意义与推荐依据 | 验证方法 | Traces-From（正式上游追溯） | 状态 |
|---|---|---|---|---|---|---|---|---|
| NFR-001 | 确定性 | 输出不依赖随机数、模型温度或执行顺序；同一输入重复编译结果一致 | 无量纲；适用于全部编译事务与全部运行环境 | 自 Source 读取开始，至 Artifact 发布结束；包含全部中间阶段与诊断排序，排除编译事务外的调用方行为 | 首要质量目标与可审计性的前提；源自 PRD §6 | Conformance 重复执行与枚举顺序扰动用例（CNL-C010、CNL-C015、CNL-C016） | PRD-003（源自 PRD §6） | CONFIRMED |
| NFR-002 | 可复现 | 相同工具版本和输入产生相同规范化结果 | 无量纲；适用于相同 Compiler 版本与相同输入 | 自同一 Source 与同一工具版本开始，至 Canonical Frame / IR Byte 比较结束；排除不同工具版本间的比较 | 跨环境复核与独立评审可重跑的前提；源自 PRD §6 | 重复执行结构/字节比较（CNL-C010）；候选冻结 Commit 可复现 checkout | PRD-001（源自 PRD §6） | CONFIRMED |
| NFR-003 | 可演进 | 语言与 IR 均具有显式版本（`apls-zh-CN-0.1`、`apls-cnl-ir-0.1`），版本变化触发兼容性规则评估 | 版本标识符；适用于语言 Profile、Grammar、IR 与诊断契约 | 自版本声明开始，至消费方版本核验结束；包含 Header 中 `language_version` 与 `ir_schema_version` | 契约演进与下游兼容判定的前提；源自 PRD §6 | Header/Schema 版本字段核验（CNL-C014）；`--version` 固定行 | PRD-009（源自 PRD §6） | CONFIRMED |
| NFR-004 | 可诊断性 | 失败关闭，不产生部分可信结果；任一 Error/Tool Failure 存在时不发布 Verified IR | 无量纲；适用于全部失败路径 | 自首个失败点开始，至编译事务终止结束；排除任何部分 Artifact 输出 | 防止不可信中间结果被 Agent 消费；源自 PRD §6 | Conformance 负向样例全套 + 发布契约核验（原子发布、Exit 非零无交付） | PRD-006（源自 PRD §6） | CONFIRMED |
| NFR-005 | 可移植性 | 核心语义不绑定特定 LLM 或实现语言；规范含义由 Grammar/Profile 机械定义，LLM 不作规范语义裁决器 | 无量纲；适用于语言契约与 Compiler 实现 | 自语言契约定义开始，至参考实现边界结束；排除具体后端与宿主环境适配 | 多实现一致性与供应链独立的前提；源自 PRD §6；DEC-014 | 契约文档与实现一致性评审；机器契约（Schema/诊断码）跨实现可复算 | PRD-009（源自 PRD §6） | CONFIRMED |
| NFR-006 | 可测试性 | 每条规范性规则必须能映射到正向或负向一致性样例 | 用例计数；适用于 CNL Conformance 套件 | 自规范条款开始，至 Conformance 用例登记结束；包含失败关闭路径，排除编译事务外 AI 行为 | 需求—验证闭环与 Baseline 就绪证据的前提；源自 PRD §6 | Conformance 用例覆盖核对（CNL-C001～C020 追溯表） | PRD-002（源自 PRD §6） | CONFIRMED |

阈值批准后如需改变，必须按实际影响范围更新需求、验证、追溯和重新批准；不得为了测试通过而降低阈值。
