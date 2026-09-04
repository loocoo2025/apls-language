# 需求评审记录

## Review Readiness

```text
REVIEW_ID: {{REVIEW_ID}}
REVIEW_LINE: INDEPENDENT_REVIEW
INDEPENDENT_REVIEW_SUBTYPE: FORMAL_C04
REVIEW_TARGET: {{TARGET}}
EXACT_GIT_COMMIT_OR_CONTROLLED_VERSION: {{COMMIT_OR_VERSION}}
TARGET_FROZEN: YES / NO
INDEPENDENT_REVIEW_SESSION: {{SESSION_ID_OR_REFERENCE}}
NEW_SESSION_EVIDENCE: {{EVIDENCE}}
EXECUTION_OR_REMEDIATION_SESSIONS_EXCLUDED: {{SESSION_IDS_OR_NOT_APPLICABLE}}
PRIVATE_CONTEXT_INHERITED: NO
CONTEXT_PACKAGE_MANIFEST: {{MANIFEST_OR_REFERENCE}}
TARGET_BINDING_EVIDENCE: {{EVIDENCE}}
REVIEW_TARGET_ACCESS: READ_ONLY
ALLOWED_WRITE_PATHS: {{FORMAL_REVIEW_RECORD_ONLY_OR_NONE}}
GIT_WRITE_ALLOWED: NO
REMOTE_MUTATION_ALLOWED: NO
PRE_REVIEW_TARGET_STATUS: {{STATUS_EVIDENCE}}
POST_REVIEW_TARGET_STATUS: {{STATUS_EVIDENCE_OR_PENDING}}
FORMAL_REVIEW_RECORD_LOCATION_DEFINED: YES / NO
FORMAL_REVIEW_RECORD: {{PATH_OR_ID}}
REVIEW_READINESS: READY / REVIEW_NOT_READY
NOT_READY_REASONS: {{REASONS_OR_NOT_APPLICABLE}}
MISSING_INPUTS: {{INPUTS_OR_NOT_APPLICABLE}}
RESPONSIBLE_OWNER: {{OWNER_OR_NOT_APPLICABLE}}
RESTART_CONDITIONS: {{CONDITIONS_OR_NOT_APPLICABLE}}
```

`REVIEW_NOT_READY` 时只记录未就绪原因、责任人和重新发起条件，不填写 Finding 或正式 Gate Decision。独立性证据缺失同样属于未就绪。“只读”针对被评审对象，唯一 Scoped Write 只能是预定义 Review Record。完整规则见 `AI_ENGINEERING_RULES_V2.md` 第 38.7 节。

## Review Scope

- 适用需求 / Baseline：
- 适用决策 / ADR：
- 证据：
- 明确排除：

## Finding Summary

```text
OPEN_FINDINGS:
S0: {{COUNT}}
S1: {{COUNT}}
S2: {{COUNT}}
S3: {{COUNT}}

ADVISORIES: {{COUNT}}
```

| Finding ID | Severity | Finding | Evidence | Violated Basis / Acceptance Impact | Required Closure Condition | Default Route | Status | Closure Evidence |
|---|---|---|---|---|---|---|---|---|
| REV-001 | S0/S1/S2/S3 | | | | | | OPEN | |

## Advisory / Observation / Future Improvement

| Advisory ID | Type | Observation | Suggested Follow-up | Non-blocking Confirmation |
|---|---|---|---|---|
| ADV-001 | ADVISORY / OBSERVATION / FUTURE_IMPROVEMENT | | | YES |

## Formal Decision

只有 `REVIEW_READINESS: READY` 时才填写：

```text
ALL_APPLICABLE_MANDATORY_CHECKS_COMPLETED: YES / NO
REQUIRED_EVIDENCE_COMPLETE: YES / NO
ALL_APPLICABLE_EXCEPTIONS_APPROVED_BY_CORRECT_OWNER: YES / NO / NOT_APPLICABLE
FORMAL_DECISION: PASS / CHANGES_REQUESTED
DECISION_BASIS: {{SUMMARY}}
```

任一 Open Finding 都阻断 `PASS`。Finding 不得由提出它的 C04 Session 自行关闭；整改或正式 Exception 批准后，必须由面向新精确 Review Target 的全新独立 C04 Session 复核。

## 需求追溯机械门（适用于需求 Baseline / SRS 封板）

执行：

```bash
python3 09_quality/traceability/validate_traceability.py
```

记录：

```text
NODE CHECK
Expected IDs:
Covered IDs:
Missing IDs:
Unexpected IDs:
Node Result: PASS / FAIL

EDGE CHECK
Detailed Metadata Edges:
Traceability Matrix Edges:
Intersection:
Detailed-only:
Matrix-only:
Edge Result: PASS / FAIL

OVERALL
TRACEABILITY_CLOSED / TRACEABILITY_NOT_CLOSED
```

评审规则：
- 不得以“所有 ID 都出现”代替关系边一致性。
- 非零差异必须逐条有正式解释。
- 未通过时不得将需求追溯宣称为完整闭合。
