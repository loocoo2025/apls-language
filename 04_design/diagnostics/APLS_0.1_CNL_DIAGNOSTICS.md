# APLS 0.1 CNL 诊断目录

- 设计 ID：`DES-APLS-CNL-DIAG-001`
- 状态：`TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018`
- 日期：`2026-09-03`
- 输入：`DEC-004`、`DEC-014`、`DEC-015`、`DEC-017`、`DEC-021`、`DES-APLS-ZH-CNL-001`、`DES-APLS-CNL-FRAME-001`、`DES-APLS-CNL-SEMVAL-001`
- 适用范围：`apls-zh-CN-0.1` Source 到 Typed Semantic Frame
- 机器 Schema：`apls-cnl-diagnostic-0.1.schema.json`

> 本目录扩展既有 Diagnostic Envelope。错误消息可以改善，但 Code 的根因身份在 0.1 内不得漂移。

## 1. 诊断不变量

每条 CNL 诊断必须提供：

```text
code
severity = error
message
primary_source_span
related_source_spans
normative_rule_reference
missing_or_ambiguous_roles
candidate_symbols
optional_fix_suggestions
```

- 本目录中的所有公共诊断项都是 Error；公共 Error 存在时不输出 Canonical Frame 或 Verified IR；
- 单个分析候选被确定性规则淘汰时只形成内部 Candidate Rejection Evidence。若其他候选最终收敛为唯一 Canonical Frame，不得把已淘汰路径的局部失败提升为公共 Error；
- 只有全部候选失效时，才从候选失败证据中按稳定根因规则形成公共 Source Error；
- 修复建议不得自动应用；
- AI 可以解释这些字段或草拟新 Source，但不能改写当前编译事务；
- 同一根因在同一最小 Span 只发一条主诊断；
- 候选列表按 Symbol ID 的 UTF-8 Byte 升序稳定排列；
- 诊断集使用第 1.2 节唯一公共总排序键；该候选键替代旧 MVP 的 Line/Column 简写，并等待 `HDP-APLS-018` 一并采用。

机器输出必须通过 `apls-cnl-diagnostic-0.1.schema.json`，并额外通过本文的排序、聚合和 Span Cross-validation。Schema 或 Cross-validation 任一失败都不能发布诊断 Envelope。

### 1.1 零有效候选的根因聚合

当且仅当一个 Sentence 的 Canonical Frame 等价类数量为 `0` 时：

1. 每条完整候选路径只贡献其首个失败 Stage 的全部直接 Terminal Finding；Stage 固定为 `lexical < grammar < binding < type_unit < semantic_role`，失败后不运行下游 Stage；
2. 没有 Complete Token Stream 时，使用 Lattice/Sentence Validator 的直接 Finding；有 Stream 但无 Parse 时，使用各 Stream 的 Grammar Terminal Finding；
3. 对全部 Terminal Finding 取并集，按 `(code,primary_span,related_spans,missing_or_ambiguous_roles,candidate_symbols,payload)` 的 Canonical JSON Byte 去重；不得按发现顺序、候选评分或“第一个错误”选择；
4. 只应用以下封闭抑制关系，且必须是同一 Primary Span：具体 `E1103/E1104/E1105/E1106/E1201..E1403` 抑制派生 `E1101`；`E1201/E1204/E1206/E1207` 抑制 `E1203`；`E1401` 抑制 `E1308`；不存在其他隐式优先级；
5. 剩余诊断按第 1.2 节唯一公共总排序键升序输出；
6. 若有一个有效 Canonical Frame 等价类，候选局部失败证据全部保持内部；若有两个及以上有效类，该 Sentence 只输出 `APLS-E1310`，不混入被淘汰候选的局部错误。

公共诊断超过 `1,000` 条时，按 `DES-APLS-CNL-RESOURCE-001` 的固定规则形成 `APLS-T0007`，不得在稳定排序前按遍历顺序截断。

每条 Diagnostic 内部集合也必须稳定：`related_source_spans` 按 `(logical_path,start_byte,end_byte)`；`missing_or_ambiguous_roles` 按 UTF-8 Byte；`candidate_symbols` 按 `(symbol_id,symbol_kind,display_name)`；`fix_suggestions` 按 `(kind,target_span,message)`。所有数组先去重后排序。JSON Object Member 顺序不具有语义；跨实现比对先解析并按 `APLS-CNL-C14N-0.1` 规范化。

### 1.2 唯一公共诊断总排序键

所有公共 Source/Tool/Internal Diagnostic 在去重并规范化内部数组后，按下列 Tuple 从左到右升序：

```text
(
  primary_span_group,
  logical_path_or_empty,
  start_byte_or_zero,
  end_byte_or_zero,
  code,
  related_source_spans_canonical_bytes,
  full_diagnostic_canonical_bytes
)
```

- 有 `primary_source_span` 时 `primary_span_group=0`，使用其逻辑路径和 Byte Offset；无 Span 时 `primary_span_group=1`，路径固定 `""`、起止固定 `0`，因此所有无 Span Diagnostic 排在有 Span之后；
- String 按 UTF-8 Byte，Integer 按数值升序；`related_source_spans_canonical_bytes` 是按第 1.1 节排序后的完整数组经 `APLS-CNL-C14N-0.1` 序列化所得 Byte；
- `full_diagnostic_canonical_bytes` 是完整 Diagnostic Object 在内部数组已排序后经同一 Canonicalization 得到的 Byte，作为最终稳定 Tie-breaker；完全相同的 Object 已在排序前去重；
- Line/Column 是由同一 Byte Span 推导并 Cross-validation 的展示字段，不参与排序，也不得与 Byte Offset 冲突；
- 公共诊断第 `1,001` 条的保留集合只能在该总排序完成后确定。

这是 Compiler 0.1 的唯一公共诊断排序合同。`APLS_0.1_COMPILER_MVP_DESIGN.md` 与通用诊断目录中的旧简写同步由本 Tuple 取代；实现不得在 CNL 与非 CNL 公共 Envelope 中选择不同排序键。

## 2. 编码与句界

| Code | 根因 | 最小主要范围 | 修复方向 |
|---|---|---|---|
| `APLS-E1001` | 非 UTF-8、BOM、禁止控制字符或非字符码位 | 非法 Byte/Scalar | 保存为无 BOM UTF-8 |
| `APLS-E1004` | Source 不是 Profile 要求的 NFC | 首个不符合序列 | 显式转换后重新提交 |
| `APLS-E1005` | 使用 Tab、CR 单独出现或禁止 Unicode 空白 | 对应空白 | 改为普通空格、LF 或 CRLF |
| `APLS-E1006` | 规范标点错误或句子没有以 `。` 结束 | 错误标点或文件末尾 | 使用冻结的中文标点 |
| `APLS-E1007` | 术语/说明引号未配对、嵌套或跨行 | 开引号至错误点 | 闭合并移除嵌套 |
| `APLS-E1008` | 术语名字符集或 1～64 长度不合法 | 完整术语名 | 使用允许字符和长度 |

旧 DSL 的 `APLS-E1002/E1003` 根因身份保留，不复用于新含义。

## 3. Profile 与 Grammar

| Code | 根因 | 最小主要范围 | 修复方向 |
|---|---|---|---|
| `APLS-E1101` | Token 序列不符合当前 CNL Grammar | 首个无法继续的位置 | 使用受支持句式 |
| `APLS-E1102` | 旧 DSL Grammar 的多个合法解析或 Parser 冲突 | 最小分歧文本 | 旧兼容路径修正规则；CNL 不因中间候选多解发出此 Code |
| `APLS-E1103` | 句式缺少必需结构或 Semantic Role | 缺失点或不完整短语 | 补充主体、对象、值或时限 |
| `APLS-E1104` | 首句缺失或 Profile 声明不精确 | 首句 | 使用固定 Profile 声明 |
| `APLS-E1105` | 使用英文/混合语法或未登记自然语言句式 | 整句最小不匹配段 | 改为 `apls-zh-CN-0.1` 句式 |
| `APLS-E1106` | 一个句子形成多个主 Frame 或并列行为 | 并列连接范围 | 拆为多个规范句 |

## 4. 术语与引用

| Code | 根因 | 最小主要范围 | 修复方向 |
|---|---|---|---|
| `APLS-E1201` | 术语或引用没有声明候选 | 引用文本 | 先声明或修正名称 |
| `APLS-E1202` | 声明名或别名重复 | 后一声明名 | 使用唯一名称；首声明为 Related |
| `APLS-E1203` | 引用候选无法由 CNL Profile 支持的绑定规则继续分析 | 引用文本 | 声明、修正或使用带引号完整名称；不得仅因中间候选多于一个发出 |
| `APLS-E1204` | 引用类别与 Grammar 角色不匹配 | 引用文本 | 使用正确类别术语 |
| `APLS-E1205` | 使用未声明同义词、缩写、繁简转换或近似名称 | 引用文本 | 显式声明别名或使用原名 |
| `APLS-E1206` | 动作与目标不匹配动作声明 | 动作短语 | 使用已声明的 Action/Target 配对 |
| `APLS-E1207` | 状态不属于指定实体或初始状态不在状态集 | 状态引用 | 使用该实体已声明状态 |

## 5. 自然语言歧义

| Code | 根因 | 最小主要范围 | 修复方向 |
|---|---|---|---|
| `APLS-E1301` | 模糊阈值、程度或变化量，如“高”“适当”“一点” | 模糊词 | 给出比较词、精确值和单位 |
| `APLS-E1302` | 使用“它”“该设备”“前者”等未冻结指代 | 指代词 | 重述精确术语 |
| `APLS-E1303` | 主体、动作对象或属性所属缺失 | 缺失点/短语 | 显式补全角色 |
| `APLS-E1304` | 模态不在 `必须/不得/禁止` 内 | 模态短语 | 选择一个已定义模态 |
| `APLS-E1305` | 使用首版不支持的否定或并列作用域结构 | 否定/连接范围 | 拆句或改用支持结构 |
| `APLS-E1306` | 使用 `或者`、嵌套条件或一般自然语言否定 | 对应连接词/范围 | 改为独立规则或受支持合取 |
| `APLS-E1307` | 时间参考、时限或先后关系不确定 | 时间短语 | 给出事件与精确时长 |
| `APLS-E1308` | 数值缺少唯一类型或必需单位 | 数值 | 补充单位或属性类型声明 |
| `APLS-E1309` | 句子依赖跨句省略、上下文或领域常识 | 依赖范围 | 在本句重述完整事实 |
| `APLS-E1310` | 全部有效候选规范化后仍有两个或以上不等价 Canonical Frame | 最小最终语义分歧范围 | 补充能排除分歧的精确信息；可使用引号或拆句 |
| `APLS-E1401` | Property 类型、声明单位、比较运算符或右值 Literal 不满足封闭兼容矩阵 | 冲突的单位、运算符或右值 | 按 Property 类型使用唯一允许的值与单位 |
| `APLS-E1402` | State 在显式 Transition 拓扑中不能从初始状态到达 | 不可达 State 的声明名 | 增加合法进入路径或移除不可达状态 |
| `APLS-E1403` | 相同实体、源状态与规范 Trigger 要求进入多个不同目标状态 | 稳定排序后的第一条冲突 Transition | 使 Trigger 或目标状态互不冲突 |

`APLS-E1310` 的机器 Payload 固定为 Schema 中的 `ambiguity` 类型：`outcome=AMBIGUOUS`、`differing_roles` 和恰好两个 Witness。所有 Canonical Frame 等价类先按不含 Provenance 的 Canonical Semantic Payload Byte 升序，只公开前两个类的见证；这只是确定性截断，不是语义选择。

每个 Witness 只公开 `frame_kind`、`semantic_fingerprint` 和各分歧 Role 的 `role_fingerprints`，不公开 AST、IR 或完整内部 Frame。Fingerprint Preimage 分别为：

```text
UTF8("APLS-CNL-FRAME-WITNESS-0.1") + 0x00 + canonical_frame_payload_bytes
UTF8("APLS-CNL-ROLE-WITNESS-0.1") + 0x00 + UTF8(role) + 0x00 + canonical_role_value_bytes
```

使用 SHA-256 并加 `sha256:` 前缀。`differing_roles` 是两个见证之间值不同或一侧缺失的规范 Role 名 UTF-8 Byte 升序集合；每个 Witness 的 `role_fingerprints` 按 Role 名排序。Primary Span 是所有 Differing Role Provenance Span 的最小包含区间，且不能扩张到相邻句。

中间 Tokenization、Parse Tree 或候选 Frame 数量大于 1 不是 E1310 的充分条件。零个有效 Canonical Frame 必须由具体词法、Grammar、绑定、类型、单位或角色错误解释，不得标记为 `AMBIGUOUS`。

若实现因固定资源上限无法完整枚举或验证候选，必须使用 `APLS-T0007` Tool Failure；“未能证明唯一”不能伪装成“已证明存在多个含义”。T0007 的 `resource/limit/observed/scope/sentence_index/stage`、Primary Span 和首次超限规则只由 `DES-APLS-CNL-RESOURCE-001` 定义。

## 6. 诊断示例

输入：

```text
温度高的时候适当降低一点速度。
```

最少产生一个根因诊断组：

```text
APLS-E1301
无法形成唯一规范解释。

未确定内容：
- “高”没有阈值和单位；
- “适当”和“一点”没有可计算含义。

APLS-E1303
- “温度”和“速度”没有唯一所属对象；
- 没有确定的动作目标。

本次编译已拒绝，未选择候选含义，也未产生 Semantic Frame。
```

若一个最小范围同时触发通用 Grammar Error 和更具体 CNL Error，优先发具体 CNL Error，并抑制同范围的派生 `APLS-E1101`。

## 7. 结构化修复建议

建议可以包含：

- `declare_term`：建议声明缺失术语；
- `quote_exact_term`：建议使用 `“精确名称”`；
- `supply_value_and_unit`：建议补充精确量；
- `supply_semantic_role`：建议补充主体、对象或所属；
- `split_sentence`：建议拆分并列 Frame；
- `choose_candidate`：列出候选供用户选择，但不得设置默认候选。

修复建议只描述可选动作，不得携带 `auto_apply=true`，也不得在编译器内部触发新语义选择。
