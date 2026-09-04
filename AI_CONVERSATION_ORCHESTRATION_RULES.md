# AI 多对话 / 多智能体协作规范
## 对话分工、上下文切换、交接、独立评审与项目状态管理

> **适用范围**
>
> 本文件适用于使用一个或多个 AI 智能体长期参与软件项目的情况。
>
> 本文件解决的问题不是“代码怎么写”，而是：
>
> - 一个项目应该建立几个 AI 对话；
> - 每个对话只负责什么；
> - 哪些事情禁止混在同一个对话里；
> - 多个对话之间如何交换信息；
> - 上下文太长时什么时候必须换新对话；
> - 换新对话时旧上下文如何交接；
> - 如何避免新 AI 重新发明需求、架构和历史决策；
> - 如何保证独立评审真的独立；
> - 如何让项目状态存在文件里，而不是只存在聊天记录里。
>
> **最高原则**
>
> > 聊天记录不是项目的最终事实来源。  
> > 项目文件、已批准需求、ADR、设计、代码、测试结果和正式问题记录才是事实来源。
>
> 任何重要信息如果只存在于某个 AI 对话中，而没有进入项目正式文件，则视为“尚未正式沉淀”。

---

# 1. 默认逻辑角色与物理 Session

每个正式软件项目保留 C00～C06 七个逻辑角色，但不要求启动时预先建立七个长期物理 Session。

默认由持续逻辑 C00 控制通道面对项目负责人。只要当前 Role、授权、上下文健康和独立性要求兼容，就继续当前 Session；当工作边界、保障节奏或独立性规则明确要求时，再创建对应 Worker / Expert / Reviewer Session。

标准逻辑角色如下：

```text
C00  项目控制 / 总控对话
C01  产品需求与需求质询对话
C02  架构与详细设计对话
C03  编码实现对话
C04  独立评审对话
C05  测试、验证、CI 与发布对话
C06  现场问题、Bug 与变更闭环对话
```

物理 Session 被实际创建时，推荐命名：

```text
<项目名>-C00-Control-v01
<项目名>-C01-Requirements-v01
<项目名>-C02-Architecture-Design-v01
<项目名>-C03-Implementation-v01
<项目名>-C04-Independent-Review-v01
<项目名>-C05-Verification-Release-v01
<项目名>-C06-Issues-Change-v01
```

`v01 / v02 / v03` 表示这个角色的第几代对话。

当上下文过长时，不改变角色编号，只增加版本号。

例如：

```text
ProjectA-C03-Implementation-v01
↓
ProjectA-C03-Implementation-v02
↓
ProjectA-C03-Implementation-v03
```

C04 每次正式评审和复审都必须新建独立 Session。普通阶段切换、轻量澄清或当前 Session 内 Auxiliary/Advisory 工具调用不自动要求新建用户可见 Session。

---

# 2. C00：项目控制 / 总控对话

## 2.1 主要职责

C00 是项目的“调度中心”和“状态管理员”。

负责：

- 判断项目当前处于哪个阶段；
- 维护项目整体进度；
- 维护当前基线；
- 维护当前有效文档索引；
- 维护对话地图；
- 把任务分派到正确角色对话；
- 汇总各角色产出；
- 检查阶段门禁是否满足；
- 检查 Definition of Done；
- 决定下一步应该进入哪个工作阶段；
- 发现多个角色输出冲突时，组织冲突解决；
- 保证重要结果已经落入正式文件；
- 保证没有重要决策只存在聊天里。

## 2.2 C00 不应负责

C00 原则上不直接承担：

- 大规模编码；
- 详细模块实现；
- 自己评审自己；
- 大量测试代码编写；
- 深度架构实现细节推演。

原因：

> 总控对话必须保持上下文干净，避免被大量代码和调试日志淹没。

## 2.3 C00 可以做的轻量工作

允许：

- 修改项目状态文档；
- 更新索引；
- 更新里程碑；
- 更新任务分派；
- 汇总结果；
- 维护少量流程类 Markdown 文件。

---

# 3. C01：产品需求与需求质询对话

## 3.1 主要职责

负责：

- 理解用户真正要做什么；
- 梳理产品目标；
- 用户场景；
- 产品边界；
- 功能需求；
- 非功能需求；
- 系统需求；
- 接口需求；
- 约束条件；
- 验收标准；
- 需求编号；
- 需求之间的冲突检查；
- 产品/系统需求连续质询；
- 形成 PRD、SRS 等文档；
- 建立需求到验收测试的初始映射；
- 维护 SYS / NFR / IF 的正式上游追溯元数据与 `requirements_traceability.md`；
- 在需求 Baseline 提交 C04 前运行 Node Coverage + Edge Consistency 机械校验。

## 3.2 C01 禁止做什么

原则上禁止：

- 直接开始大规模编码；
- 为了代码实现方便而改变产品需求；
- 在没有产品负责人确认的情况下自行决定重大产品行为；
- 因为某个功能难写就弱化需求。

## 3.3 C01 的结束条件

本轮需求工作结束时，至少应达到：

- `QUESTION_PRIORITY` P0/P1 需求问题已经闭环；
- 关键 `QUESTION_PRIORITY` P2 问题已经闭环或明确延期；
- 需求可以被验证；
- 重要需求有编号；
- 重要验收标准已经定义；
- 未决问题已进入 `OPEN_QUESTIONS.md`；
- 如本轮准备建立/更新正式需求 Baseline，Node Coverage 与 Edge Consistency 已机械校验通过，或例外已正式批准并逐条解释。

---

# 4. C02：架构与详细设计对话

## 4.1 主要职责

负责：

- 系统架构；
- 软件架构；
- 硬件/软件边界；
- 模块划分；
- 接口；
- 协议；
- 数据模型；
- 状态机；
- 线程模型；
- 生命周期；
- 错误处理；
- 日志；
- 配置；
- 部署架构；
- 安全设计；
- 性能设计；
- 可靠性设计；
- 可测试性设计；
- ADR；
- 详细代码设计文档。

## 4.2 C02 必须基于什么工作

必须先阅读：

- 已批准产品需求；
- 已批准系统需求；
- 当前有效 ADR；
- 已批准接口约束；
- 当前项目状态。

## 4.3 C02 禁止做什么

禁止：

- 重新解释已经明确的产品需求以迁就架构；
- 擅自修改已批准需求；
- 未记录 ADR 就改变关键技术路线；
- 直接把“代码已经这样写”作为设计依据；
- 在重大架构问题没有闭环时直接进入正式编码。

---

# 5. C03：编码实现对话

## 5.1 主要职责

负责：

- 根据批准的需求和设计实现代码；
- 编写或补充单元测试；
- 进行局部重构；
- 修复明确的代码级问题；
- 运行必要的构建和测试；
- 记录修改文件；
- 记录提交或版本；
- 记录未完成事项；
- 对实现过程中发现的设计冲突提出上报。

## 5.2 C03 的权限边界

C03 可以自行决定：

- 私有变量名；
- 私有函数拆分；
- 不改变外部行为的内部重构；
- 常规实现细节；
- 测试代码组织；
- 明显的代码风格问题。

C03 不得自行决定：

- 改变产品行为；
- 改变公共 API；
- 改变通信协议；
- 改变持久化格式；
- 改变数据库关键结构；
- 删除已有能力；
- 降低可靠性要求；
- 修改关键超时/恢复策略；
- 推翻 Accepted ADR。

遇到这些情况：

> 停止相关实现，记录冲突，交回 C02 或 C01 决策。

## 5.3 C03 不负责最终证明自己正确

C03 可以自测。

但：

> C03 的“我已经检查过了”不能替代 C04 独立评审和 C05 正式验证。

---

# 6. C04：独立评审对话

## 6.1 这是必须保持独立的对话

C04 必须使用一个没有参与当前实现推导过程的干净上下文。

它的工作不是帮助原作者证明方案正确。

它的工作是：

> 主动找错。

能够调用 Reviewer Model、Harness 或 CLI 不等于当前 Session 已成为 C04。`AUXILIARY / ADVISORY != FORMAL C04`；正式身份和成立条件以 `AI_ENGINEERING_RULES_V2.md` 第 38.7 节为准。

## 6.2 主要职责

负责：

- 需求评审；
- 需求 Baseline 评审时独立复核 Node Coverage 与 Edge Consistency；
- 架构评审；
- 详细设计评审；
- Code Review；
- 测试缺口审查；
- 变更风险审查；
- 发布前独立审查。

重点寻找：

- 需求遗漏；
- 需求冲突；
- 隐式假设；
- 不合理架构；
- 接口问题；
- 状态机漏洞；
- 并发问题；
- 生命周期问题；
- 内存问题；
- 错误处理问题；
- 恢复机制问题；
- 数据一致性问题；
- 兼容性问题；
- 安全问题；
- 可测试性问题；
- 测试覆盖缺口；
- 代码与设计不一致；
- 设计与需求不一致。

## 6.3 为了保持独立，C04 第一次评审时不要先读什么

第一次独立评审前，原则上不要先给 C04 阅读：

- C03 对自己代码的辩护；
- C02 对自己设计“为什么一定没问题”的长篇解释；
- 上一次评审 AI 的最终结论；
- 为了引导它赞同方案而写的提示。

C04 应优先阅读：

- 已批准需求；
- 当前 ADR；
- 当前设计；
- 实际代码；
- 实际测试；
- 必要的运行证据。

先形成第一轮独立判断。

之后再允许阅读原作者解释。

---

# 7. C05：测试、验证、CI 与发布对话

## 7.1 主要职责

负责：

- 测试策略；
- 测试计划；
- 单元/集成/系统/回归测试组织；
- 测试覆盖分析；
- ASan；
- UBSan；
- 必要时 TSan；
- 静态分析；
- 性能测试；
- 长时间运行测试；
- 故障注入；
- CI；
- 测试环境部署；
- 准真实/真实环境验证；
- 发布门禁；
- 发布检查表；
- 测试证据整理。

## 7.2 C05 的原则

C05 负责回答：

> “这个东西真的被验证了吗？”

而不是：

> “开发 AI 说它应该可以。”

## 7.3 C05 禁止做什么

禁止：

- 为了通过测试擅自修改产品需求；
- 为了 CI 变绿删除失败测试；
- 用旧测试报告证明新代码；
- 用不同构建产物代替已经测试过的产物；
- 未说明原因就降低验收标准。

---

# 8. C06：现场问题、Bug 与变更闭环对话

## 8.1 什么时候启用

从以下任一阶段开始正式使用：

- 测试人员开始使用；
- 真实环境部署；
- 现场试运行；
- Beta；
- 正式运行；
- 用户反馈进入项目。

## 8.2 主要职责

负责：

- 问题单；
- Bug 分类；
- 复现；
- 根因分析；
- 影响分析；
- 判断问题属于哪一层；
- 生成变更请求；
- 建议回归测试；
- 跟踪问题闭环；
- 检查修复是否真正关闭问题。

## 8.3 C06 不应直接做什么

不要看到 Bug 就立刻自己改代码。

正确流程：

```text
问题进入 C06
↓
复现与证据
↓
根因分类
↓
如果是需求问题 → C01
如果是架构/设计问题 → C02
如果是实现问题 → C03
如果是验证问题 → C05
↓
C04 必要时独立复核
↓
C06 确认问题闭环
```

---

# 9. 对话之间如何通信

多个 AI 对话之间，不应依赖：

> “你去看另一个聊天里我刚才说了什么。”

正确方式是：

> **通过项目正式文件通信。**

会影响任务、权限、Gate、受控产物或正式状态的跨岗位动作，还必须实例化 `INTERACTION_CONTRACT / INTERACTION_OPERATION`。字段、生命周期和授权边界由 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 与 `00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml` 定义。自然语言可以作为说明或证据附件，但不能替代 Sender/Receiver、Action、Scope、Authority、Allowed/Forbidden Actions、Receipt、Status、Terminal State 和 Audit Reference。

必须建立：

```text
00_project/
└── ai_context/
    ├── CURRENT_STATE.md
    ├── CONVERSATION_MAP.md
    ├── DECISION_INDEX.md
    ├── OPEN_QUESTIONS.md
    ├── ACTIVE_TASKS.md
    ├── BASELINE_INDEX.md
    ├── ROLE_BRIEFS/
    │   ├── C00_CONTROL.md
    │   ├── C01_REQUIREMENTS.md
    │   ├── C02_ARCHITECTURE_DESIGN.md
    │   ├── C03_IMPLEMENTATION.md
    │   ├── C04_INDEPENDENT_REVIEW.md
    │   ├── C05_VERIFICATION_RELEASE.md
    │   └── C06_ISSUES_CHANGE.md
    └── HANDOFFS/
        └── ...
```

---

# 10. CURRENT_STATE.md 必须写什么

`CURRENT_STATE.md` 是所有新对话快速理解项目现状的入口。

至少包含：

```text
# 当前项目状态

项目：
当前版本：
当前 Git Commit：
当前开发阶段：
当前里程碑：
最后更新时间：

## 当前已经批准的基线
- 产品需求：
- 系统需求：
- 架构：
- ADR：
- 详细设计：
- 测试计划：

## 当前正在开发
- ...

## 当前正在测试
- ...

## 当前阻塞项
- ...

## 当前已知重大风险
- ...

## 当前未决问题
- ...

## 当前下一步
- ...
```

禁止把几个月历史全部复制进去。

它只描述：

> **现在是什么状态。**

---

# 11. CONVERSATION_MAP.md 必须写什么

至少记录：

| 对话ID | 角色 | 当前版本 | 状态 | 当前任务 | 允许写入范围 |
|---|---|---|---|---|---|
| C00 | Control | v02 | ACTIVE | 里程碑管理 | 00_project |
| C01 | Requirements | v03 | ACTIVE | SYS-NET需求 | 01/02 requirements |
| C02 | Architecture | v02 | ACTIVE | 网络架构 | 03/04 design |
| C03 | Implementation | v05 | ACTIVE | ConnectionManager | src + unit tests |
| C04 | Review | v04 | ACTIVE | 网络模块独立审查 | READ ONLY + review |
| C05 | Verification | v03 | ACTIVE | sanitizer/CI | tests/quality/ci |
| C06 | Issues | v01 | ACTIVE | BUG-017 | issues/change |

同时记录：

- 谁是某个文件或模块当前允许的主要修改者；
- 是否存在并行实现；
- 是否存在文件写入冲突。

---

# 12. 一个重要规则：同一正式文件不要让多个对话同时写

除非使用明确的 Git 分支和合并流程，否则：

> **同一个权威文档或同一个代码文件，在同一时间只允许一个主要写入者。**

例如：

- C02 正在修改 `system_architecture.md`；
- C04 只能提交评审意见；
- C04 不应该直接偷偷改架构文件。

评审通过后：

- C02 根据评审意见修改；
- 或由 C00 明确转移写入权。

这样避免多个 AI 相互覆盖。

---

# 13. 对话什么时候必须换新

每个角色对话都不能无限延长。

满足以下任一条件时，必须准备切换新对话版本。

## 13.1 第一优先级：平台有上下文使用率

如果平台能够显示上下文占用：

```text
达到 60%：开始准备交接文档
达到 70%：原则上停止接受新的大任务
达到 80%：必须切换新对话
```

不要等到上下文几乎满了才整理。

## 13.2 第二优先级：平台不显示上下文使用率

使用以下机械规则。

一个对话达到 **25 个“有效工作回合”** 后：

> 必须检查是否应该切换。

达到 **40 个有效工作回合** 后：

> 原则上必须切换新对话。

### 什么叫“有效工作回合”

一次满足下面任一条件的用户任务 + AI 工作结果，算 1 个有效工作回合：

- 修改了正式文档；
- 做了一个明确设计决策；
- 修改了代码；
- 运行并分析了测试；
- 完成了一次评审；
- 分析并关闭了一个 Bug；
- 完成了一个明确工程任务。

简单问一句定义、寒暄、不改变项目状态的讨论不计。

## 13.3 无论回合数量多少，出现以下情况必须立即切换

以下任一事件发生，就不要继续硬撑旧上下文：

1. AI 开始重复询问已经明确的信息；
2. AI 对已经批准的决策产生明显遗忘；
3. AI 前后引用不同版本需求，却没有意识到；
4. AI 把已经废弃的方案当成当前方案；
5. AI 无法准确说出当前基线；
6. AI 对文件版本、Commit、接口版本产生混淆；
7. AI 开始把旧 Bug 的临时方案当正式方案；
8. AI 连续两次出现明显上下文记忆错误；
9. 非 C00 的阶段专用工作 Session 已跨越两个以上重大里程碑，导致职责或事实范围明显混杂；
10. 阶段专用工作 Session 的职责已经结束，下一阶段需要不同上下文；用户面对的 C00 逻辑控制通道不因普通阶段切换自动关闭；
11. 完成一次大型重构；
12. 完成一个重要发布版本；
13. 即将开始一个与前一任务几乎无关的大型子系统。

---

# 14. 哪些时候即使上下文没长，也建议主动换新

以下情况推荐创建干净的新对话：

- 从需求阶段转架构阶段；
- 从架构阶段进入大型编码；
- 做重大 Code Review；
- 做发布前独立审查；
- 分析严重现场事故；
- 对已经争论很久的方案做第三方裁决；
- 原 AI 明显对自己此前观点产生“路径依赖”。

特别是：

> **独立评审必须优先使用新上下文。**

这些建议主要约束阶段专用 Worker / Expert / Reviewer Session。C00 作为项目负责人面对的逻辑控制通道默认持续存在；物理 C00 Session 只在上下文阈值、完整性失效或明确 Clean Context Reset / Baseline Relearn 触发时切换为 `C00-vNext`。

---

# 15. 换新对话前必须先做交接

禁止：

```text
旧对话太长
↓
直接开新聊天
↓
对新 AI 说“继续刚才的工作”
```

因为新 AI 根本不知道“刚才”是什么。

正确流程：

```text
旧对话
↓
更新正式项目文件
↓
更新 CURRENT_STATE.md
↓
更新 OPEN_QUESTIONS.md
↓
更新 ACTIVE_TASKS.md
↓
形成 HANDOFF 文件
↓
冻结旧对话
↓
创建新对话
↓
新对话先读规则和交接材料
↓
确认理解
↓
继续工作
```

---

# 16. 标准 HANDOFF 文件模板

每次上下文切换都应创建：

```text
00_project/ai_context/HANDOFFS/
C03-v05-to-v06-2026-08-21.md
```

内容至少如下：

```text
# AI 对话交接记录

## 1. 基本信息
项目：
角色：
旧对话：
新对话：
交接时间：
当前 Git Commit：
当前分支：

## 2. 当前开发阶段
当前处于：
当前里程碑：

## 3. 本轮任务目标
本对话原本要完成什么？

## 4. 已经完成
- ...
- ...
- ...

## 5. 当前批准的需求
- SYS-...
- NFR-...
- IF-...

## 6. 当前有效 ADR
- ADR-...
- ADR-...

## 7. 当前有效设计
- 文件：
- 版本：
- 关键设计点：

## 8. 本轮做出的新决策
只写已经正式记录的决策：
- DEC/ADR/需求编号：
- 决策内容：
- 正式记录位置：

## 9. 修改过的文件
- ...
- ...
- ...

## 10. 测试情况
执行过：
结果：
未执行：
失败项：

## 11. 当前未完成
- ...
- ...

## 12. 当前阻塞
- ...

## 13. 当前未决问题
- ...

## 14. 当前已知风险
- ...

## 15. 临时方案 / Workaround
如果没有，明确写“无”。

## 16. 禁止新 AI 误解的事项
例如：
- ADR-003 已经确定使用 TCP，不要重新改成 MQTT；
- TC-017 失败尚未解决，不要标记通过；
- 某临时分支尚未合并。

## 17. 下一步唯一建议动作
写清楚新对话开始后第一件应该做什么。

## 18. 权威文件列表
新 AI 必须优先阅读哪些文件？

## 19. 交接完成检查
- [ ] 项目状态已更新
- [ ] 重要决策已落盘
- [ ] 未决问题已记录
- [ ] 测试结果已记录
- [ ] 修改文件已记录
- [ ] 临时方案已记录
- [ ] 下一步已明确
```

---

# 17. 新对话启动时必须按固定顺序读取

新对话不得先阅读整个旧聊天记录。

权威读取入口：

```text
1. 首先完整阅读 AI_START_HERE.md
2. 严格遵循 AI_START_HERE.md 维护的当前权威启动顺序
3. 应用本角色 ROLE_BRIEF 和任务最小相关输入
4. 普通连续 Session 按规则读取最新 HANDOFF；C04 使用精确 Review Target，不继承实现 HANDOFF 或私有推理
```

原因：

> 先读当前有效事实，再读任务细节。  
> 不要先被旧聊天中的废弃思路污染。

---

# 18. 新对话启动后必须先做“上下文校验”

新 AI 读完材料后，不允许立刻修改代码。

先输出一个简短校验结果：

```text
当前角色：
当前阶段：
当前任务：
当前有效基线：
当前主要约束：
当前未决问题：
我被允许修改的范围：
我不能擅自修改的事项：
下一步动作：
```

如果这里明显理解错误：

> 先纠正理解，再工作。

---

# 19. 旧聊天记录的地位

旧聊天可以作为：

- 历史参考；
- 调试思路来源；
- 讨论过程记录。

但不得自动作为：

- 当前需求；
- 当前架构；
- 当前接口；
- 当前测试标准；
- 当前产品行为。

如果旧聊天和当前批准文档冲突：

> 以当前批准文档为准。

---

# 20. 不要把“总结”当成新的事实来源

AI 的上下文总结、handoff、CURRENT_STATE 都是导航材料。

它们不能凭空创造新决策。

如果总结里写：

> “系统决定采用方案 A。”

那么必须能追溯到：

- ADR；
- 批准的需求；
- 批准的设计；
- 正式决策记录。

否则这句话只是“待核实”。

---

# 21. 跨对话冲突如何处理

如果两个对话得出冲突结论：

例如：

```text
C02 认为必须使用方案 A
C05 发现方案 A 无法满足性能要求
```

禁止：

- C05 自己偷偷改成 B；
- C02 无视测试证据；
- C00 随便拍脑袋选一个。

正确流程：

```text
记录冲突
↓
明确受影响需求
↓
收集证据
↓
C02 重新评估架构
↓
必要时创建新 ADR
↓
C04 独立评审
↓
重大业务影响由项目负责人决策
↓
更新正式文件
```

---

# 22. 哪些信息必须“落盘”，不能只留在聊天里

以下信息必须进入项目文件：

- 已批准需求；
- 需求变更；
- 架构决策；
- 接口变化；
- 协议变化；
- 数据结构重大变化；
- 关键性能指标；
- 安全策略；
- 测试标准；
- Bug 根因；
- 临时 workaround；
- 测试结果；
- 发布结果；
- 未决重大风险；
- 重要设计取舍；
- 关键用户决定。

规则：

> **如果一个决定未来可能影响代码，就必须落盘。**

---

# 23. 每次重要对话结束前必须做“落盘检查”

AI 在结束一项重要任务前，必须检查：

```text
我刚才有没有形成新的正式决策？
↓
有没有只存在聊天里？
↓
需要更新哪个文件？
↓
需求追溯是否需要更新？
↓
CURRENT_STATE 是否需要更新？
↓
OPEN_QUESTIONS 是否需要更新？
↓
是否需要 ADR？
↓
是否需要 BUG / CR？
↓
是否需要 HANDOFF？
```

---

# 24. 并行开发什么时候允许增加更多实现对话

如果项目很大，可以在 C03 下增加子对话：

```text
C03A Network
C03B Storage
C03C UI
C03D Device
```

但只有同时满足以下条件才允许：

1. 模块边界已经由 C02 明确；
2. 公共接口已经明确；
3. 每个对话主要修改不同文件；
4. 使用独立 Git 分支或 worktree；
5. 已在 `CONVERSATION_MAP.md` 登记；
6. 有明确合并责任人；
7. 合并后必须重新跑集成测试；
8. C04 必须评审跨模块影响。

如果两个 AI 需要同时大改同一组核心文件：

> 默认不要并行。

---

# 25. Git 推荐规则

如果项目使用 Git：

每个重要实现任务建议使用：

```text
ai/<role>/<task-id>
```

例如：

```text
ai/c03/SYS-NET-001-reconnect
ai/c03/BUG-017-memory-leak
```

每次交接必须记录：

- 分支；
- Commit；
- 未提交修改；
- 是否已合并；
- 是否已验证。

禁止只说：

> “代码已经改好了。”

必须知道：

> 改的是哪个版本。

---

# 26. 一个任务应该在哪个对话里做

使用下面的机械判断：

```text
这是“用户到底要什么”的问题？
→ C01

这是“系统应该怎么设计”的问题？
→ C02

这是“按设计把代码写出来”的问题？
→ C03

这是“别人写的东西有没有问题”的问题？
→ C04

这是“怎么证明它真的正确”的问题？
→ C05

这是“现场出了问题，根因在哪里”的问题？
→ C06

这是“项目现在到哪、下一步谁做”的问题？
→ C00
```

---

# 27. 禁止角色混淆

以下行为原则上禁止：

### 禁止 1
C03 一边写代码，一边自己修改需求让代码合理。

### 禁止 2
C02 设计完架构后，在同一上下文里假装自己是“独立评审者”。

### 禁止 3
C05 发现测试失败后，直接修改验收标准。

### 禁止 4
C06 看到 Bug 后直接大规模重写代码，不做根因分析。

### 禁止 5
C00 被大量编译日志和代码 diff 占满上下文。

### 禁止 6
为了省事，把所有角色重新合并成一个长期无限对话。

---

# 28. 一个重要原则：作者和审查者尽量分开

对于重要内容：

```text
需求作者 ≠ 最终需求评审者
架构作者 ≠ 独立架构评审者
代码作者 ≠ 唯一 Code Reviewer
测试作者 ≠ 唯一验收人
```

AI 可以参与所有角色。

但：

> 重要评审最好使用不同对话、不同上下文。

---

# 29. 当一个 AI 忘记东西时，不要继续喂旧聊天

如果 AI 开始忘记：

错误做法：

```text
继续在旧聊天里不断补充：
“我之前不是跟你说过了吗……”
“你再往前看看……”
```

正确做法：

```text
停止
↓
把当前正确状态写入正式文件
↓
建立 HANDOFF
↓
新开角色同编号的新版本对话
↓
从当前基线重新开始
```

因为：

> 上下文已经混乱时，继续往里面塞信息通常会更乱。

---

# 30. 对话版本切换后，旧对话默认冻结

例如：

```text
C03-v05 → C03-v06
```

切换完成后：

`C03-v05` 默认进入：

```text
ARCHIVED / READ ONLY
```

不要一会儿回旧对话改代码，一会儿又去新对话。

否则会产生两个不同“当前状态”。

如果确实需要查历史：

> 可以阅读旧对话，但新工作必须继续在最新版本。

---

# 31. 一个项目什么时候必须重新建立 C04 独立评审对话

C04 即使还没达到上下文上限，在以下情况也建议开新版本：

- 新的重大架构版本；
- 大型重构；
- 一个重大版本发布前；
- 严重 Bug 修复后；
- 前一个 C04 已经深度参与整改，可能不再独立；
- 需要“真正第二意见”。

例如：

```text
C04-Independent-Review-v03
```

做完评审后长期参与修改。

那么下一次重大独立评审可以直接：

```text
C04-Independent-Review-v04
```

保持干净。

---

# 32. 项目阶段与主要责任对话

推荐如下：

| 阶段 | 主责任对话 | 必须参与 |
|---|---|---|
| 产品目标 | C01 | C00 |
| 产品需求 | C01 | C00、C04 |
| 系统需求 | C01 | C00、C04 |
| 架构 | C02 | C00、C04、C05 |
| 详细设计 | C02 | C03、C04、C05 |
| 测试设计 | C05 | C01、C02、C03 |
| 编码 | C03 | C04、C05 |
| 集成 | C03/C05 | C04 |
| 真实环境验证 | C05 | C06 |
| Bug 闭环 | C06 | C01/C02/C03/C05，按根因选择 |
| 发布 | C05 | C00、C04 |
| 现场运行 | C06 | C00 |

---

# 33. 每个角色对话的输入和输出

## C00

输入：

- 所有阶段状态；
- 评审结果；
- 测试结果；
- Bug 状态。

输出：

- 当前状态；
- 任务分配；
- 阶段门禁；
- 下一步动作。

## C01

输入：

- 用户目标；
- 业务约束；
- 现场反馈。

输出：

- PRD；
- SRS；
- 需求编号；
- 验收标准；
- 需求质询记录。

## C02

输入：

- 已批准需求；
- 技术约束；
- C04/C05 反馈。

输出：

- 架构；
- ADR；
- 详细设计；
- 接口；
- 状态机；
- 模块边界。

## C03

输入：

- 已批准需求；
- ADR；
- 详细设计；
- 任务范围。

输出：

- 代码；
- 单元测试；
- 构建结果；
- 实现记录；
- 实现阶段发现的问题。

## C04

输入：

- 需求；
- 架构；
- 设计；
- 代码；
- 测试证据。

输出：

- 独立问题清单；
- C04 Finding Severity；
- 必须整改项；
- 非阻断 `ADVISORY / OBSERVATION / FUTURE_IMPROVEMENT`；
- 已正式批准的 Exception / Risk Acceptance 引用。

## C05

输入：

- 需求验收标准；
- 设计；
- 代码；
- 构建产物。

输出：

- 测试设计；
- 自动化测试；
- CI；
- 验证结果；
- 发布门禁结果。

## C06

输入：

- 现场问题；
- 日志；
- 复现数据；
- 用户反馈。

输出：

- BUG；
- 根因分类；
- CR；
- 回归要求；
- 闭环结论。

---

# 34. 每次新对话的标准启动提示词

可以直接复制：

```text
你现在是本项目的【CXX 角色名称】。

你必须首先完整阅读 `AI_START_HERE.md`，再遵循它维护的权威启动顺序。本提示词不复制另一份顺序。

随后应用你的 ROLE_BRIEF 和当前任务最小相关输入。普通连续 Session 按规则读取最新 HANDOFF；C04 使用精确 Git Review Target，不继承实现 HANDOFF 或私有推理。

规则：

- 聊天历史不是最高级事实来源；
- 当前批准项目文件才是事实来源；
- 不得擅自修改超出本角色权限的事项；
- 不得重新发明已经有明确决策的内容；
- 如果发现文件之间冲突，先报告冲突，不要偷偷选一个；
- 如果需要改变重要需求、接口或架构，必须走正式变更流程；
- 所有重要新决策必须落入正式文件；
- 当前对话接近上下文切换条件时，必须主动生成 HANDOFF，不得硬撑到上下文混乱。

阅读完成后，先不要直接修改代码。

先输出：

- 当前角色；
- 当前项目阶段；
- 当前任务；
- 当前有效基线；
- 当前主要约束；
- 当前未决问题；
- 你允许修改的范围；
- 你禁止擅自修改的事项；
- 你建议的下一步动作。

确认理解一致后，再开始执行本角色工作。
```

---

# 35. C04 独立评审对话专用启动提示词

```text
你现在是 C04 独立评审智能体。

你的职责不是帮助原作者证明方案正确，而是主动寻找问题。

第一次评审时：

1. 先阅读已批准需求；
2. 阅读当前有效 ADR；
3. 阅读当前设计；
4. 阅读实际代码和测试；
5. 阅读必要运行证据；
6. 不要先接受原作者的自我解释；
7. 不要因为代码能运行就默认设计正确；
8. 不要因为测试全绿就默认测试充分；
9. 主动寻找需求遗漏、架构风险、状态机漏洞、并发问题、生命周期问题、资源问题、错误处理问题、恢复问题、接口问题、兼容性问题和测试缺口；
10. 如果评审对象包含正式需求 Baseline 或需求追溯，独立运行 `09_quality/traceability/validate_traceability.py`，不得用“ID 全覆盖”替代关系边一致性证明；
11. 每个重要问题说明：
   - 问题是什么；
   - 为什么重要；
   - 影响什么；
   - C04 Finding Severity（S0/S1/S2/S3）；
   - 需要什么证据才能关闭。

先按工程总则第 38.7 节记录 Review Readiness。`REVIEW_NOT_READY` 时只记录缺失的前置条件，不产生 Gate Decision。只有 `READY` 时才能输出 `PASS / CHANGES_REQUESTED`。你只提交评审记录，不修改被评审对象、不参与整改设计、不批准 Exception，也不关闭自己提出的 Finding。
```

---

# 36. 上下文切换专用指令

当某个角色对话需要换新时，可以直接发：

```text
当前对话准备进行上下文切换。

不要继续接受新的大型任务。

现在执行交接：

1. 检查本对话形成的所有重要决策是否已经写入正式项目文件；
2. 更新 CURRENT_STATE.md；
3. 更新 ACTIVE_TASKS.md；
4. 更新 OPEN_QUESTIONS.md；
5. 更新 DECISION_INDEX.md；
6. 更新 CONVERSATION_MAP.md；
7. 创建本角色从当前版本到下一版本的 HANDOFF 文件；
8. 记录当前 Git 分支、Commit、未提交改动；
9. 记录已执行测试和结果；
10. 明确所有未完成事项、风险、临时方案；
11. 明确新对话开始后的第一项任务；
12. 不要在 HANDOFF 中创造新的需求或设计决策。

完成后，本对话进入 READ ONLY。
```

---

# 37. 项目开始时一次性建立的文件

每个新项目建议先建立：

```text
AI_START_HERE.md
AI_ENGINEERING_RULES_V2.md
AI_CONVERSATION_ORCHESTRATION_RULES.md

00_project/
└── ai_context/
    ├── CURRENT_STATE.md
    ├── CONVERSATION_MAP.md
    ├── DECISION_INDEX.md
    ├── OPEN_QUESTIONS.md
    ├── ACTIVE_TASKS.md
    ├── BASELINE_INDEX.md
    ├── ROLE_BRIEFS/
    │   ├── C00_CONTROL.md
    │   ├── C01_REQUIREMENTS.md
    │   ├── C02_ARCHITECTURE_DESIGN.md
    │   ├── C03_IMPLEMENTATION.md
    │   ├── C04_INDEPENDENT_REVIEW.md
    │   ├── C05_VERIFICATION_RELEASE.md
    │   └── C06_ISSUES_CHANGE.md
    └── HANDOFFS/
```

---

# 38. 最低要求：再小的项目也不要低于 4 个逻辑角色

极小项目允许减少实际对话数量。

但至少保留 4 个逻辑角色：

```text
A. 需求/设计
B. 实现
C. 独立评审
D. 验证
```

其中：

> 独立评审不能和实现角色在同一个连续上下文里完成。

正式长期产品仍保留标准 C00～C06 七个逻辑角色；物理 Session 按实际触发创建。

---

# 39. 最重要的十条机械规则

如果 AI 只能记住十条，就记下面十条：

1. **保留 C00～C06 七个逻辑角色；默认持续 C00，按需建立物理 Session。**
2. **一个角色只做自己的事。**
3. **聊天记录不是项目真相，正式文件才是。**
4. **重要决策必须落盘并编号。**
5. **作者和独立评审者必须尽量分开。**
6. **同一个正式文件不要让多个对话同时修改。**
7. **上下文达到 60% 开始交接，70% 不接大任务，80% 必须换；没有百分比时 25 回合检查、40 回合强制换。**
8. **换对话前必须更新状态并生成 HANDOFF。**
9. **新对话先读规则、当前状态、基线和交接，不要先读整个旧聊天。**
10. **旧对话切换后冻结为 READ ONLY，不再两边同时工作。**

---

# 40. 一句话总纲

```text
把“长期记忆”放进项目文件，
把“临时推理”放进当前对话，
把“不同职责”放进不同对话，
把“独立检查”放进干净上下文，
把“上下文切换”做成正式交接，
这样 AI 才能在长期项目里持续工作而不逐渐失控。
```

---

# 41. 模型路由下的对话编排

固定 Role、动态 Profile、Role / Model / Runtime / Harness / Session / Tool 定义、Interaction、通用授权、四条运行线、Formal Seal 和执行保障模式，以 `00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md` 为唯一权威来源；执行槽位、Expert 路由、权限继承和正式 C04 语义见 `AI_ENGINEERING_RULES_V2.md` 第 38 章。本文件只规定 Session、上下文和交接如何编排；当前实际运行槽位值、Autonomy Mode、Enforcement Mode 和自动授权上限见 `CURRENT_STATE.md`。

## 41.1 三类执行 Session

```text
Primary Session
→ 承担 C00/C01/C02/C03/C05/C06 的默认连续执行

Expert Escalation Session
→ 接收最小 Escalation Package
→ 只加载回答精确问题所需的最小文件
→ 输出技术结论和 HUMAN DECISION REQUIRED

C04 Independent Review Session
→ 使用全新独立上下文
→ 从正式文件和精确 Git Review Target 重建事实
→ 不继承实现 AI 的私有推理
→ 先记录 Review Readiness，只有 READY 时才产生 Gate Decision
```

任何受控 Session 执行前必须存在 `DYNAMIC_ROLE_PROFILE` 和 `KNOWLEDGE_MANIFEST`，并与当前 Role、Task、当前或适用 Gate、适用事实 Owner、Interaction、Authority、Model/Runtime/Harness/Session 绑定一致。Profile 未就绪时不得通过提示词猜测权限。

执行角色在 Session 内发起的辅助 Model / CLI / API 调用不是新的治理 Session，沿用调用者 Role 和权限边界，其输出仅为 `ADVISORY / AUXILIARY`。只有按照正式独立评审流程建立的 `C04 Independent Review Session` 才能产生 C04 Gate 结论。

项目负责人默认停留在持续的逻辑 C00 控制通道。C00 可以协调兼容的 Primary 工作、发起当前 Session 内 Auxiliary/Advisory Tool 调用、建立 Expert/C04 子 Session，并把受控结果收回当前任务。子 Session 不要求负责人手工切换查看；只有命中负责人保留决策时才向负责人提出问题。

```text
DEFAULT_SESSION_ACTION: CONTINUE_CURRENT_SESSION
```

需要独立 Session 时必须先明确输出并形成第 41.5 节请求包；不得静默停止并假设负责人会自行创建会话。

Role、Model、Runtime、Harness、Session 和 Tool 是六个独立维度。`CONVERSATION_MAP.md` 继续只维护角色、对话版本和生命周期，不登记 Model/Runtime/Harness 路由或工具调用。

## 41.2 升级与返回

检查或裁决意图必须先按岗位交互与可执行治理政策唯一分类为：

```text
INDEPENDENT_REVIEW
CONTEXTUAL_REVIEW
SELF_REVIEW
HUMAN_DETERMINATION
```

`INDEPENDENT_REVIEW` 进一步区分 `FORMAL_C04` 与 `INFORMAL_INDEPENDENT`。只有 `FORMAL_C04` 能产生正式 C04 Gate Decision；非正式独立评审、Contextual Review 和 Self Review 都不能冒充 C04。普通实现、查询和机械动作不因完成后需要自检就自动进入正式评审。

Primary 命中工程总则的 Expert Escalation 触发条件时：

1. 固定问题范围和 What Must Not Change；
2. 生成最小 Escalation Package；
3. 优先启动 `EXPERT_ESCALATION_PRIMARY`；
4. Expert Primary 不可用时启动 `EXPERT_ESCALATION_FALLBACK`；
5. Expert 输出 `HUMAN DECISION REQUIRED = NO` 时，结论返回原角色和原 `ACTIVE_TASK` 自动继续；
6. 只有输出 `YES` 且命中人工权威边界时，才由 C00 向项目负责人提出一个最重要问题。

Primary 执行中 `QUESTION_PRIORITY / WORK_PRIORITY` 为 P0/P1 的问题首先触发 Expert Escalation，不直接等同于人工阻塞。P2/P3 在当前批准范围内默认由 Primary 自动处理。P0～P3 不是 C04 Finding Severity。

C04 形成 S0/S1 Finding 时只记录 Finding、给出关闭条件和 `CHANGES_REQUESTED`，然后停止。后续编排必须为：

```text
Primary Executor / C00
→ 根据 Finding 启动 Expert Escalation
→ 完成受控整改
→ 形成新的精确 Review Target
→ 启动全新独立 C04 Session 复审
```

C04 不得加入被审对象的整改设计 Session，也不得自行关闭自己提出的 Finding。Finding 只能由面向新精确 Review Target 的全新独立 C04 Session 复核关闭。

S2/S3 Finding 由 Primary Executor 在现有授权范围内整改，同样必须形成新的精确 Review Target 并启动全新独立 C04 Session 复审。Severity 只决定风险表达、优先顺序和默认路由；任一 Open Finding 都阻断 `PASS`。

## 41.3 C04 Provider 与独立性

`AUXILIARY / ADVISORY != FORMAL C04`。Primary、Expert 或其他执行 Session 调用一次 Codex、DeepSeek、Kimi、`codex exec` 或其他 Tool，只能产生辅助结论；正式 C04 的冻结 Review Target、精确 Git Commit / HEAD、角色独立性、Review Record 和结论要求见工程总则第 38.7 节。

C04 默认使用 `INDEPENDENT_REVIEWER_PRIMARY`，不可用时使用 `INDEPENDENT_REVIEWER_FALLBACK`，具体 Model/Runtime/Harness 只从 `CURRENT_STATE.md` 解析。每次切换和每次复审都必须新建独立 C04 Session；不得复用实现或整改 Session，也不得因 Reviewer Provider、Model、Runtime 或 Harness 替换而改变评审输入、标准或结论格式。

某 Expert 实质参与当前整改方案时，C04 优先使用另一 Reviewer Provider。另一 Provider 不可用时，可使用同 Provider 的全新独立 Session，但必须保持上下文完全隔离。

Reviewer Provider 只是 Reviewer Model/Runtime/Harness 的运行选择属性，不是新角色、新 Owner 或新 Current Truth 来源。

## 41.4 Model / Runtime / Harness 替换后的恢复

逻辑 C00 在 Provider、Model、Runtime 或 Harness 切换时保持连续。物理上下文按岗位交互与可执行治理政策第 8 节执行 `KNOWLEDGE_CONTINUATION_CHECK` 或 `BASELINE_RELEARN`，并记录前后运行身份、Git Anchor、Task、Authority、检查结果和证据。切换不增加 Role、Tool、Gate 或批准权，也不能满足正式 C04 独立性。

## 41.5 独立 Session 请求与自动创建

何时必须建立独立 Session 由 `00_project/governance/PROJECT_ASSURANCE_CADENCE_POLICY.md` 唯一定义。请求格式由本节唯一维护。

```yaml
NEW_INDEPENDENT_SESSION_REQUEST:
  schema_version: "1.2"
  request_id: "{{UNIQUE_REQUEST_ID}}"
  action: "CREATE_INDEPENDENT_SESSION"
  independent_session_required: true
  reason_code: "{{ALLOWED_INDEPENDENCE_REASON}}"
  trigger_rule_reference: "{{AUTHORITATIVE_RULE_REFERENCE}}"
  trigger_evidence: "{{FACT_OR_RECORD_PROVING_THE_TRIGGER}}"

  caller:
    session_id: "{{CURRENT_SESSION_ID}}"
    role: "{{CURRENT_ROLE}}"
    dynamic_role_profile: "{{PROFILE_ID}}"
    interaction_id: "{{INTERACTION_ID}}"

  governance_identity:
    target_role: "{{C04|EXPERT|ADVISORY}}"
    formal_gate_authority: "{{C04_ONLY|NONE}}"
    execution_slot: "{{EXECUTION_SLOT}}"
    runtime: "{{RUNTIME_OR_HARNESS_NATIVE}}"
    provider_separation_required: false

  task:
    self_contained: true
    objective: "{{ONE_PRECISE_OBJECTIVE}}"
    exact_question: "{{QUESTION_TO_ANSWER}}"
    exact_git_target: "{{FULL_COMMIT_HASH_OR_NOT_APPLICABLE}}"
    applicable_baseline: "{{BASELINE_ID_OR_NOT_APPLICABLE}}"
    required_inputs:
      - "{{PATH_OR_CONTROLLED_REFERENCE}}"
    must_not_assume:
      - "{{EXCLUDED_CONTEXT_OR_UNAPPROVED_FACT}}"
    expected_output: "{{OUTPUT_CONTRACT}}"

  context_package:
    mode: "MINIMUM_SUFFICIENT_SELF_CONTAINED"
    package_complete: true
    include_full_chat_history: false
    include_private_reasoning: false
    handoff_or_review_record: "{{PATH_OR_NOT_APPLICABLE}}"

  permissions:
    filesystem: "{{READ_ONLY|SCOPED_WRITE}}"
    allowed_write_paths:
      - "{{PATH_OR_NONE}}"
    commit: false
    push: false
    pull_request: false
    release: false
    remote_mutation: false

  enforcement:
    mode: "{{PROCEDURAL_FALLBACK|TOOL_ENFORCED}}"
    evidence: "{{CONTROL_EVIDENCE_REFERENCE}}"

  authorization:
    contract_refs:
      independent_session_creation: "{{INDEPENDENT_SESSION_CREATION_AUTHORIZATION_ID}}"
      formal_c04_dispatch: "{{FORMAL_C04_DISPATCH_AUTHORIZATION_ID_OR_NOT_APPLICABLE}}"
      real_model_invocation: "{{REAL_MODEL_INVOCATION_AUTHORIZATION_ID_OR_NOT_APPLICABLE}}"
    required_action_classes:
      independent_session_creation: "INDEPENDENT_SESSION_CREATION"
      formal_c04_dispatch: "{{FORMAL_C04_DISPATCH|NOT_APPLICABLE}}"
      real_model_invocation: "{{REAL_MODEL_INVOCATION|NOT_APPLICABLE}}"
    applicability_evidence:
      independent_session_creation: "{{ALWAYS_APPLICABLE_EVIDENCE}}"
      formal_c04_dispatch: "{{APPLICABILITY_OR_NOT_APPLICABLE_EVIDENCE}}"
      real_model_invocation: "{{APPLICABILITY_OR_NOT_APPLICABLE_EVIDENCE}}"
    authority_owner: "{{AUTHORITY_OWNER}}"
    action_scope_target: "{{ACTION_SCOPE_AND_EXACT_TARGET}}"
    allowed_side_effects: "{{ALLOWED_SIDE_EFFECTS}}"
    forbidden_side_effects: "{{FORBIDDEN_SIDE_EFFECTS}}"
    source_type: "{{PREAUTHORIZED_GATE|EXPLICIT_CONFIRMATION}}"
    source_reference: "{{GATE_OR_CONFIRMATION_RECORD}}"
    validity: "ONE_INDEPENDENT_SESSION"
    consumption_event: "SESSION_DISPATCHED"
    terminal_state: "{{EXECUTION_COMPLETED|RESULT_UNKNOWN_RECONCILIATION_REQUIRED}}"
    retry_policy: "NO_AUTOMATIC_RETRY"
    dispatch_limit: 1
    capability_enablement_is_authorization: false

  placement:
    target: "RESOLVE_FROM_CONFIG"
    default_target: "CURRENT_AI_ENVIRONMENT"
    project_binding: "CURRENT_PROJECT"
    external_profile: "RESOLVE_FROM_CONFIG_OR_NULL"

  return_route:
    destination_session_id: "{{CALLER_SESSION_ID}}"
    result_record: "{{PATH_OR_MESSAGE_CHANNEL}}"

  failure_policy:
    automatic_retry: false
    on_failure: "RETURN_TO_CALLER"
```

机械要求：

1. 请求必须通过 Schema、Reason Code、触发证据、自足输入、权限、Authorization Contract 和唯一 `request_id` 校验；Session 创建、正式 C04 Dispatch 与真实 Model 调用是独立 Action Class，必须分别有适用授权；
2. 默认在当前 AI/Harness 的当前项目自动创建；
3. 同一环境的新 Session 也必须拥有真正独立上下文，不继承实现/整改私有推理；
4. 只有 `EXTERNAL_AI_TRANSFER_CONFIG.yaml` 已由负责人手动启用并选择外部 Profile 时，才允许把独立 Session 建到外部 AI；
5. 本地创建失败不得自动 fallback 到外部；应输出 `LOCAL_INDEPENDENT_SESSION_CREATION_UNAVAILABLE` 和可复制的手动创建指令；
6. 子 Session 权限不得超过 Caller；一个 `request_id` 最多创建一次；失败或超时不得自动再次付费调用；
7. 正式 C04 还必须满足 Review Readiness，且 `formal_gate_authority` 只能由正式分配的 C04 使用。
8. `authorization.source_reference` 必须指向适用于本请求的预授权 Gate 或明确确认记录，`validity` 必须为 `ONE_INDEPENDENT_SESSION`，`dispatch_limit` 必须为 `1`；外部能力开关、Profile 启用或技术可调用性本身都不是调用授权。
9. 正式 C04 还必须记录 `00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml` 定义的独立性证据：新 Session、排除的实现/整改 Session、上下文包、精确 Target、Target 只读、允许写入范围、Git/远程写入禁止和评审前后 Target 状态。
10. `INDEPENDENT_SESSION_CREATION` 对本请求始终适用；只创建独立 Session 而不发起正式 C04、也不触发真实 Model 调用时，后两项必须分别标记 `NOT_APPLICABLE` 并给出证据。
11. 当请求把任务作为正式 C04 发出时，`FORMAL_C04_DISPATCH` 适用；如果该 Operation 只完成 Dispatch、没有触发新的真实 Model 推理调用，则 `REAL_MODEL_INVOCATION` 可以标记 `NOT_APPLICABLE`，但必须给出证据。
12. 当一次 Operation 同时创建独立 Session、发起正式 C04 并触发真实 Model 调用时，三项 Action Class 必须同时出现，并分别引用各自的有效 Authorization Contract；任何一项不得隐含另一项。
13. `NOT_APPLICABLE` 不是授权 ID。Schema 校验必须拒绝以下请求：适用 Action Class 缺少独立合同、合同与 Action Class 不匹配，或 `NOT_APPLICABLE` 缺少适用性证据。

## 41.6 当前 Session 外部 AI 调用与独立 Session 的分离

必须区分：

```text
CURRENT_SESSION
-> EXTERNAL_AI_TOOL_CALL
-> AUXILIARY / ADVISORY RESULT
-> RETURN_TO_CURRENT_SESSION
```

和：

```text
INDEPENDENCE_TRIGGER
-> NEW_INDEPENDENT_SESSION_REQUEST
-> NEW LOCAL OR MANUALLY-CONFIGURED EXTERNAL SESSION
```

前者在配置和当前授权允许时可以静默调用，不创建新的治理 Session，由 Caller 提供上下文、保持任务 Owner、判断结果并继续工作；它不得产生正式 C04 Gate Decision。后者只用于权威规则明确要求独立性、且最小输入包已经自足的任务。

外部 AI 当前调用、确认、预算、重试、并发和放置值只由 `00_project/governance/EXTERNAL_AI_TRANSFER_CONFIG.yaml` 维护。本文件只维护交互语义和请求格式。
