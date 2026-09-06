# 产品验收标准

- 状态：`CONFIRMED`
- 建立依据：`DEC-031`（`HDP-APLS-024` Q4=A，F-08 整改）；AC-001～AC-009 提炼自 `PRD.md` PRD-001～009 既有验收段，不新增 PRD 未表述的需求；AC-007 验收措辞随 `HDP-APLS-024` Q2=B 与失败关闭设计一致。

| ID | 对应需求 | 验收条件/阈值 | 单位与适用环境 | 测量边界（开始/结束、包含/排除） | 实际意义与依据 | 验证方法 | 状态 |
|---|---|---|---|---|---|---|---|
| AC-001 | PRD-001 | 同一输入的全部有效分析候选被完整、确定地处理；最终 Canonical Frame 等价类基数恰好为 1，且重复执行得到结构相同的 Canonical Frame | 无量纲；适用于全部合法 `apls-zh-CN-0.1` Source | 自 Source 读取开始，至 Canonical Frame 收敛 Gate 判定结束；包含全部中间 Tokenization/Parse/绑定候选，排除编译事务外的 AI 辅助 | 首要成功标准（唯一规范化解释）；DEC-017 唯一性判定层级 | Conformance 正向样例 + 重复执行与顺序扰动下结构比较（CNL-C001、CNL-C015） | CONFIRMED |
| AC-002 | PRD-002 | 每个最终语义多解样例均返回非零结果、`AMBIGUOUS` 稳定诊断代码、精确源码范围和至少两个分歧见证；仅中间分析多候选但最终收敛的样例必须成功 | 无量纲；适用于全部输入 Source | 自候选分析开始，至收敛 Gate 输出 `APLS-E1310` 或唯一等价类结束；零有效候选属非法输入，不计入歧义 | DEC-004 首要质量目标：拒绝歧义、禁止 AI 静默猜测 | Conformance 歧义负向样例与收敛正向样例（CNL-C004、CNL-C011） | CONFIRMED |
| AC-003 | PRD-003 | 语义等价输入生成字节级稳定或经规范化比较相等的 Canonical IR；空白、注释和允许的表面表示差异不改变规范化语义 | Byte 级；适用于全部通过检查的 Source | 自通过检查开始，至 Verified IR 发布结束；包含 Canonicalization 与复验，排除发布后的下游消费 | PRD-003 确定的 Canonical IR；DEC-019 IR 身份与 Canonicalization | Conformance 字节稳定性与 Schema/Cross-validation 样例（CNL-C010、CNL-C014） | CONFIRMED |
| AC-004 | PRD-004 | 未定义名称、重复定义、类型不匹配、单位不匹配和非法循环依赖的负向样例不能进入 IR 生成阶段 | 无量纲；适用于全部输入 Source | 自名称绑定开始，至类型/单位检查结束；任一失败即失败关闭，不构造下游 Artifact | PRD-004 完整名称与类型检查；跨节点引用必须显式解析 | Conformance 名称/类型/单位矩阵负向样例（CNL-C002、CNL-C003） | CONFIRMED |
| AC-005 | PRD-005 | 状态转换冲突和规则冲突均有稳定诊断；尚未定义冲突解决规则时拒绝而不是选择任意结果 | 无量纲；适用于含 Rule/Transition 的 Source | 自语义验证开始，至冲突检查结束；0.1 冻结最小直接冲突模型（同 Canonical Condition + 同行为三元组的 REQUIRE×PROHIBIT 以 `APLS-E1405` 拒绝；Transition 冲突以 `APLS-E1402/E1403/E1404` 拒绝），更一般冲突按未定义即拒绝处理 | PRD-005 语义冲突检测；`system_architecture.md` §5.5；DEC-031（HDP-APLS-024 Q1=A） | Conformance 冲突负向样例（CNL-C005、CNL-C006、CNL-C017、CNL-C018） | CONFIRMED |
| AC-006 | PRD-006 | 所有 Conformance 负向样例都能定位到最小相关源码范围；每条错误至少包含诊断代码、严重度、文件、起止位置和原因；可修复建议不在未授权时修改源文件 | 无量纲；适用于全部非法输入 Source | 自首个失败 Stage 开始，至公共诊断总排序与输出结束；按封闭根因聚合与抑制表（诊断目录 §1.1/§1.2） | PRD-006 可操作诊断；`DES-APLS-CNL-DIAG-001` 根因身份合同 | Conformance 根因聚合、截断与排序样例（CNL-C010、CNL-C013、CNL-C016） | CONFIRMED |
| AC-007 | PRD-007 | IR Schema 机械区分 normative 与 informative；unknown 与 open 内容不得进入 Verified IR，由稳定诊断在编译期拒绝并定位 | 无量纲；适用于 Verified IR 输出与全部输入 Source | 自语义验证开始，至 Verified IR 发布结束；unknown/open 在编译期终止事务，不存在于任何 IR Artifact | PRD-007 Agent 安全消费（验收措辞经 DEC-031 / HDP-APLS-024 Q2=B 修订，与失败关闭设计一致） | IR Schema 正负向样例与 Cross-validation（CNL-C014） | CONFIRMED |
| AC-008 | PRD-008 | 工具链支持 Parse、Check、Emit IR、Diagnose 四项逻辑能力；0.1 CLI 命令表面为 `apls parse` / `apls check` / `apls emit-ir` / `apls diagnose` 与 `--version` 固定行，退出码 0/1/2/3 边界稳定 | 无量纲；适用于 CLI 进程调用 | 自进程启动开始，至退出码与 stdout/stderr 输出结束；stdout 仅承载有效 Artifact，诊断走 stderr/Envelope | PRD-008 编译器式最小工作流；`DES-APLS-COMPILER-001` 命令与发布契约 | CLI 黑盒测试（命令表面、退出码、`--version` 固定行） | CONFIRMED |
| AC-009 | PRD-009 | 每个合法句子最终产生恰好一个 Canonical Frame 等价类；多种中间 Tokenization/Parse 可收敛为同一规范含义；同义句组产生同一 Canonical Meaning；模糊量、未定义术语、多指代和最终作用域多解样例全部失败关闭 | 无量纲；适用于 `apls-zh-CN-0.1` 公开 Source | 自词法候选生成开始，至收敛 Gate 判定结束；唯一公开规范性 Source 为版本化简体中文 CNL Profile | PRD-009 受控自然语言公开界面；DEC-014、DEC-015、DEC-017 | Conformance 句式正向样例与失败关闭负向样例（CNL-C001、CNL-C003、CNL-C009） | CONFIRMED |

需要负责人批准的阈值必须说明替代值、成本/风险影响，以及批准后改变该值所需的影响分析和重新批准方式。
