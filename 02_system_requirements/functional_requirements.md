# 功能需求

- 状态：`CONFIRMED`
- 建立依据：`DEC-031`（`HDP-APLS-024` Q4=A，F-08 整改）；SYS-001～SYS-009 一对一派生自 PRD-001～009，不新增 PRD 未表述的需求；SYS-007 验收措辞随 `HDP-APLS-024` Q2=B 与失败关闭设计一致。

## SYS-001 — 最终规范语义唯一

- Traces-From（正式上游追溯）：
  - PRD-001
  - AC-001
- 描述：系统应完整、确定地处理同一 Source 的全部有效词法、语法与语义分析候选，使其在名称/语义绑定、类型与单位检查及规范化后收敛为恰好一个结构和语义等价的 Canonical Frame 等价类。
- 前置：输入为符合 `apls-zh-CN-0.1` 的 UTF-8 NFC Source。
- 正常流程：词法候选枚举 → 逐候选 Parse/绑定/类型/规范化 → 收敛 Gate 判定等价类基数为 1 → 进入后续阶段。
- 异常流程：零有效候选按非法输入失败关闭；两个及以上不等价等价类按歧义拒绝（见 SYS-002）。
- 边界：中间候选多解不得单独导致拒绝；不得通过任意优先级、最高概率、LLM 猜测或选择第一个 Parse 制造唯一性。
- 验收：最终 Canonical Frame 等价类基数为 1，且重复执行得到结构相同的 Canonical Frame。
- 状态：CONFIRMED

## SYS-002 — 拒绝歧义

- Traces-From（正式上游追溯）：
  - PRD-002
  - AC-002
- 描述：语义绑定、检查与规范化后仍存在两个或以上不等价 Canonical Frame 时，系统必须拒绝输入。
- 前置：候选分析已完整执行。
- 正常流程：全部有效候选收敛为唯一等价类，编译继续。
- 异常流程：返回非零结果、`AMBIGUOUS` 稳定诊断代码、精确源码范围和至少两个分歧见证。
- 边界：不允许“选择最可能含义”、静默补全或自动改写后继续编译；零个有效 Canonical Frame 属于非法输入，不属于歧义。
- 验收：每个最终语义多解样例均满足上述拒绝与诊断要求；仅中间分析多候选但最终收敛的样例必须成功。
- 状态：CONFIRMED

## SYS-003 — 确定的 Canonical IR

- Traces-From（正式上游追溯）：
  - PRD-003
  - AC-003
- 描述：通过检查的程序必须生成唯一的规范化 IR；与语义无关的空白、注释和允许的表面表示差异不得改变规范化语义。
- 前置：Source 已通过名称、类型与语义检查。
- 正常流程：Canonical Frame → 无损映射 → Canonicalization → 复验 → 发布 Verified IR。
- 异常流程：复验或发布失败时失败关闭，不输出部分 IR。
- 边界：禁止使用内存地址、线程调度、随机 UUID 或系统时间影响 IR 内容。
- 验收：语义等价输入生成字节级稳定或经规范化比较相等的 IR。
- 状态：CONFIRMED

## SYS-004 — 完整名称与类型检查

- Traces-From（正式上游追溯）：
  - PRD-004
  - AC-004
- 描述：所有跨节点引用必须显式解析；未定义名称、重复定义、类型不匹配、单位不匹配和非法循环依赖必须被诊断。
- 前置：候选已完成语法分析。
- 正常流程：名称绑定 → 类型与单位检查全部通过 → 进入规范化。
- 异常流程：任一检查失败即失败关闭，以稳定诊断定位，不进入 IR 生成阶段。
- 边界：检查阶段失败不得构造下游 Artifact。
- 验收：上述负向样例均不能进入 IR 生成阶段。
- 状态：CONFIRMED

## SYS-005 — 语义冲突检测

- Traces-From（正式上游追溯）：
  - PRD-005
  - AC-005
- 描述：状态转换、规则优先级、通信契约和约束之间的冲突必须有明确定义；尚未定义冲突解决规则时，必须拒绝而不是选择任意结果。
- 前置：候选已完成绑定与类型检查。
- 正常流程：无冲突或冲突按已定义规则显式组合，编译继续。
- 异常流程：0.1 冻结最小直接冲突模型——相同 Canonical Condition 且行为三元组相同的 REQUIRE×PROHIBIT 直接冲突以稳定诊断 `APLS-E1405` 拒绝；Transition 冲突以 `APLS-E1402/E1403/E1404` 拒绝；Invariant×Rule、Safety 优先级等更一般冲突在 0.1 未定义，按未定义即拒绝处理。
- 边界：不得依赖声明顺序或任意选择解决冲突。
- 验收：状态转换冲突和规则冲突均有稳定诊断。
- 状态：CONFIRMED

## SYS-006 — 可操作诊断

- Traces-From（正式上游追溯）：
  - PRD-006
  - AC-006
- 描述：每条错误至少包含诊断代码、严重度、文件、起止位置和原因；可修复建议只能作为建议，不能在未授权时修改源文件。
- 前置：编译事务在任一阶段失败。
- 正常流程：不适用（本需求约束失败路径）。
- 异常流程：零有效候选时按封闭根因聚合（候选路径首个失败 Stage Terminal Finding 并集、去重、封闭抑制表）生成根因诊断，按公共总排序输出。
- 边界：诊断 Code 的根因身份在 0.1 内不得漂移。
- 验收：所有 Conformance 负向样例都能定位到最小相关源码范围。
- 状态：CONFIRMED

## SYS-007 — Agent 安全消费

- Traces-From（正式上游追溯）：
  - PRD-007
  - AC-007
- 描述：Agent 默认消费已验证的 Canonical IR，而不是自行解释未经验证的源文本；Verified IR 机械区分规范事实（normative）与说明文本（informative）；unknown 与 open 内容不进入 Verified IR，由稳定诊断在编译期拒绝并定位。
- 前置：Source 已通过全部检查并收敛唯一。
- 正常流程：仅 Exit 0 且完整可验证时交付 Verified IR。
- 异常流程：unknown/open 内容在编译期以稳定诊断拒绝并定位，不产出 IR。
- 边界：未经验证的源文本不构成 Agent 的规范事实来源。
- 验收：IR Schema 机械区分 normative 与 informative；unknown 与 open 内容不得进入 Verified IR，由稳定诊断在编译期拒绝并定位。
- 状态：CONFIRMED

## SYS-008 — 编译器式最小工作流

- Traces-From（正式上游追溯）：
  - PRD-008
  - AC-008
- 描述：工具链至少支持 Parse（源文件到 AST）、Check（名称、类型与语义检查）、Emit IR（输出 Canonical IR）、Diagnose（机器可读与人可读诊断）四项逻辑能力；0.1 CLI 命令表面为 `apls parse` / `apls check` / `apls emit-ir` / `apls diagnose` 与 `--version`。
- 前置：本地进程环境，可读取 Entry Source。
- 正常流程：按命令执行对应阶段，成功时 Exit 0；`emit-ir` 仅在完整可验证时交付 Verified IR。
- 异常流程：Source 诊断拒绝、Tool Failure 与 Compiler Internal Failure 分别以约定退出码失败关闭。
- 边界：stdout 仅承载有效 Artifact；命令集合、退出码 0/1/2/3 与 `--version` 固定行为冻结契约。
- 验收：四项能力经 CLI 可用，且命令表面、退出码与诊断边界符合已批准契约。
- 状态：CONFIRMED

## SYS-009 — 受控自然语言公开界面

- Traces-From（正式上游追溯）：
  - PRD-009
  - AC-009
- 描述：APLS 唯一公开规范性 Source 为版本化受控自然语言；0.1 首个规范性语言 Profile 为简体中文单语 Profile；多个已批准等价句式映射为同一形式语义；存在零个或多个完整语义的句子必须拒绝，不得由 AI 猜测；术语、同义表达、指代、数量、单位、模态、否定与时间范围由语言 Profile 机械定义；用户无需查看 DSL、AST 或 Canonical IR 即可完成正常编写、检查和修正流程；AI 可在编译事务外解释诊断和建议改写，但不能替编译器确定规范含义。
- 前置：Source 使用 `apls-zh-CN-0.1` Profile。
- 正常流程：合法句子经候选分析收敛为恰好一个 Canonical Frame 等价类。
- 异常流程：零个或多个完整语义的句子失败关闭。
- 边界：未被 Grammar 接受的自由自然语言只承担说明语义，不能隐式改变规范性语义。
- 验收：同 PRD-009 验收段（合法句子唯一收敛、同义句组同一 Canonical Meaning、模糊量/未定义术语/多指代/最终作用域多解样例全部失败关闭）。
- 状态：CONFIRMED
