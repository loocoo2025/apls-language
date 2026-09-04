# APLS 0.1 CNL Unicode NFC 实现 Profile

- 设计 ID：`DES-APLS-CNL-UNICODE-001`
- 状态：`TASK019_REMEDIATION_CANDIDATE_AWAITING_HDP_APLS_018`
- 日期：`2026-09-03`
- 输入：`DEC-015`、`DEC-020`、`DEC-021`、`DES-APLS-ZH-CNL-001`
- 关闭 Finding：`BF-07`

> 本文冻结完整 NFC 的数据版本、算法来源、唯一机器声明落点和依赖候选；当前没有修改或安装依赖，也没有修改 `Cargo.toml` / `Cargo.lock`。

## 1. 规范选择

APLS Compiler 0.1 的 CNL Source NFC 检查候选固定为：

```text
normalization form: NFC
Unicode data version: 17.0.0
normative algorithm: Unicode Standard 17.0.0, Section 3.11
normative annex: UAX #15 Revision 57
```

Compiler 必须对解码后的完整 Source String 执行权威 NFC 判定；不得只检查术语、汉字范围或 Quick_Check=No。Quick Check 返回 Maybe 时必须完成规范判定。Source 不在 NFC 时使用 `APLS-E1004` 拒绝，且不得静默转换后继续编译。

Unicode 17.0.0 UAX #15 要求声称执行 Normalization 的实现满足相应 `NormalizationTest.txt` Conformance 数据。后续实现验证必须覆盖官方 Unicode 17.0.0 文件，而不是自建少量样例替代完整算法证据。

## 2. Rust 依赖候选

推荐新增一个精确直接依赖：

```toml
unicode-normalization = { version = "=0.1.25", default-features = false }
```

候选理由与审查事实：

| 项目 | 候选事实 |
|---|---|
| Crate | `unicode-normalization 0.1.25` |
| API | `unicode_normalization::is_nfc(&str) -> bool`，权威检查完整 NFC |
| 内嵌数据 | `UNICODE_VERSION=(17,0,0)`，已从 crates.io 官方发布包 `src/tables.rs` 交叉确认 |
| Crate SHA-256 | `5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8` |
| Crate MSRV 声明 | Rust `1.36`，低于项目 MSRV `1.86` |
| Crate License | `MIT OR Apache-2.0`，与项目 Apache-2.0 可兼容选用 |
| Runtime Dependency | `tinyvec 1.6.0`；实际激活 `alloc + default(empty) + tinyvec_macros`，不启用 `std` |
| Unsafe 边界 | 上游 crate 与 tinyvec 均声明纯 Safe Rust；项目仍以实际锁定 Source Review 为准 |

为避免 `tinyvec = "1"` 随时间改变 Lock 结果，首次获准修改依赖时必须把解析结果精确锁定并审查。候选传递闭包为：

| Package | Feature | SHA-256 / Registry Checksum | License/MSRV 依据 |
|---|---|---|---|
| `tinyvec 1.6.0` | `alloc + default(empty) + tinyvec_macros` | `87cc5ceb3875bb20c2890005a4e226a4651264a5c75edb2421b52861a0a0cb50` | `Zlib OR Apache-2.0 OR MIT`；Feature 注释只要求 Rust 1.40，低于项目 1.86 |
| `tinyvec_macros 0.1.1` | none | `1f3ccbac311fea05f86f61904b462b55fb3df8837a366dfc601a0161d0532f20` | `MIT OR Apache-2.0 OR Zlib`、Edition 2018、无运行依赖；由 `tinyvec/alloc` 唯一引入 |

上游 `unicode-normalization` README 明确把 `tinyvec <=1.6.0` 作为 Rust 1.36 兼容边界示例。最终 Cargo Lock、Feature 闭包和 MSRV 1.86 构建证据必须进入同一次依赖变更审查；本文件不伪称该 Lock 已生成。

`HDP-APLS-019 Option A` / `DEC-024` 已接受锁定版本的上述实际闭包：`unicode-normalization/default-features=false`；`tinyvec/default=[]` 为空 Feature，`tinyvec/alloc` 引入 `tinyvec_macros`；`unicode-normalization/std` 与 `tinyvec/std` 均未启用。此更正不改变版本、Checksum、Lock 或运行能力。

如果依赖解析、许可或 MSRV 实测与本候选不一致，必须保持 `TASK-018 BLOCKED` 并返回 C02/Human，不得回退为部分 NFC 检查。

## 3. 唯一机器声明落点与 Binary 绑定

`apls-cnl-ir-0.1.header` 新增并要求：

```json
{
  "unicode_data_version": "17.0.0",
  "unicode_normalization": "NFC"
}
```

这两个字段描述 Source 接受判断使用的规范数据，不参与文档领域语义 Hash；它们参与 IR Schema 验证。它们是 APLS 0.1 唯一公开的 Unicode 机器声明字段。

APLS 0.1 不定义独立 `Compiler Manifest` Artifact，也不增加隐藏文件、JSON Endpoint 或 CLI 字段。既有版本输出保持：

```text
apls <compiler-version> language=0.1 ir=0.1
```

`apls --version` 不另行输出 Unicode 版本。Compiler Binary 的 `compiler_version`、编译期常量 `unicode_data_version=17.0.0` / `unicode_normalization=NFC`、CNL IR Header Writer 和依赖公开常量必须绑定同一 Profile；同一 Compiler Version 不得替换 Unicode 数据表或 Normalization 算法行为。对不产生 IR 的 `parse/check/diagnose`，Binary 绑定由构建证据和启动断言证明，不虚构另一个公开 Manifest。

启动或构建期必须断言依赖公开的 `UNICODE_VERSION == (17,0,0)`；不一致属于 Internal Failure，不得继续接受 Source。

## 4. 检查顺序与 Span

1. 先验证原始 Byte 是无 BOM 的合法 UTF-8；
2. 对完整解码 String 调用权威 `is_nfc`；
3. 若失败，为了给出 `APLS-E1004` 最小 Span，用同一版本算法从左到右寻找第一个最短前缀扩展，使其规范结果与原 Byte 不同；Primary Span 覆盖产生首个差异的完整 Canonical Combining Sequence；
4. Span 必须位于原始 Byte 的 Scalar 边界，不能指向静默规范化后的临时 String；
5. NFC 通过后才执行结构空白、标点、句界与术语处理。

实现可以用 Quick Check 加速，但最终 Boolean 和 E1004 Span 必须与完整 NFC 比较算法等价。资源不足是 Tool Failure，不能把未完成判断当作非 NFC 或合法。

## 5. 最小算法证据

除 Unicode 17.0.0 `NormalizationTest.txt` 外，项目级最小黑盒证据必须覆盖：

- 已组合字符与 Base+Combining Mark（例如 `é` 与 `e + U+0301`）；
- Canonical Combining Class 重排；
- Hangul 预组合音节与 Jamo 序列；
- 非 BMP Scalar；
- Quick_Check Maybe 但最终已规范/未规范的两类边界；
- `APLS-E1004` 的原始 Byte Span 稳定性。

这些证据验证依赖接线和 APLS Span，不重复实现 Unicode 全量测试逻辑。

## 6. 官方来源

- Unicode 17.0.0 UAX #15：<https://www.unicode.org/reports/tr15/tr15-57.html>
- Unicode 17.0.0 Core Spec §3.11：<https://www.unicode.org/versions/Unicode17.0.0/core-spec/chapter-3/>
- Unicode 17.0.0 Conformance Data：<https://www.unicode.org/Public/17.0.0/ucd/NormalizationTest.txt>
- `unicode-normalization 0.1.25` API：<https://docs.rs/unicode-normalization/0.1.25/unicode_normalization/fn.is_nfc.html>
- crates.io 发布包：<https://static.crates.io/crates/unicode-normalization/unicode-normalization-0.1.25.crate>
- 上游 Source/Manifest：<https://docs.rs/crate/unicode-normalization/0.1.25/source/>

## 7. 批准边界

采用 Unicode 17.0.0 与新增 `unicode-normalization` 会改变公开 Source 接受版本和依赖闭包，必须由 `HDP-APLS-018` 精确批准。批准前不得修改 Compiler Source、Cargo Manifest、Cargo Lock 或依赖缓存；批准后仍必须生成并审查实际 Lock Diff，完成 MSRV 与许可证据后才能恢复其余实现。
