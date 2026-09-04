# 需求追溯矩阵

> 本文件同时管理“正式上游需求关系”和“向架构/代码/测试的下游追溯”。
>
> **Node Coverage（节点覆盖）与 Edge Consistency（关系边一致性）必须分开验证。**
> “所有 ID 都出现过”不能证明关系边已经闭合。

---

## 1. 正式关系语义

默认正式上游关系类型只有：

```text
FORMAL_TRACE
```

默认关系方向：

```text
PRD / CON / AC（以及项目明确批准的其他产品层来源）
→
SYS / NFR / IF
```

如项目确需 `SUPPORTS / ALLOCATES_TO / CONSTRAINS` 等不同关系类型，必须先正式定义每种语义、规范来源和机械校验规则，不得为了消除差集临时发明关系类型。

---

## 2. 正式上游追溯关系矩阵

> 本节与 SYS / NFR / IF 详细元数据中的 `Traces-From` 应表达同一组 `FORMAL_TRACE` 关系。
> 一条关系就是一条边：`Source → Target`。

| Source | Target | Relation Type | Status |
|---|---|---|---|
| <SOURCE_ID> | <TARGET_ID> | FORMAL_TRACE | DRAFT |

---

## 3. 下游实现与验证追溯

| 系统需求 | 架构/设计 | 代码模块 | 测试 | 状态 |
|---|---|---|---|---|
| <TARGET_ID> | <ARCH-/DES-> | <Module> | <TC-> | MISSING |

状态：`MAPPED / PARTIAL / MISSING / UNKNOWN`

---

## 4. Traceability Mechanical Verification

在建立或复审正式需求 Baseline 前执行：

```bash
python3 09_quality/traceability/validate_traceability.py
```

### 4.1 Node Coverage

```text
Expected IDs: NOT_RUN
Covered IDs: NOT_RUN
Missing IDs: NOT_RUN
Unexpected IDs: NOT_RUN

NODE RESULT: NOT_RUN
```

### 4.2 Edge Consistency

```text
Detailed Metadata Edges: NOT_RUN
Traceability Matrix Edges: NOT_RUN
Intersection: NOT_RUN
Detailed-only: NOT_RUN
Matrix-only: NOT_RUN

EDGE RESULT: NOT_RUN
```

### 4.3 Overall Result

只有 Node Coverage 与 Edge Consistency 均通过，或所有非零例外均已有正式批准且逐条标注时，才允许写：

```text
TRACEABILITY_CLOSED
```

否则必须保持：

```text
TRACEABILITY_NOT_CLOSED
```

---

## 5. 差异边处理规则

发现 `Detailed-only` 或 `Matrix-only` 时，每条差异必须分类：

```text
VALID_FORMAL_TRACE
→ 关系有效，两侧同步。

INVALID_RELATION
→ 关系无效，从错误一侧删除。

DIFFERENT_RELATION_SEMANTICS
→ 不是同一种关系；必须先正式定义并批准关系类型。
```

禁止把任意一侧直接批量覆盖到另一侧。
