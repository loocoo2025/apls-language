# TASK-018 实现增量非正式独立复审

## 1. 身份、范围与结论

- Review ID：`IIR-APLS-TASK018-IMPLEMENTATION-001`
- Target ID：`IIR-TARGET-APLS-TASK018-IMPLEMENTATION-001`
- Review Line：`INFORMAL_INDEPENDENT`
- Review Scope：`DELTA_ONLY`
- Reviewer：全新隔离独立 Reviewer；`private_context_inherited: false`
- Gate Authority：`ADVISORY_ONLY`
- 复审结论：`CHANGES_RECOMMENDED`
- 唯一写入：本报告

本轮只复审 TASK-019 R3 冻结后形成的 TASK-018 CNL 实现增量：简体中文 Lexer/Lattice/Stream、唯一 LALRPOP Grammar、AST、绑定/类型/单位、Canonical Frame/IR、资源登记、诊断、Schema/跨节点复验、Unicode 依赖与 CLI 四命令。直接影响闭包仅包括 `DEC-017/019/020/023`、`WP-APLS-CNL-C03-001` 八组行为证据、公开 Source 接受/拒绝边界及旧 DSL 不可达性。未扩张到全项目或 TASK-019 设计整改过程。

## 2. Target 完整性

摘要严格按 Target 第 1 节算法复算：67 个文件的 `<sha256><two ASCII spaces><relative path><LF>` 记录按 C Locale 整行 Byte 升序排列，再对完整记录流计算 SHA-256。

| 时点 | 文件数 | Target Set SHA-256 | 结果 |
|---|---:|---|---|
| 开始 | 67 | `b4e377fe844ffd55403bf8a74bb4954107fef4dfe3719e2dfa461e1bd3952658` | MATCH |
| 报告写入前 | 67 | `b4e377fe844ffd55403bf8a74bb4954107fef4dfe3719e2dfa461e1bd3952658` | MATCH |
| 结束 | 67 | `b4e377fe844ffd55403bf8a74bb4954107fef4dfe3719e2dfa461e1bd3952658` | MATCH |

本报告不属于 Target Set，不产生自引用。

## 3. 继承项

`IIR-APLS-TASK019-REREVIEW-003` 的 BF-02～BF-10 与 NF-01～NF-05 全部记为 `INHERITED_CLOSED`。本轮不重复复审这些设计整改，也不重新打开其原始 Finding；第 4 节记录的是当前实现与已关闭契约的新直接冲突。

| 继承组 | 状态 |
|---|---|
| BF-02～BF-10 | `INHERITED_CLOSED` |
| NF-01～NF-05 | `INHERITED_CLOSED` |

## 4. 阻断观察项

### IO-01 — 实际 Cargo Feature 闭包与获批边界不一致

- 实现证据：`07_src/Cargo.toml:26` 正确将直接依赖固定为 `unicode-normalization =0.1.25, default-features=false`；`07_src/Cargo.lock:1083-1096,1146-1153` 也固定了三个获批版本与 Checksum。但独立 `cargo tree -p apls-compiler -i tinyvec -e features --locked --offline` 明确显示 `tinyvec` 同时激活 `alloc`、`default` 和由 `alloc` 引入的 `tinyvec_macros`；C03 记录 `11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md:33-35` 只声称 `alloc -> tinyvec_macros`。
- 违反契约：`04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md:44-54` 冻结 `tinyvec` “只启用 `alloc`”；`00_project/ai_context/HUMAN_DETERMINATION_018.md:46-59` 要求实际 Feature 闭包与候选不一致时立即重新阻塞。即使 `tinyvec/default` 当前展开为空集，它仍是 Cargo 报告的实际激活 Feature，不能从审批证据中静默删除。
- 影响：`DEC-023` 依赖边界和 C03 依赖闭包主张尚未被实际 Cargo Feature 解析支持。
- 可验证关闭条件：实际 `cargo tree -e features --locked` 与获批精确闭包一致，或由有权角色对包含空 `default` 的实际闭包作出显式新裁决，并更正 C03 证据。

### IO-02 — 公开 Parse 边界宽于唯一 EBNF

- 实现证据：`07_src/crates/apls-compiler/src/cnl_lexer.rs:91-92` 把非 Grammar 字面量 `且` 与 `和` 都映射为同一 `AndList` Token，而 `07_src/crates/apls-compiler/src/apls_grammar.lalrpop:51-55` 会因此接受用 `且` 连接的状态列表。另外，Lexer 在 `cnl_lexer.rs:315-326,339-341,491-505` 对任意 Token 边界跳过任意多个结构空白，数值/单位的 `opt-space` 约束到 `cnl_pipeline.rs:985-992,1079-1086,1109-1126` 才检查；但 `cnl_pipeline.rs:184-189` 在 `CompileThrough::Parse` 于此前直接成功返回。
- 违反契约：`04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf:44-47` 的状态列表连接词只有 `和`；同文件 `:89-93,167-169` 把百分比/温度/时长限定为零或一个 ASCII 空格。`APLS_0.1_COMPILER_MVP_DESIGN.md:149-165` 把 `parse` 定义为到 `PARSED` 的公开 Grammar Gate，不能把 Grammar 违规推迟到 Check。
- 影响：未获批 Source 可在 `apls parse` / `diagnose --through parse` 中被当作合法语法，破坏唯一公开语言接受集。
- 可验证关闭条件：删除所有非冻结字面量 Token 映射，并在 Parse Gate 内执行所有 `ws1/opt-space` 源敏感约束；添加通过四条公开命令路径的正反例。

### IO-03 — 等价候选收敛时丢失候选 Provenance

- 实现证据：`07_src/crates/apls-compiler/src/cnl_pipeline.rs:190-225` 以 Canonical Frame Byte 为 Key，但对每个等价候选直接执行 `canonical.insert(..., frame)`，后一个候选覆盖前一个 Frame 及其 Provenance。`:253-266` 只会合并已被保留 Frame 的 Provenance。现有测试 `cnl_pipeline.rs:2932-2937` 仅断言最终 Rule 数量为 1，没有断言两条 Tokenization 的 Role Span 并集。
- 违反契约：`04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md:257-264` 与 `04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md:162-167` 要求同一 Canonical Frame 的所有 Tokenization/Parse/Bound Candidate Provenance 稳定并集，不保存“获胜 Parse”。
- 影响：Verified IR `source_map` 不再是所有收敛候选的无损映射；同一语义可因候选遍历顺序保留不同 Role Span。
- 可验证关闭条件：按 Canonical Key 合并而非覆盖 Provenance，并使多 Tokenization 收敛用例断言每个候选的所有唯一 Role Span 按规范顺序进入唯一 Source Map Entry。

### IO-04 — Verified IR 跨节点复验没有重新执行批准的语义闭包

- 实现证据：`07_src/crates/apls-compiler/src/cnl_pipeline.rs:1681-1687` 对 Property 只检查 Unit Ref 存在；`:1927-1986` 对 Comparison 只检查 Property Ref 与 Quantity Unit Ref 存在；`:1745-1767` 没有重新检查 Transition 的不同状态、可达性和同 Trigger 冲突；`:1788-1801` 只对 Acceptance Deadline 调用 Unit 存在性检查。`cross_validate` 在 `:1915-1924` 随后直接成功返回，也没有检查 `units[]` 精确 Materialization Closure。现有篡改测试 `:2777-2800` 只覆盖 Header、缺失 Unit Ref、Pointer 和未知字段。
- 违反契约：`04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md:268-288` 明确要求对实际待交付 Byte 复验 Property/Unit/Literal 矩阵、Transition 局部及全局语义、正 Time Deadline 和精确 Unit Closure；任一失败不得构造 Verified Artifact。
- 影响：Publisher 前的独立失败关闭边界实际只是部分复验，C03 记录 `:13-14` 的“Schema 及跨节点复验”和“只有复验成功才构造 VerifiedArtifact”因此过度概括。
- 可验证关闭条件：从 Canonical Output Byte 重新执行第 9 节列出的每个跨节点不变量，并通过逐项篡改测试证明每个违规在 `VerifiedArtifact` 构造前使用稳定诊断失败。

### IO-05 — `bound_frame_candidates` 在绑定成功前登记

- 实现证据：`07_src/crates/apls-compiler/src/cnl_pipeline.rs:192-203` 对每个 Parsed Candidate 先增加 `Resource::BoundFrames`，然后才进入同时执行 Owner、Action Target、类型和单位检查的 `type_and_normalize`。因此后续绑定失败的 Candidate 也已被记为 Bound。
- 违反契约：`04_design/compiler/APLS_0.1_CNL_RESOURCE_AND_ORDERING_PROFILE.md:178-185` 要求只在全部引用、类别和 Owner 成功绑定后计一个完整 Bound Candidate，每级都不计失败 Candidate。
- 影响：七类候选资源中的 Bound 计数对象与首次超限语义不符合已批准 Profile；私有 Ledger 单元测试只证明登记器本身的 Payload，不能证明生产 Pipeline 在正确 Stage 调用它。
- 可验证关闭条件：分离 Bind/Type/Normalize Stage，仅在各级完整成功后调用同一生产登记函数；用一个绑定失败 Candidate 和一个绑定成功/类型失败 Candidate 断言精确计数差异。

### IO-06 — 稳定诊断的 Primary/Related Span 契约未落实

- 实现证据：`07_src/crates/apls-compiler/src/cnl_pipeline.rs:1277-1289` 以整个 State Declaration Sentence 作为 E1402 Primary Span，且没有添加该 State Model 的全部 Transition Related Span；`:888-896,1128-1130` 的 E1307 通过恒等 fallback 使用整个 Acceptance Sentence，而非 Deadline Span；`:2661-2702` 虽生成 E1310 Witness，但 Primary Span 直接使用整句传入值，没有计算 Differing Role Provenance 的最小包含区间。
- 违反契约：`04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md:113-137` 冻结 E1402/E1403 的 Primary/Related Span；`04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md:124-145` 冻结 E1307 时间短语范围及 E1310 的最小分歧范围。
- 影响：同一根因的诊断 Byte、排序和上下文证据与机器契约不一致；现有测试的“产生正确 Code”不足以证明精确诊断。
- 可验证关闭条件：保留 State Item/Deadline/Differing Role 原始 Span，按冻结顺序构造 Primary/Related Span，并对整个诊断 Envelope Byte 进行重复执行断言。

### IO-07 — stdout 写入失败没有返回 `APLS-T0005`

- 实现证据：`07_src/crates/apls-cli/src/main.rs:7-13` 在 stdout 的 `write_all` 失败时只返回 Exit `2`，没有在 stderr 生成或写出 `APLS-T0005`诊断。
- 违反契约：`04_design/compiler/APLS_0.1_COMPILER_MVP_DESIGN.md:157-168` 与 `04_design/compiler/APLS_0.1_IMPLEMENTATION_FOUNDATION_DESIGN.md:204-213` 均要求 stdout 传输失败返回 `APLS-T0005` / Exit `2`，已暴露前缀无效。
- 影响：程序只保留了退出码，丢失已批准的稳定 Tool Diagnostic 身份。
- 可验证关闭条件：在可控的 stdout 中途失败测试中，等待进程结束后必须观测 Exit `2` 且 stderr 含稳定 `APLS-T0005`，任何 stdout 前缀不被标记为成功 Artifact。

## 5. 必答问题判定

| # | 判定 | 独立回答 |
|---:|---|---|
| 1 | `PARTIAL` | 公开 Pipeline 与默认测试覆盖 Profile、七类声明、Rule/Transition/Invariant/Acceptance/Informative，`lib.rs` 没有连入旧 DSL 模块；但 IO-02 证明实际接受边界大于唯一公开 Grammar。 |
| 2 | `PARTIAL` | 实现不使用概率、LLM 或语义排名，并在 Canonical Frame Key 层实现 `0/1/>1`；但 IO-03 在等价类内以覆盖代替 Provenance 并集，仍形成了一个“获胜 Parse”的非语义结果。 |
| 3 | `PARTIAL` | 默认测试支持样例化的术语/类别/Owner 具体错误与 E1310 两个 Witness；但 IO-06 证明 E1310 Primary Span 不符机器契约。 |
| 4 | `PARTIAL` | Declaration Bootstrap、Owner/类别、状态比较、Transition 可达/冲突与名义单位主路已实现；但 IO-04/06 证明发布前复验和诊断 Span 尚未闭合。 |
| 5 | `FAIL` | Canonical ID/Hash/JSON、Schema 与部分引用/Source Map 检查已实现；IO-03 破坏候选 Provenance 并集，IO-04 破坏完整跨节点失败关闭。 |
| 6 | `FAIL` | 资源 Ledger、T0007 Payload、私有 `#[cfg(test)]` 注入和失败后禁止 Artifact 的主体边界存在；但 IO-05 的 Bound Candidate 生产计数对象错位。 |
| 7 | `FAIL` | NFC 17.0.0、Binary 断言、IR Header、精确版本/Checksum/License 和 Rust 1.86 离线构建均有可复验证据；但 IO-01 的实际 Feature 闭包与获批证据不一致。 |
| 8 | `FAIL` | 四命令、退出码主路、stdout/stderr 分配、Envelope、emit 前 Source 复读和原子文件发布已实现；但 IO-02 使 Parse 语法 Gate 过宽，IO-07 丢失 stdout 写失败的 T0005。 |
| 9 | `NOT_SUPPORTED` | 本轮成功复现了 C03 的默认 `check/test` 数量和 MSRV 构建，但 C03 第 1 节的完整 Provenance/跨节点复验主张及第 3 节的 Feature 闭包主张分别被 IO-03/04/01 反证；默认回归通过不足以支持 `PASS / OUTPUT_READY_CANDIDATE`。 |
| 10 | `YES` | IO-01～IO-07 均直接位于 TASK-018 实现增量或其 C03 验证闭包，会阻止进入 `READY_FOR_REVIEW` 或后续 C05 验证。 |

## 6. 独立机械核验

| 检查 | 结果 |
|---|---|
| `cargo check --workspace --locked --offline` / Rust 1.98.0 | PASS |
| `cargo check --workspace --locked --offline` / Rust 1.86.0 | PASS |
| `cargo test -p apls-compiler --locked --offline` / Rust 1.98.0 | PASS：20 passed，1 ignored |
| `cargo test -p apls-cli --locked --offline` / Rust 1.98.0 | PASS：5 passed，1 ignored |
| `cargo tree -p apls-compiler -i tinyvec -e features --locked --offline` | 观测到 `tinyvec/alloc`、`tinyvec/default`、`tinyvec/tinyvec_macros` |
| Target Digest（开始/写入前/结束） | 三次 MATCH |

本轮未重复百万级资源用例和 Unicode 20,034 行官方用例；两项仍按 C03 证据处理，不被本 Reviewer 冒充为本轮独立重跑。默认测试通过仅证明现有用例未回归，不抵消第 4 节中以契约/源码直接对照确认的缺口。

## 7. 结论与权限边界

结论：`CHANGES_RECOMMENDED`。

本报告仅是非正式独立建议，不批准 TASK-018、不修改设计/实现/测试/Cargo/治理状态，不建立 Baseline，不发起 Formal C04、C05、Commit、Push、Release 或 Formal Seal。报告完成后停止，不参与整改。
