# TASK-019 实施前契约整改记录

- Record ID：`REC-APLS-TASK019-001`
- 状态：`THIRD_REMEDIATION_CANDIDATE_READY_FOR_EXACT_REREVIEW`
- 日期：`2026-09-03`
- Authority：`HDP-APLS-017 Option A` / `DEC-021`
- 输入复审：`IIR-APLS-TASK018-PREIMPL-001`、`IIR-APLS-TASK019-REREVIEW-001`、`IIR-APLS-TASK019-REREVIEW-002`

> 本记录证明整改候选覆盖范围，不自我宣告 Finding 已由独立复审关闭。只有全新 Reviewer 对精确 Target 的结论可形成再复审证据。

## 1. Finding 对应输出

| Finding | 整改候选 | 预期关闭证据 |
|---|---|---|
| BF-02 | `APLS_0.1_CNL_TO_IR_MAPPING.md` 第 6 节 | `BLOCKED_BY_SCHEMA_GAPS` 明确标为 DEC-019 前历史 Gate；当前 Gate 单列 |
| BF-03 | `APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md` 第 6 节 | 非 Grammar 示例改为 `灌溉水泵处于“故障”状态`；Grammar 未扩张 |
| BF-04 | `DES-APLS-CNL-SEMVAL-001` 第 2～4 节 | 六种 Property 类型、单位、六个运算符、Literal 与 Canonical Value 封闭矩阵及正反例 |
| BF-05 | `DES-APLS-CNL-SEMVAL-001` 第 6 节 | Source State 隐式前置条件、结构可达算法、确定同 Trigger 冲突、E1402/E1403 与一般 SAT 非目标 |
| BF-06 | `DES-APLS-CNL-RESOURCE-001` | Edge Sentinel/Identity、全句 Stream Token 计数、固定 Syntax Node Taxonomy、声明句和稳定顺序 |
| BF-07 | `DES-APLS-CNL-UNICODE-001` 与 CNL IR Header Schema | Unicode 17.0.0、UAX #15 rev57、完整 is_nfc、IR Header 字段 + Binary 绑定断言、精确依赖候选和 Conformance 来源 |
| BF-08 | `DES-APLS-CNL-SEMVAL-001` 第 5 节 | 合取排序去重；一项降 Atomic；两项以上才 Conjunction；Provenance 并集 |
| BF-09 | `DES-APLS-CNL-SEMVAL-001` 第 7～8 节 | Built-in Materialization Closure、Source Map 豁免、Role Enum、UTF-8 Byte Span/Coverage Cross-validation |
| BF-10 | CNL Diagnostics、`apls-cnl-diagnostic-0.1.schema.json`、Resource Profile | 零候选聚合/抑制、E1310 恰好两个指纹 Witness、T0007 封闭 Payload 与诊断上限 |

第一次再复审判定 BF-02/03/04/05/08/09 为 `CLOSED`，BF-06/07/10 为 `PARTIAL`，并提出 NF-01～NF-04。本记录不把第二轮修正自我宣告为关闭；下列是再次独立复审的待验证项：

| 复审缺口 | 第二轮整改候选 | 预期关闭证据 |
|---|---|---|
| NF-01 / BF-06 | Language/Grammar/Frontend/Resource 的 Declaration Bootstrap | 无类别 `DECLARED_TERM`、同一正式 Grammar Entry、全部 Provisional Header 后绑定、前向引用、无未计数预扫描、Pass 资源顺序闭合 |
| NF-02 / BF-10 | CNL Diagnostics、Compiler MVP、通用 Diagnostics、Foundation、Resource | 唯一 Byte-based 总排序键，含 Null Group、End Byte、Related Canonical Bytes 与 Full Diagnostic Tie-breaker；第 1001 条在总排序后截断 |
| NF-03 / BF-06/BF-10 | Resource 第 5.1 节与 CNL-C012 | 每个 Syntax Node 的唯一原始 Byte Span；确定 Generator 使第二个 TypedValue 成为第 1,000,001 个 Node |
| NF-04 / BF-07 | Language/Unicode/CNL-C009 | 明确不设独立 Compiler Manifest、不改变 `--version`；IR Header + Binary 常量/构建启动断言是唯一契约 |

第二次再复审判定 BF-02/03/04/05/07/08/09 与 NF-01/02/04 为 `CLOSED`，BF-06/10 与 NF-03 为 `PARTIAL`，新增 NF-05。第三轮只修复资源证据可达性，不改变任何资源数值：

| 复审缺口 | 第三轮整改候选 | 预期关闭证据 |
|---|---|---|
| NF-05 / BF-06/BF-10/NF-03 | Resource/Work Package/Foundation/CNL-C012 | 前三类资源标记 `PUBLIC_SOURCE_REACHABLE` 并做 CLI 边界；后四类标记 `DEFENSIVE_DOMINATED`，用 `#[cfg(test)]` 私有 Ledger 注入验证生产登记函数；普通 Source 验证支配关系和先行 Token T0007 |
| NO-04 | 本记录 BF-07 行 | 删除“Manifest 字段”残留，改为“IR Header 字段 + Binary 绑定断言” |

## 2. Schema 候选变化

### CNL IR

- Header 新增必需字段 `unicode_data_version=17.0.0` 与 `unicode_normalization=NFC`；
- `roleSpan.role` 从开放 Pattern 收紧为封闭 Enum；
- 单项 Conjunction 继续由既有 `minItems=2` 拒绝，语义层负责去重后降 Atomic；
- Unit Closure、Source Map 豁免与 Span 边界由 Cross-validator 执行，不伪称 JSON Schema 能验证全部跨节点语义。

### Diagnostic Envelope

- 新建 `apls-cnl-diagnostic-0.1.schema.json`，保持既有四个顶层字段；
- 所有 Diagnostic 公共字段变为封闭结构；
- E1310 与 T0007 通过 Code 条件绑定专用 Payload；
- Envelope Status 与 Internal/Tool/Source 类别建立 Schema 约束；稳定排序和 Span 关系由 Cross-validator 执行。

## 3. 依赖候选变化

当前项目文件没有依赖变化。建议由下一 Human Determination 批准：

```text
direct: unicode-normalization =0.1.25, default-features=false
Unicode tables: 17.0.0
crate sha256: 5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8
transitive lock target: tinyvec 1.6.0 with alloc
transitive lock target: tinyvec_macros 0.1.1
checksums: tinyvec=87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50
           tinyvec_macros=1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20
```

批准后必须在真正修改 Cargo Lock 时再次审查精确 checksum、Feature、License 和 Rust 1.86 构建结果。若不通过，不能缩小 NFC 检查替代。

## 4. Conformance 候选

`TESTDES-APLS-CNL-PREIMPL-001` 以 16 个高价值 Case 覆盖原九项 Finding 和两轮再复审缺口；CNL-C012 拆为公开可达 CLI 资源、私有防御性 Ledger 和生产支配关系三个子项，CNL-C015/C016 分别覆盖 Declaration Bootstrap 与诊断总排序。没有实施测试、压力测试、Fuzz 或无依据全排列。Matrix 的每一行由正/反代表覆盖，Unicode 算法另绑定官方 17.0.0 Normalization Test。

## 5. 当前边界

- 这是 `TASK-019` 输出候选，不是当前公共契约的自动采用；
- `TASK-018` 继续 `BLOCKED`；
- 没有修改 Compiler Source、实现测试、Cargo Manifest 或 Cargo Lock；
- 没有 Commit、Push、Baseline、Formal C04、Release 或 Formal Seal；
- 第一、第二次再复审均为 `CHANGES_RECOMMENDED`；第三轮整改完成机械自检后，必须形成新的精确 Target 并由另一个全新 `INFORMAL_INDEPENDENT` Reviewer 只读再复审。
