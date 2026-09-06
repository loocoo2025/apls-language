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
>
> 本矩阵依据 `DEC-031`（`HDP-APLS-024` Q4=A）建立：AC-001～009 提炼自 PRD-001～009 既有验收段，SYS-001～009 一对一派生自 PRD-001～009，NFR-001～006 派生自 PRD §6（Q4-a=否：PRD §6 无独立编号，NFR 行使用最直接相关 P0 需求 ID 并注明源自 §6），IF-001～003 派生自 PRD-006～008 公开契约。

| Source | Target | Relation Type | Status |
|---|---|---|---|
| PRD-001 | SYS-001 | FORMAL_TRACE | CONFIRMED |
| AC-001 | SYS-001 | FORMAL_TRACE | CONFIRMED |
| PRD-002 | SYS-002 | FORMAL_TRACE | CONFIRMED |
| AC-002 | SYS-002 | FORMAL_TRACE | CONFIRMED |
| PRD-003 | SYS-003 | FORMAL_TRACE | CONFIRMED |
| AC-003 | SYS-003 | FORMAL_TRACE | CONFIRMED |
| PRD-004 | SYS-004 | FORMAL_TRACE | CONFIRMED |
| AC-004 | SYS-004 | FORMAL_TRACE | CONFIRMED |
| PRD-005 | SYS-005 | FORMAL_TRACE | CONFIRMED |
| AC-005 | SYS-005 | FORMAL_TRACE | CONFIRMED |
| PRD-006 | SYS-006 | FORMAL_TRACE | CONFIRMED |
| AC-006 | SYS-006 | FORMAL_TRACE | CONFIRMED |
| PRD-007 | SYS-007 | FORMAL_TRACE | CONFIRMED |
| AC-007 | SYS-007 | FORMAL_TRACE | CONFIRMED |
| PRD-008 | SYS-008 | FORMAL_TRACE | CONFIRMED |
| AC-008 | SYS-008 | FORMAL_TRACE | CONFIRMED |
| PRD-009 | SYS-009 | FORMAL_TRACE | CONFIRMED |
| AC-009 | SYS-009 | FORMAL_TRACE | CONFIRMED |
| PRD-003 | NFR-001 | FORMAL_TRACE | CONFIRMED |
| PRD-001 | NFR-002 | FORMAL_TRACE | CONFIRMED |
| PRD-009 | NFR-003 | FORMAL_TRACE | CONFIRMED |
| PRD-006 | NFR-004 | FORMAL_TRACE | CONFIRMED |
| PRD-009 | NFR-005 | FORMAL_TRACE | CONFIRMED |
| PRD-002 | NFR-006 | FORMAL_TRACE | CONFIRMED |
| PRD-008 | IF-001 | FORMAL_TRACE | CONFIRMED |
| AC-008 | IF-001 | FORMAL_TRACE | CONFIRMED |
| PRD-006 | IF-002 | FORMAL_TRACE | CONFIRMED |
| AC-006 | IF-002 | FORMAL_TRACE | CONFIRMED |
| PRD-007 | IF-003 | FORMAL_TRACE | CONFIRMED |
| AC-007 | IF-003 | FORMAL_TRACE | CONFIRMED |

---

## 3. 下游实现与验证追溯

| 系统需求 | 架构/设计 | 代码模块 | 测试 | 状态 |
|---|---|---|---|---|
| SYS-001 | ARCH-APLS-CNL-001 §4；DES-APLS-CNL-FRAME-001；DES-APLS-CNL-FRONTEND-001 | 07_src/crates/apls-compiler（cnl_lexer.rs 候选枚举、cnl_pipeline.rs 收敛 Gate） | CNL-C001、CNL-C015 | MAPPED |
| SYS-002 | DES-APLS-CNL-FRAME-001；DES-APLS-CNL-DIAG-001 §5 | cnl_pipeline.rs（收敛 Gate 与 E1310 分歧见证） | CNL-C004、CNL-C011 | MAPPED |
| SYS-003 | DES-APLS-CNL-IR-001；apls-cnl-ir-0.1.schema.json | cnl_pipeline.rs（build_ir、Canonicalization 与发布复验） | CNL-C010、CNL-C014 | MAPPED |
| SYS-004 | DES-APLS-CNL-SEMVAL-001 | cnl_pipeline.rs（bind / type 阶段） | CNL-C002、CNL-C003 | MAPPED |
| SYS-005 | DES-APLS-CNL-SEMVAL-001；DES-APLS-CNL-DIAG-001（E1402～E1405） | cnl_pipeline.rs（validate_transitions、validate_rule_conflicts） | CNL-C005、CNL-C006、CNL-C017、CNL-C018 | MAPPED |
| SYS-006 | DES-APLS-CNL-DIAG-001 §1.1/§1.2 | diagnostic.rs、cnl_pipeline.rs（封闭根因聚合与总排序） | CNL-C010、CNL-C013、CNL-C016 | MAPPED |
| SYS-007 | DES-APLS-CNL-IR-001；apls-cnl-ir-0.1.schema.json | cnl_pipeline.rs（Verified IR 发布与跨节点复验） | CNL-C014 | MAPPED |
| SYS-008 | DES-APLS-COMPILER-001；DES-APLS-COMPILER-FOUNDATION-001 | 07_src/crates/apls-cli（lib.rs、main.rs） | apls-cli 内嵌 CLI 黑盒测试 | MAPPED |
| SYS-009 | DES-APLS-ZH-CNL-001；APLS_0.1_ZH_CN_GRAMMAR.ebnf；DES-APLS-CNL-AMB-001 | cnl_lexer.rs、apls_grammar.lalrpop、cnl_pipeline.rs | CNL-C001、CNL-C003、CNL-C009 | MAPPED |
| NFR-001 | DES-APLS-CNL-RESOURCE-001（稳定顺序）；ARCH-APLS-CNL-001 §4 | cnl_pipeline.rs（BTreeMap/BTreeSet 全序确定性） | CNL-C010、CNL-C015、CNL-C016 | MAPPED |
| NFR-002 | DES-APLS-COMPILER-FOUNDATION-001（精确依赖闭包） | Cargo.lock（锁定离线构建） | CNL-C010；候选 Commit 复现 checkout | MAPPED |
| NFR-003 | 版本化 Profile 与 Schema（apls-zh-CN-0.1、apls-cnl-ir-0.1） | IR Header 版本字段 | CNL-C014；`--version` 固定行 | MAPPED |
| NFR-004 | ARCH-APLS-CNL-001 §4（失败关闭点）；DES-APLS-CNL-DIAG-001 | cnl_pipeline.rs、apls-cli（失败关闭与原子发布） | Conformance 负向样例全套 | MAPPED |
| NFR-005 | DES-APLS-ZH-CNL-001（规范含义机械定义）；DEC-014 边界 | 依赖闭包不含 LLM 组件 | 契约文档与实现一致性评审 | MAPPED |
| NFR-006 | 06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md | — | CNL-C001～C020 追溯表 | MAPPED |
| IF-001 | DES-APLS-COMPILER-001（命令表面、退出码与发布契约） | 07_src/crates/apls-cli | apls-cli 内嵌 CLI 黑盒测试 | MAPPED |
| IF-002 | DES-APLS-CNL-DIAG-001；apls-cnl-diagnostic-0.1.schema.json | diagnostic.rs | CNL-C013、CNL-C014、CNL-C016 | MAPPED |
| IF-003 | DES-APLS-CNL-IR-001；apls-cnl-ir-0.1.schema.json | cnl_pipeline.rs（发布与复验） | CNL-C014 | MAPPED |

状态：`MAPPED / PARTIAL / MISSING / UNKNOWN`

---

## 4. Traceability Mechanical Verification

在建立或复审正式需求 Baseline 前执行：

```bash
python3 09_quality/traceability/validate_traceability.py
```

### 4.1 Node Coverage

```text
Expected IDs: 18
Covered IDs: 18
Missing IDs: 0
Unexpected IDs: 0

NODE RESULT: PASS
```

### 4.2 Edge Consistency

```text
Detailed Metadata Edges: 30
Traceability Matrix Edges: 30
Intersection: 30
Detailed-only: 0
Matrix-only: 0
Duplicate Metadata Edges: 0
Duplicate Matrix Edges: 0
Invalid Source IDs: 0

EDGE RESULT: PASS
```

### 4.3 Overall Result

2026-09-06 执行 `python3 09_quality/traceability/validate_traceability.py`，Node Coverage 与 Edge Consistency 均通过（`RESULT: PASS`）：

```text
TRACEABILITY_CLOSED
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
