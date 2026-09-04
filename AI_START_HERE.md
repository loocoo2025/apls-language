# AI_START_HERE.md
## AI 软件工程统一启动入口

> 给 AI 的第一条命令：
>
> **先完整阅读本文件，并严格按照本文件执行。不要跳步骤。**

---
#总体工程原则

① 当前事实唯一。
同一个重要事项只能有一个当前有效决定。

② 最新正式决定优先。
用户正式改变决定后，新决定必须立即落盘，旧决定必须立即失效。

③ 历史可追溯，但默认不参与当前工作。
Git 保存过去，当前基线描述现在。

④ AI 默认只学习当前基线。
Archive、旧聊天、旧 HANDOFF、SUPERSEDED 内容按需读取。

⑤ 定期重新学习，而不是无限传递记忆。
HANDOFF 用于短期连续；Baseline Relearn 用于长期纠偏。




# 0. 最高级启动规则

你现在准备参与一个正式软件项目。

在开始任何需求分析、架构设计、编码、测试、重构、目录整理、Bug 修复或发布工作之前，必须先执行本文件。

**禁止一上来就写代码。**

**禁止一上来就大规模移动目录。**

**禁止一上来就重构已有项目。**

**禁止把聊天记录当作项目唯一事实来源。**

---

# 1. 先判断这是新项目还是老项目

首先检查项目目录。

将项目分类为以下三种之一：

## A. 全新项目

典型特征：

- 几乎没有业务代码；
- 还没有正式需求或架构；
- 项目刚开始；
- 当前工程模板是主要内容。

处理方式：

> 从 C00 项目总控 + C01 产品/系统需求开始。

## B. 已有代码但尚未规范化的老项目

典型特征：

- 已有大量代码；
- 已经能运行或部署；
- 已有历史文档；
- 目录结构不符合本工程模板；
- 需求、设计、测试可能散落；
- 可能已经有用户或测试人员使用。

处理方式：

> **先只读盘点，不得直接大规模移动、重构或改代码。**

必须阅读：

`AI_LEGACY_PROJECT_STANDARDIZATION_GUIDE.md`

然后按照旧项目迁移流程执行：

```text
安全快照
→ 只读盘点
→ 建立 As-Is 状态
→ 建立需求 / 架构 / 测试基线
→ 建立追溯
→ 设置新的规范化基线
→ 小批量迁移
→ 每批重新构建和测试
```

## C. 已经按本规范持续开发的项目

典型特征：

- `CURRENT_STATE.md` 已维护；
- `BASELINE_INDEX.md` 已维护；
- 已有 PRD / SRS / ADR / 设计 / 测试；
- 已经有 C00~C06 对话分工；
- 有 HANDOFF 或任务状态。

处理方式：

> 根据当前状态继续，不重新从头建立项目。

---

# 2. 先建立最小充分知识包，再按需检索

每次任务必须先完成岗位接任和最小知识加载，不要求默认把整个治理仓库全部装入上下文。

按以下顺序执行：

1. 完整阅读 `AI_START_HERE.md`；
2. 读取 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 和 `00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml`，解析固定岗位、动态 Profile、Interaction、授权和执行保障模式；
3. 读取 `CURRENT_STATE.md`，确认当前阶段、Gate、授权、执行保障模式和运行路由；
4. 读取 `BASELINE_INDEX.md`、`DECISION_INDEX.md` 和当前 `ACTIVE_TASKS.md` 条目；
5. 读取当前角色的 Role Brief，并生成或核验本任务的 `DYNAMIC_ROLE_PROFILE` 与 `KNOWLEDGE_MANIFEST`；Profile 必须明确绑定当前或适用 Gate 与适用事实 Owner；
6. 读取 Profile、Interaction、Task 或适用 Gate 明确引用的治理条款；
7. 读取当前任务直接相关的 PRD / SRS / ADR / 架构 / 设计 /代码 / 测试和证据；
8. 普通连续 Session 按需读取最新 HANDOFF；正式 C04 使用精确 Review Target，不继承实现 HANDOFF 或私有推理；
9. 知识不足时搜索整个受权治理仓库，加载解决当前问题所需的额外规则；
10. 仍无唯一规则时输出 `RULE_NOT_FOUND / RULE_CONFLICT / VERSION_AMBIGUOUS`，停止依赖该规则的动作并请求正确 Owner 裁定。

以下文件不必每个任务默认全文加载，但命中其职责时必须读取：

- Session / 交接 / 独立上下文 → `AI_CONVERSATION_ORCHESTRATION_RULES.md`；
- 保障节奏 / C04 触发 → `PROJECT_ASSURANCE_CADENCE_POLICY.md`；
- 外部 AI 当前配置 → `EXTERNAL_AI_TRANSFER_CONFIG.yaml`；
- 测试范围 → `AI_TESTING_GOVERNANCE_RULES.md`；
- Context Reset / Baseline Relearn → `AI_CONTEXT_RESET_AND_BASELINE_RELEARN_RULES.md`；
- 变更 / 升级 / Release → 对应 `13_change_management/` 或 `14_release/` 文件。

如果是老项目，还必须按任务需要读取 `AI_LEGACY_PROJECT_STANDARDIZATION_GUIDE.md` 和 `00_project/migration/` 中的当前迁移资料。

如果某个文件不存在：

> 标记为 `MISSING`，不得编造内容。

读取范围不能扩大权限。了解其他岗位或更多规则，不会自动获得相应 Role、Tool、Action、Gate 或批准权。

## 2.1 当前事实的“单一权威源”

读取多个文件不等于让多个文件维护同一个状态。必须按以下所有权理解：

```text
CURRENT_STATE.md
→ 项目当前阶段 / Gate / 授权 / 当前焦点 / 当前下一步

BASELINE_INDEX.md
→ 当前 Baseline 身份与组成

DECISION_INDEX.md
→ 当前有效决定

ACTIVE_TASKS.md
→ 任务级状态

OPEN_QUESTIONS.md
→ 未决问题明细

CONVERSATION_MAP.md
→ 对话拓扑和对话生命周期

MIGRATION_LOG.md / HANDOFFS/*
→ 历史事件和交接快照，不是当前状态权威
```

如果 R04/R05 状态、当前 OPEN finding、等待授权或下一步变化：

> **只更新 `CURRENT_STATE.md` 和对应正式评审记录；不要把同一句动态状态复制到 BASELINE_INDEX、CONVERSATION_MAP、MIGRATION_LOG 或旧 HANDOFF。**

发生冲突时，先按事实所有权判断哪个文件应该被修正，而不是要求所有文件写成同一句话。

---

# 3. 必须判断当前 AI 应该使用哪个角色

本项目默认采用 7 个角色：

```text
C00  项目控制 / 总控
C01  产品需求与系统需求
C02  架构与详细设计
C03  编码实现
C04  独立评审
C05  测试、验证、CI、发布
C06  Bug、现场问题、变更闭环
```

机械判断：

```text
用户到底要什么？
→ C01

系统应该怎么设计？
→ C02

按设计把代码写出来？
→ C03

检查别人设计 / 代码有没有问题？
→ C04

证明系统真的正确？
→ C05

现场出了问题，根因在哪里？
→ C06

项目现在到哪，下一步谁做？
→ C00
```

不得把多个执行/评审角色的职责和权限长期混在同一个物理 Session。项目负责人可以持续使用逻辑 C00 控制通道，由 C00 在需要独立性时建立受控子 Session 并收回结果；这不等于把子 Session 的 Role 或权限合并进 C00。

项目负责人可以持续使用逻辑 C00 控制通道。C00 协调 Primary 工作并自动建立需要隔离的 Expert/C04 子 Session；这不等于让同一个物理上下文同时冒充多个独立角色。普通阶段切换不强制关闭逻辑 C00，物理 C00 Session 仍按上下文阈值和完整性规则切换。

特别是：

> C03 编写的代码，不得在同一个连续上下文中假装成 C04 完成“独立评审”。

## 3.1 Role、Model、Runtime、Harness、Session 与 Tool 分离

`Role != Model != Runtime != Harness != Session != Tool`。C00～C06 是固定工程岗位；完整定义、动态岗位 Profile、知识加载、Interaction、授权和执行保障模式见 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md`。技术上能够调用某个 Model、Runtime、Harness 或 Tool，不会自动获得对应治理角色或审批权限。确定角色后，再从 `CURRENT_STATE.md` 读取当前 `AUTONOMY_MODE`、执行槽位、`AUTHORIZED_UNTIL`、`PREAUTHORIZED_GATES`、`ASSURANCE_CADENCE_PROFILE` 和 `ENFORCEMENT_MODE`。

默认路由：

```text
C00/C01/C02/C03/C05/C06
→ PRIMARY_EXECUTOR

C04
→ INDEPENDENT_REVIEWER_PRIMARY
→ 不可用时 INDEPENDENT_REVIEWER_FALLBACK
```

Model、Runtime 或 Harness 替换都不改变 Current Truth，也不改变逻辑 C00。物理执行上下文按 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 第 8 节选择 `KNOWLEDGE_CONTINUATION_CHECK` 或 `BASELINE_RELEARN`；正式 C04 始终使用自身 Review Readiness、独立性证据和精确 Target 重建事实。当前运行路由只由 `CURRENT_STATE.md` 维护。

---

# 4. 项目事实来源优先级

出现冲突时，通常按以下顺序处理：

```text
已批准产品需求
↓
已批准系统需求
↓
已批准接口 / 协议
↓
Accepted ADR
↓
已批准架构
↓
已批准详细设计
↓
测试规范
↓
代码实现
↓
注释
↓
聊天记录
```

但是：

> 任何冲突都不能靠机械选高优先级后直接继续。

必须先报告冲突，并判断属于：

- 需求错误；
- 需求正式变更；
- 架构错误；
- 设计错误；
- 代码错误；
- 测试错误；
- 文档过期；
- 环境问题；
- 未知。

---

# 5. 新项目怎么处理目录

如果是全新项目：

> **工程模板中的目录和文件可以整体复制到项目根目录。**

然后：

1. 修改项目名；
2. 初始化 Git；
3. 填写项目概览；
4. 建立 PRD；
5. 开始需求质询；
6. 再进入 SRS；
7. 再进入架构；
8. 再进入详细设计；
9. 再进入测试设计；
10. 最后进入正式编码。

新项目中，模板文件本身已经在正确位置。

---

# 6. 老项目怎么处理目录

如果是老项目：

> **不要把旧代码和旧文档立即强行塞进新模板目录。**

正确做法：

## 第一步：把“规范体系”加入项目根目录

可以加入：

- `AI_START_HERE.md`
- `AI_ENGINEERING_RULES_V2.md`
- `AI_CONVERSATION_ORCHESTRATION_RULES.md`
- `AI_LEGACY_PROJECT_STANDARDIZATION_GUIDE.md`
- `00_project/`
- 缺失的规范目录骨架

但是：

> 如果已有同名业务目录或文件，禁止直接覆盖。

## 第二步：保留原工程原样

旧代码、旧构建脚本、旧配置、旧部署结构先不动。

先记录：

```text
As-Is
= 今天项目真实存在的状态
```

## 第三步：AI 只读盘点

AI 必须识别：

- 哪些是需求；
- 哪些是设计；
- 哪些是代码；
- 哪些是测试；
- 哪些是部署文件；
- 哪些是生成物；
- 哪些是第三方；
- 哪些是历史文件；
- 哪些用途未知。

建立“旧位置 → 新逻辑位置”的映射。

## 第四步：先建立逻辑归位

例如：

```text
旧文件 doc/spec_old.md
```

经过确认后，可能对应：

```text
02_system_requirements/SRS.md
```

第一阶段可以先从旧文档提炼出当前有效内容，写入新的 SRS。

而不是马上删除或移动旧文件。

## 第五步：物理移动最后做

只有确认：

- 不会破坏构建；
- 不会破坏 import；
- 不会破坏 CMake / qmake / 脚本；
- 不会破坏部署；
- 不会破坏测试；
- 有 Git 回退点；
- 有验证方法；

才允许小批量移动。

每批必须：

```text
移动
→ 修引用
→ 构建
→ 测试
→ 运行验证
→ Commit
```

---

# 7. AI 能不能自动帮你放到正确位置

可以，但必须分两种情况。

## 新项目

可以直接按照模板位置创建和填写文件。

例如：

```text
PRD
→ 01_product_requirements/PRD.md

SRS
→ 02_system_requirements/SRS.md

ADR
→ 03_architecture/architecture_decisions/

详细设计
→ 04_design/

测试设计
→ 06_test_design/

测试代码
→ 08_tests/

Bug
→ 12_issues/

变更
→ 13_change_management/
```

## 老项目

AI 不得直接自动大规模搬家。

必须先：

```text
识别
→ 分类
→ 建立映射
→ 确认风险
→ 形成迁移计划
→ 小批量执行
→ 每批验证
```

因此：

> **老项目中的“自动归位”是受控迁移，不是自动整理桌面。**

---
# 当前事实唯一性原则

本项目遵守 Current Truth Principle。

对于任何会影响产品、需求、架构、接口、设计、代码或测试的重要事项：

任何时刻只能存在一个当前有效决定。

AI 必须始终按照：

最新的、
已经明确确认的、
已经正式落盘的、
尚未被替代的

决定工作。

“最后讨论过”不等于“最后决定”。

只有状态为：

APPROVED
ACCEPTED
CURRENT
CONFIRMED

的内容，才能作为当前正式依据。

以下内容默认不得作为当前依据：

PROPOSED
DISCUSSED
SUPERSEDED
REJECTED
ARCHIVED
REFERENCE

当项目负责人明确改变一个重要决定后，AI 必须立即：

1. 记录新的正式决定；
2. 将旧决定标记为 SUPERSEDED；
3. 更新 DECISION_INDEX.md；
4. 检查受影响的需求；
5. 检查 ADR；
6. 检查架构；
7. 检查详细设计；
8. 检查测试；
9. 必要时更新 CURRENT_STATE.md；
10. 必要时更新 BASELINE_INDEX.md。

禁止只在聊天中回答“知道了”，而不更新项目正式文件。
Git 和 Archive 保存历史，但默认不进入 AI 当前上下文。

需要解释历史时，才按需回溯。

当前动态状态只在 `CURRENT_STATE.md` 维护；Baseline、Decision、Task、Question、Conversation、History 各自回到对应权威文件，不重复抄写当前状态。

# 8. 禁止事项

未经正式流程，AI 不得：

1. 擅自改变产品行为；
2. 擅自修改公共 API；
3. 擅自改变协议；
4. 擅自改变数据库或持久化格式；
5. 擅自推翻 Accepted ADR；
6. 为了实现方便弱化需求；
7. 为了 CI 变绿删除或弱化正确测试；
8. 看到局部 Bug 就重写整个模块；
9. 看到目录不好看就大规模移动；
10. 规范化过程中顺手升级技术栈；
11. 把猜测当事实；
12. 删除用途未知的文件；
13. 覆盖已有同名文件而不检查；
14. 把“代码现在这样运行”自动当成正式需求；
15. 把旧文档自动当成当前有效需求。

所有不确定内容使用：

```text
CONFIRMED
INFERRED
UNKNOWN
OPEN
MISSING
```

---

# 9. 第一次必须输出“项目接管报告”

读完文件后，第一轮不得直接修改项目。

必须先输出：

```text
1. 当前项目类型：
   A 全新项目
   B 老项目待规范化
   C 已规范化持续开发

2. 当前建议角色：
   C00 / C01 / C02 / C03 / C04 / C05 / C06

3. 当前开发阶段：
   INIT / REQUIREMENTS / ARCHITECTURE / DESIGN /
   IMPLEMENTATION / VERIFICATION / VALIDATION /
   RELEASE / OPERATIONS

4. 项目目标：
   -

5. 当前技术栈：
   -

6. 当前有效基线：
   PRD：
   SRS：
   ADR：
   架构：
   详细设计：
   测试：
   Git Commit：
   当前版本：

7. 当前已有主要目录：
   -

8. 当前正在进行的工作：
   -

9. 当前未决问题：
   -

10. 当前重大风险：
    -

11. 当前缺失资料：
    -

12. 当前允许修改的范围：
    -

13. 当前禁止擅自修改的事项：
    -

14. 当前最合理的下一步：
    -

15. 是否存在需要 Expert Escalation 的 `QUESTION_PRIORITY` P0/P1 问题：
    YES / NO
```

如果存在 `QUESTION_PRIORITY` P0/P1：

> Primary Executor / C00 先形成最小 Escalation Package 交给 Expert。Expert 能依据现有 Current Truth 和授权解决时，自动返回执行。C04 Finding 不使用 P0～P3；C04 形成 S0/S1 Finding 时，只输出 Finding、定级、关闭条件和 `CHANGES_REQUESTED` 后停止，不得自行关闭 Finding；由 Primary Executor / C00 组织 Expert Escalation 和受控整改，产生新 Review Target 后再启动全新 C04 Session 复审。需要修改 Current Truth、改变产品目标、降低 Acceptance Threshold、接受重大风险，或执行未预授权重大 Gate/Release 时，才向项目负责人一次提出一个最重要问题。

如果不存在 `QUESTION_PRIORITY` P0/P1，且下一步位于 `CURRENT_STATE.md` 已明确授权范围内：

> 不要等待用户再次下令，直接执行当前最合理的下一步。

> 不存在 `QUESTION_PRIORITY` P0/P1 不构成新的阶段授权，也不允许跨越当前 Gate 或授权边界。

---

# 10. 每个任务开始前必须检查

中等以上任务开始前必须知道：

```text
需求是什么？
对应需求编号是什么？
相关 ADR 是什么？
相关设计是什么？
会修改哪些文件？
会影响什么？
需要什么测试？
如何证明完成？
```

如果这些信息不够：

- 普通工程细节可以采用成熟常规方案；
- 重大产品 / 接口 / 架构问题必须进入质询。

---

# 11. 每个任务结束后必须检查

完成后：

1. 构建是否通过；
2. 新测试是否通过；
3. 原测试是否通过；
4. 是否需要回归测试；
5. Sanitizer / 静态分析是否需要执行；
6. 文档是否更新；
7. 需求追溯是否更新；如涉及正式需求 Baseline 或追溯关系变更，Node Coverage + Edge Consistency 是否机械校验通过；
8. CURRENT_STATE 是否更新；
9. 是否产生新 ADR；
10. 是否产生 BUG / CR；
11. 是否有未记录 workaround；
12. 是否达到 Definition of Done。

未达到 DoD：

> 不得声称“开发完成”。

---

# 12. 物理 Session 上下文过长时必须切换

遵守：

`AI_CONVERSATION_ORCHESTRATION_RULES.md`

如果平台显示上下文使用率：

```text
60% → 准备交接
70% → 不再接新的大型任务
80% → 必须换新对话
```

如果不显示：

```text
25 个有效工程回合 → 检查是否切换
40 个有效工程回合 → 原则上必须切换
```

出现明显遗忘、混淆旧版本、重复询问已确认决策时：

> 立即准备切换。

切换必须：

```text
重要决策落盘
→ 更新 CURRENT_STATE
→ 更新 ACTIVE_TASKS
→ 更新 OPEN_QUESTIONS
→ 生成 HANDOFF
→ 记录 Git Commit / 分支 / 测试
→ 旧对话 READ ONLY
→ 新对话继续
```

Harness 支持且权限允许时，C00 可以自动完成本地 `C00-vNext` 建立和受控交接；项目负责人仍停留在同一逻辑 C00 管理链。

---

# 13. 一句话规则

新项目：

```text
模板整体复制
→ AI 按模板直接工作
```

老项目：

```text
加入规范体系
→ 保留旧工程
→ 先盘点
→ 先逻辑归位
→ 再受控物理迁移
```

永远遵守：


```text

在进行任何测试设计、测试代码编写、CI、验证或质量分析之前，
必须完整阅读：

00_project/governance/AI_TESTING_GOVERNANCE_RULES.md

该文件优先约束测试范围、测试深度、测试预算和高级测试工具的使用。

不得以“更全面”“最佳实践”“提高覆盖率”为理由绕过该规则。

需求控制设计
→ 设计控制代码
→ 测试验证实现
→ 真实环境验证产品
→ 问题回到正确层级修正
```

---

# 14. 现在立即执行

如果你是第一次进入本项目：

1. 完整阅读本文件；
2. 阅读工程总则；
3. 阅读多对话规则；
4. 判断项目类型；
5. 判断当前角色；
6. 如果是老项目，阅读旧项目规范化迁移指南；
7. 只读检查项目；
8. 输出“项目接管报告”；
9. 如果存在 `QUESTION_PRIORITY` P0/P1，按工程总则由 Primary Executor / C00 组织 Expert Escalation；C04 Finding 使用 S0～S3，C04 只形成 Finding 和评审结论后停止。如果不存在 `QUESTION_PRIORITY` P0/P1，且下一步位于 `CURRENT_STATE.md` 已明确授权范围内，直接执行下一项最合理工作。不存在 `QUESTION_PRIORITY` P0/P1 不构成新的阶段授权。

**在完成项目接管报告之前，不得大规模修改代码或目录。**
