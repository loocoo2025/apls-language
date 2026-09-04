# ADR-APLS-001：Compiler 实现语言与 Parser 技术

- ADR ID：`ADR-APLS-001`
- 状态：`ACCEPTED`
- 日期：`2026-09-02`
- 决策 Owner：`HUMAN_PROJECT_OWNER`
- 确认依据：`HDP-APLS-006 / OPTION A`
- 输入：`DEC-004`、`DEC-009`、`DES-APLS-COMPILER-001`

> 本 ADR 是当前有效架构决定，但仍不等于产品 Baseline，也不授权安装依赖或开始实现。

## 1. 背景

APLS Compiler MVP 必须把同一 Source Graph 确定地处理为以下互斥结果之一：

```text
REJECTED + stable diagnostics

或

ACCEPTED + verified Canonical JSON IR
```

实现技术必须优先服务 `DEC-004`：无法得到唯一语法或语义解释时拒绝输入，禁止由规则顺序、运行时猜测、静默恢复或 AI 补全产生隐式选择。

本 ADR 只比较 Compiler 主实现语言与 Parser 生成技术。JSON 库、Schema Validator、CLI 库、依赖版本、目标后端、语义哈希和发行平台矩阵均留给后续设计。

## 2. 决策准则

按以下顺序评估；顺序高于开发便利性：

1. Grammar 冲突能否在生成或构建阶段失败关闭；
2. 类型系统能否机械表达互斥 Stage、Artifact 和 Error；
3. 是否便于实现显式排序、Canonical JSON 和稳定诊断；
4. 是否便于交付不依赖项目运行时的命令行工具；
5. 开发、调试和维护成本；
6. Parser 工具与运行时的版本、依赖和升级风险。

所有方案都必须执行同一 Parser Gate：

- Parser 生成过程出现未解决冲突、警告或生成失败时，构建失败；
- 不允许用“规则先后顺序”作为 APLS 的隐式消歧语义；
- 运算符优先级和结合性只能来自已批准 Grammar 中的显式规则；
- Error Recovery 只允许产生诊断，Recovery/Poison 节点不得进入 Resolver；
- 生成代码不得手工修改，Grammar 与固定工具版本是唯一生成输入；
- Conformance 必须证明同一输入不会产生两个可接受 Parse Tree。

## 3. 候选方案比较

| 准则 | A：Rust + LALRPOP | B：Go + goyacc | C：Java + ANTLR4 |
|---|---|---|---|
| Grammar 冲突失败关闭 | **强**：默认 LR(1)，适合把冲突暴露在 Parser 生成阶段 | 中：yacc 路线可报告冲突，但需额外包装为零冲突构建门槛 | 中：可将工具 Warning 转 Error，但 ALL(*) 的运行时预测与部分“第一 Alternative 胜出”行为需要额外歧义监听和测试约束 |
| 互斥状态类型建模 | **强**：Enum + Exhaustive `match` 可直接表达 Stage/Result/Artifact 联合类型 | 中：可用接口与显式 Tag 表达，但穷尽性约束较弱 | 中强：sealed 类型可表达封闭层级，但需统一 Java 版本与编码约定 |
| 确定序列化 | 强：必须自建显式 Canonicalizer；语言不会自动保证 Map 顺序 | 强：同样必须显式排序，不能依赖 Map 迭代 | 强：同样必须显式排序，不能依赖 Map 迭代 |
| CLI 分发 | 强：可交付本机可执行文件 | **强**：工具链直接生成可执行文件，跨平台路线简单 | 中：通常要求匹配的 JVM 或额外运行时打包 |
| 开发成本 | 中：所有权与生命周期增加学习和编译成本 | **低**：语言与工具链较简洁 | 中：ANTLR/IDE 生态成熟，但 Tool、Runtime 和 JVM 组合更重 |
| 依赖与升级风险 | 中：LALRPOP 是外部 Crate，需锁定版本并验证维护性 | 中：goyacc 位于 `golang.org/x/tools`，当前模块仍是 pre-v1，需锁定工具版本 | 中高：ANTLR 明确要求 Tool 与 Runtime 同版，Minor Release 也可能包含 Breaking Change并要求重新生成 Parser |

“确定序列化”不是任一语言的免费性质。无论选择哪一项，Canonicalizer 都必须显式排序、拒绝未知字段，并从待发布 JSON Byte 重新解析验证。

## 4. 方案 A：Rust + LALRPOP

### 优点

- LALRPOP 官方定位为 Rust 的 LR(1) Parser Generator，默认采用 LR(1)；其静态 Grammar 路线与“零冲突才能生成”门槛最贴合；
- Rust Enum 可把 `Accepted/Rejected`、Compiler Stage 和各类 Artifact 表达为互斥 Variant；`match` 要求穷尽处理，减少新增状态被静默遗漏；
- `Cargo.lock` 记录精确依赖版本，适合作为后续可重复构建控制的一部分；
- 可交付本机 CLI，不要求目标机器存在 JVM。

### 代价与风险

- Rust 所有权、生命周期和较长编译反馈会增加 MVP 初期成本；
- LALRPOP 文档仍明确存在未完成部分，不能把工具选择当作 Grammar 正确性的证明；
- LALRPOP 与 Runtime 版本必须锁定；升级需重新生成 Parser 并运行完整 Grammar Conformance；
- Lexer 冲突和语义多解仍需 APLS 自身的静态检查与测试，LR(1) 不能替代 Resolver/Semantic Analyzer。

## 5. 方案 B：Go + goyacc

### 优点

- Go 官方工具文档说明 goyacc 生成 Go Parser，生成 Parser 可重入；
- Go 构建可直接生成可执行文件，开发和跨平台交付路径简洁；
- 语言与构建模型简单，适合快速形成最小 Compiler 骨架。

### 代价与风险

- goyacc 文档较薄，零冲突门槛、诊断归一化和生成物稳定性需要项目脚本额外封装；
- `golang.org/x/tools` 当前仍为 pre-v1 模块，工具升级必须固定版本并单独验证；
- Go 缺少与 Rust Enum + Exhaustive Match 同等直接的代数数据类型约束，Stage/Artifact 不变量更多依赖封装、审查和测试；
- yacc 风格优先级声明若使用不当，可能把冲突“解决”为隐式策略，因此必须逐项追溯到正式 Grammar。

## 6. 方案 C：Java + ANTLR4

### 优点

- ANTLR4 从 Grammar 生成 Parser、Parse Tree、Listener/Visitor，工具与 IDE 生态成熟；
- 官方工具提供 `-Werror`，可把 Parser 生成 Warning 提升为构建失败；
- Java 的封闭类型与模式匹配可表达受控的 Compiler Stage 模型。

### 代价与风险

- ANTLR4 的 ALL(*) 运行时预测能力很强，但官方资料同时记录了左递归等场景中的“first alternative wins”消歧机制；APLS 不能把这种顺序当作规范语义，必须增加精确歧义报告、监听和反例测试；
- 官方版本政策说明 Minor Release 可能有 Breaking Change，且每次 Release 都应重新生成 Parser；Tool 与 Runtime 必须同版锁定；
- JVM/Runtime 使单文件 CLI 分发与运行环境控制比原生 Rust/Go 路线更重；
- 多目标生成能力不是 APLS 0.1 MVP 需求，不能抵消额外的歧义控制成本。

## 7. 最终决策

采用 `OPTION A：Rust + LALRPOP（默认 LR(1)）`。

核心理由不是性能，而是可执行的不变量：Parser 生成阶段暴露 Grammar 冲突，Rust 类型系统继续把 `FAILED/PUBLISHED`、内部 AST 与 Verified IR、Source/Tool/Internal Error 表达为互斥状态。这条路线最直接地服务“拒绝歧义”和失败关闭。

`HDP-APLS-006` 已于 `2026-09-02` 明确批准 `OPTION A`。Go + goyacc 与 Java + ANTLR4 不作为 APLS 0.1 Compiler MVP 的当前实现路线；未来改变必须通过新的 ADR 正式替代本决定。

## 8. 已接受约束

- Compiler 主实现语言：Rust；Parser Generator：LALRPOP 默认 LR(1)；
- Parser Generator 与 Runtime 使用同一锁定版本，生成输入与输出进入可追溯构建；
- `Cargo.lock` 进入版本控制；实现工作包不得使用浮动 Git Dependency；
- Parser 生成的任一冲突或 Warning 均使构建失败；不得配置预期非零冲突；
- Parser Error Recovery 结果只能流向 Diagnostic Engine；
- Rust 类型设计不得用一个带大量 Optional 字段的通用对象代替互斥 Stage Artifact；
- 具体 Rust Edition、MSRV、LALRPOP 版本、JSON/Schema/CLI 库由后续实现设计候选提出，不由本 ADR 擅自冻结。

## 9. 影响范围

- 需求：不改变 PRD；
- 架构：确定 Compiler 主实现与 Parser 生成路线；
- 设计：后续需要模块边界、依赖版本和 Build Gate 详细设计；
- 代码：尚未授权创建；
- 测试：后续必须建立零冲突 Grammar 工具 Gate、分析候选完整性、Canonical Frame 收敛唯一性与确定输出 Conformance；
- 部署：优先规划原生 CLI Artifact，平台矩阵另行裁定。

## 10. 重新评估条件

出现以下任一情况时重新评估，而不是静默替换技术：

- LALRPOP 无法表达已批准 Grammar 且必须引入未获准的隐式消歧；
- 维护状态、许可证或供应链风险不能满足发布要求；
- 实测表明 Rust 路线无法达到已批准的交付约束；
- APLS 语言边界发生经批准的重大变化。

## 11. 明确不在本决策内

- Rust/LALRPOP/依赖的精确版本；
- Canonical JSON、JSON Schema、CLI 或测试库；
- 语义哈希算法；
- FreeRTOS 或其他参考后端；
- LSP、IDE、Plugin、Daemon、缓存和并行编译；
- Baseline Adoption、Formal C04、Commit、Push、Release 和 Formal Seal。

## 12. 官方依据

- [LALRPOP 官方仓库：默认 LR(1) 与项目能力](https://github.com/lalrpop/lalrpop)
- [LALRPOP 官方 Error Recovery 教程](https://lalrpop.github.io/lalrpop/tutorial/008_error_recovery.html)
- [Rust 官方 Enum 与 Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [Rust 官方 `match` 穷尽性说明](https://doc.rust-lang.org/stable/core/keyword.match.html)
- [Cargo 官方 `Cargo.toml` 与 `Cargo.lock` 说明](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html)
- [Go 官方 goyacc 文档](https://pkg.go.dev/golang.org/x/tools/cmd/goyacc)
- [Go 官方 Build/Install 教程](https://go.dev/doc/tutorial/compile-install)
- [ANTLR4 官方仓库与版本政策](https://github.com/antlr/antlr4)
- [ANTLR4 官方 Tool Options：`-Werror`](https://github.com/antlr/antlr4/blob/dev/doc/tool-options.md)
- [ANTLR4 官方 Issue：左递归与 first-alternative 消歧背景](https://github.com/antlr/antlr4/issues/1398)

## 13. 替代关系

- Supersedes：无；
- Superseded by：无。
