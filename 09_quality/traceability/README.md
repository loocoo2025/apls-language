# 追溯关系机械门禁

本目录提供正式需求追溯的机械校验。

运行：

```bash
python3 09_quality/traceability/validate_traceability.py
```

校验内容：

1. **Node Coverage**：当前 SYS / NFR / IF 是否都出现在正式上游关系矩阵中；
2. **Edge Consistency**：详细 SYS / NFR / IF 的 `Traces-From` 关系集合，是否与 `requirements_traceability.md` 的正式关系集合完全一致；
3. 重复关系；
4. 未知来源 ID。

默认通过条件：

```text
Missing IDs = 0
Unexpected IDs = 0
Detailed-only = 0
Matrix-only = 0
Duplicate Metadata Edges = 0
Duplicate Matrix Edges = 0
Invalid Source IDs = 0
```

脚本退出码：

- `0`：PASS
- `1`：FAIL

## 元数据格式

SYS / IF 推荐：

```text
- Traces-From（正式上游追溯）：
  - PRD-XXX
  - AC-XXX
```

NFR 表格使用 `Traces-From（正式上游追溯）` 列，多项可用逗号或空格分隔。

`requirements_traceability.md` 的正式关系矩阵建议一条边一行。

> 该脚本只负责机械一致性，不替代需求语义评审。差异边必须由 C01/C04 逐条判断是否真实有效。

## 模板初始状态

空白模板中的示例 SYS/NFR/IF 尚未建立正式关系，因此在需求尚未整理完成时运行可能返回 FAIL。这不是项目启动阻断；该门在“准备建立或复审正式需求 Baseline”时才必须通过。
