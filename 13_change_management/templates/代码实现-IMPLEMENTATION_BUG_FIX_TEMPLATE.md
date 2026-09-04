# 纯实现 Bug 修复模板

> 当前产品需求、ADR、架构、接口和详细设计本身正确，只是代码没有正确实现。

## 核心原则

> 修代码，不改需求。不得为了让错误代码“合理化”而反过来修改正式设计或测试预期。

## 1. Bug 信息
- Bug ID：`BUG-XXX`
- 日期：
- 发现环境：
- 当前 Baseline ID：
- 当前 Git Commit：
- 严重度：`P0 / P1 / P2 / P3`

## 2. 现象
- 实际行为：
- 预期行为：
- 复现步骤：

## 3. 正式依据
- 产品需求：
- 系统需求：
- ADR：
- 架构：
- 详细设计：
- 接口：
- 测试：

## 4. 分类
```text
BUG_CLASS: IMPLEMENTATION_DEFECT

REQUIREMENT_CHANGE: NO
ARCHITECTURE_CHANGE: NO
DESIGN_CHANGE: NO
INTERFACE_CHANGE: NO
```

如果上述任一项实际为 `YES`，停止本流程，转入 `FORMAL_CHANGE_DECISION_TEMPLATE.md`。

## 5. 根因
- 根因：
- 根因层级：`CODE / CONFIGURATION / BUILD / DEPLOYMENT / ENVIRONMENT / TEST_HARNESS / OTHER`

## 6. 最小修复范围
允许修改：
-

禁止修改：
- 产品需求；
- 已批准系统需求；
- ACCEPTED ADR；
- 当前架构；
- 当前接口行为；
- 与本 Bug 无关的模块；
- 测试预期来适配错误行为。

## 7. 回归测试
- Regression Test ID：`TC-XXX`
- 防止的根因：
- 为什么现有测试没有发现：

## 8. 修复流程
```text
复现 Bug
↓
确认正式预期
↓
建立/补充最小回归测试
↓
修改代码
↓
运行相关测试
↓
必要时运行 ASan / UBSan / 其他验证
↓
确认无回归
↓
关闭 Bug
```

## 9. 禁止行为
- 修改需求来适应 Bug；
- 修改 ADR 来解释 Bug；
- 修改架构来合理化错误实现；
- 删除失败测试；
- 降低测试阈值只为变绿；
- 大范围无关重构。

## 10. 给 AI 的直接指令
```text
这是一个实现 Bug 修复，不是需求或架构变更。

当前正式需求、ADR、架构、接口和详细设计继续有效。

请确认正式预期、复现问题、定位根因，只做最小正确修复，并为真实 Bug 增加最小有效回归测试。
不得通过修改需求、设计或测试预期来适应错误代码。

如果调查发现真正的问题来自需求、架构或设计错误，请停止本流程，重新分类为正式变更。
```
