# TASK-019 第三轮实施前非正式独立再复审

## 1. 评审身份、范围与结论

- Review ID：`IIR-APLS-TASK019-REREVIEW-003`
- Review Line：`INFORMAL_INDEPENDENT`
- Reviewer：全新独立 Reviewer；未参与三轮 TASK-019 整改设计，不是正式 C04
- Target ID：`IIR-TARGET-APLS-TASK019-003`
- Target Access：`READ_ONLY`
- Review Scope：`DELTA_ONLY`
- 范围裁决：项目负责人明确要求以本轮增量复审覆盖 Target R3 原第 3 节过宽问题集
- 评审结论：`READY_FOR_HUMAN_DETERMINATION`
- Gate 权威：`ADVISORY_ONLY`
- 唯一写入：本报告

本轮只复审 NF-05、其关联的 BF-06/BF-10/NF-03、Resource Profile / Implementation Work Package / Foundation / CNL-C012 的第三轮改动、NO-04 残留清理，以及 Target/实现/Cargo 未越权变化。BF-02/03/04/05/07/08/09 与 NF-01/02/04 直接继承 `IIR-APLS-TASK019-REREVIEW-002` 的 `CLOSED`，本轮未重新打开。Schema 未属本轮改动，不重复执行完整 Schema 复审，只由 Target Digest 保护其精确内容。

## 2. Target 完整性证据

Target 摘要严格按 Target 文件第 1 节算法计算：每个文件形成 `<sha256><两个 ASCII 空格><relative path><LF>`，按 C Locale 整行 Byte 升序排序后，对完整记录流计算 SHA-256。

| 时点 | 文件数 | Target Set SHA-256 | 结果 |
|---|---:|---|---|
| 开始 | 40 | `05a9e0c556ef87f72ae5f2f21f9cbae9a75b3e59fa3c474da838a71e975fc801` | MATCH |
| 报告写入前 | 40 | `05a9e0c556ef87f72ae5f2f21f9cbae9a75b3e59fa3c474da838a71e975fc801` | MATCH |
| 结束 | 40 | `05a9e0c556ef87f72ae5f2f21f9cbae9a75b3e59fa3c474da838a71e975fc801` | MATCH |

本报告不属于 Target，不产生自引用。

## 3. BF-02～BF-10 判定

| Finding | 判定 | 本轮依据 |
|---|---|---|
| BF-02 | `CLOSED` | `INHERITED_CLOSED`；本轮未触及、未重新打开。 |
| BF-03 | `CLOSED` | `INHERITED_CLOSED`；本轮未触及、未重新打开。 |
| BF-04 | `CLOSED` | `INHERITED_CLOSED`；本轮未触及、未重新打开。 |
| BF-05 | `CLOSED` | `INHERITED_CLOSED`；本轮未触及、未重新打开。 |
| BF-06 | `CLOSED` | 原计数对象、顺序和 Span 保持唯一；第三轮把验收证据正确分为三类可达 CLI 证据和四类受支配私有 Ledger 证据，不再承诺不可达的独立黑盒路径。 |
| BF-07 | `CLOSED` | `INHERITED_CLOSED`；本轮仅确认 NO-04 措辞清理，未重做 Unicode/Schema 全量复审。 |
| BF-08 | `CLOSED` | `INHERITED_CLOSED`；本轮未触及、未重新打开。 |
| BF-09 | `CLOSED` | `INHERITED_CLOSED`；本轮未触及、未重新打开。 |
| BF-10 | `CLOSED` | T0007 公共可观测承诺现在只覆盖真正可达的三类资源；四类受支配资源的同一生产登记函数、Payload/Stage/Span 和下游 Artifact 禁止由私有 Ledger 验证，生产 Source 同时验证先行 Token T0007 立即终止。 |

## 4. NF-01～NF-05 判定

| Finding | 判定 | 本轮依据 |
|---|---|---|
| NF-01 | `CLOSED` | `INHERITED_CLOSED`；Declaration Bootstrap 本轮未改、未重新打开。 |
| NF-02 | `CLOSED` | `INHERITED_CLOSED`；诊断总排序本轮未改、未重新打开。 |
| NF-03 | `CLOSED` | 全部 Syntax Node Span 仍由 Resource Profile 第 5.1 节唯一定义；CNL-C012B 以初值 `999,995` 和七节点顺序使第二个 `TypedValue` 成为第 `1,000,001` 个登记对象，其 Primary Span 唯一；该证据明确为私有 Module 测试，不再冒充公共 CLI 可达性。 |
| NF-04 | `CLOSED` | `INHERITED_CLOSED`；本轮只检查 NO-04 残留，未重新打开。 |
| NF-05 | `CLOSED` | 资源数值和生产 T0007 语义未改；公共可达与受支配防御性证据已分层，四条支配不变量、生产检查保留和私有 Ledger 限制相互一致。 |

## 5. NF-05 增量复审

### 5.1 三类 `PUBLIC_SOURCE_REACHABLE`

1. `candidate_lattice_edges`：可在单 Entry Source 内为大量不同 Entity 声明相同显示名的 Owner-scoped State，再在同一行为句的多个 State Reference Span 处产生高多重度 Edge。一个构造性家族使用 `10,000` 个紧凑 ASCII Owner 和 `100` 个相同 State Reference，仅这些 State Edge 就为 `1,000,000`；包含 Profile、Entity/State 声明和 Rule 的 Source 只需 `960,602` Byte，低于单 Source 上限。该 Sentence 先完成 Lattice Edge 登记再枚举 Stream，因而 Stream 上限不会必然先行支配。生成器可用较小 Owner 组和单 Edge 填充精确抵达总边界。
2. `complete_token_streams`：同一 Sentence 中的多个二候选 State Reference 形成独立组合；12 个二候选位置得到 `2^12=4,096` 条完整 Stream，增加一个候选即可在确认第 `4,097` 条时触发 T0007。相关 Edge 数和前 `4,096` 条 Stream 的 Token Occurrence 均显著低于各自上限；第 `4,097` 条在进入 Parser/登记 Token 前终止。
3. `candidate_token_occurrences`：重复使用每句不超过 `4,096` 条 Stream 的短 Sentence，可以在少量 Source Byte 内让全文档 Token Occurrence 累积到 `1,000,000/1,000,001`；每条 Stream 的全部 Token 在 Parser 前登记，所以超限时还未登记该 Stream 的 Syntax Node。通过调整不同 Token 长度的合法 Sentence，确定生成器可精确抵达边界和首次超限。

因此三类资源都有普通 Source/CLI 的独立边界与首次超限路径，不被 Source Byte、Edge、Stream、Token 或下游候选上限必然先行支配。

### 5.2 四类 `DEFENSIVE_DOMINATED`

支配关系由计数对象和登记顺序直接导出：

```text
parse_candidate_syntax_nodes <= candidate_token_occurrences
bound_frame_candidates        <= candidate_token_occurrences
typed_frame_candidates        <= bound_frame_candidates
canonical_frame_candidates    <= typed_frame_candidates
```

- 每个成功 Parse Candidate 只能来自一条非空完整 Stream，而该 Stream 的全部 Token Occurrence 已在 Parser 前登记；固定 Syntax Node Taxonomy 逐个 Production 均满足 Node 数不大于 Stream Token 数。
- 一个 Bound Candidate 最多来自一个成功 Parse Candidate；Typed 与 Canonical 每级又最多对直接上游完整 Candidate 计一次。
- 五个 Document 上限同为 `1,000,000`，且 Token 登记先于 Parse/Bind/Type/Normalize，因而普通 Pipeline 必须先以 `candidate_token_occurrences` T0007 终止，不得再产生下游资源诊断或 Artifact。
- 生产登记函数仍为四类资源保留完整的 T0007/Stage/Span/Payload 检查；当 Grammar 或上游不变量将来改变时，防御性检查仍然失败关闭。

### 5.3 私有 Ledger 不形成测试后门

CNL-C012B 仅允许 `#[cfg(test)]` 私有 Module 设置 Ledger 初值，然后调用与生产相同的登记函数。合同同时禁止 CLI 参数、环境变量、Feature、配置文件、公共 API 和 Release Binary 路径，也禁止绕过公开 Pipeline 先行上限。该辅助器不改变任何生产 Source 的接受集，不建立第二公共接口；实现时还必须由 CNL-C012C 验证生产支配关系。

## 6. CNL-C012A/B/C 可执行性

- CNL-C012A 是三类真正公开行为的 CLI 黑盒合同；第 5.1 节给出构造性可达证明。
- CNL-C012B 是四类生产登记函数的私有 Module 合同；边界、超一、Payload/Stage/Span 和下游失败关闭都有唯一预期。Syntax Node 序列中第二个 `TypedValue` Span 仍是唯一预期。
- CNL-C012C 是生产支配证据；它不要求继续执行到受支配资源，而是要求直接验证不变量和先行 Token T0007 终止。

三项不再宣称每个资源都可以公共 CLI 独立触发，因而 NF-05 的不可达验收矛盾已消失。

## 7. NO-04 与越权边界

- 整改记录 BF-07 行已使用“IR Header 字段 + Binary 绑定断言”，不再保留“Manifest 字段”措辞。其他 `Source Manifest` / `Cargo Manifest` 是正常专有名称，不是 NO-04 残留。
- `07_src` 排除 `target/` 的普通文件摘要为 `5f80929f91c106ff7c0f43ce57633c78eca890e89d6eb6272e4dede7e9c20230`，与 Target 证据一致。
- 四个受保护 Cargo 文件 SHA-256 逐项与 Target 第 5 节一致；Cargo/Lock 仍无 `unicode-normalization`。
- 仓库仍无 Commit，`git remote -v` 为空。本 Reviewer 未修改任何 Target、Compiler Source、实现测试、Cargo、治理或远程状态。

## 8. 机械核验

- 40 文件 Target：开始、报告写入前与结束三次复算均与冻结摘要一致。
- 本轮未运行 Compiler 测试，未将 Conformance 设计冒充已通过的实现证据。
- Schema 未属本轮改动，按 `DELTA_ONLY` 裁决不重复执行全量 Schema 复审；其精确 Byte 内容由匹配的 Target Digest 保护。
- 未安装、新增或升级任何依赖。

## 9. 结论与权限边界

结论：`READY_FOR_HUMAN_DETERMINATION`。

NF-05 已在不改变任何资源上限、生产失败语义或 Source 接受集的前提下关闭；BF-06/BF-10/NF-03 的剩余证据缺口随之关闭。本报告只建议进入项目负责人的精确裁决，不批准公共契约、不恢复 TASK-018、不修改依赖、不提交、不推送、不建立 Baseline、不发起正式 C04、Release 或 Formal Seal。
