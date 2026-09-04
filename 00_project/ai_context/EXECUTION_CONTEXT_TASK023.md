# TASK-023 GitHub 私有同步与 Kimi 交接执行上下文

## Dynamic Role Profile

```yaml
profile_id: DRP-APLS-C00-023
schema_version: "1.0"
role_id: C00
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C00_CONTROL.md
project_ref: APLS
task_or_work_package_id: TASK-023
current_or_applicable_gate_binding: { gate_id: TASK023_PRIVATE_REMOTE_SYNC, gate_status: IN_PROGRESS, authority_source: HUMAN_DETERMINATION_023.md }
applicable_fact_owner_bindings:
  - { fact_scope: project_execution_state, fact_owner_ref: 00_project/ai_context/CURRENT_STATE.md, authority_source: AI_START_HERE.md }
  - { fact_scope: task_state, fact_owner_ref: 00_project/ai_context/ACTIVE_TASKS.md, authority_source: AI_START_HERE.md }
  - { fact_scope: conversation_lifecycle, fact_owner_ref: 00_project/ai_context/CONVERSATION_MAP.md, authority_source: C00_CONTROL.md }
upstream_roles: [HUMAN_PROJECT_OWNER]
downstream_roles: [C00_V02, C04_AFTER_SEPARATE_AUTHORIZATION]
inputs: [COMMIT_3289037BEE1AAB64DFA2D58188379A68DCFA601E, HUMAN_DIRECT_REQUEST]
outputs: [private_github_repository, synchronized_main, kimi_handoff, copyable_start_instruction]
allowed_process_starts: [private_remote_creation, governance_snapshot_commit, non_force_push, handoff]
allowed_tools_and_actions: [edit_governance_and_handoff, git_add, git_commit, git_remote_add, git_push, remote_verification]
forbidden_actions_and_side_effects: [product_change, source_change, test_change, force_push, public_visibility, collaborator_invitation, formal_c04, baseline_adoption, release, formal_seal]
authority_source: HDP-APLS-023_DIRECT_USER_AUTHORIZATION
authorized_until: TASK023_REMOTE_SYNC_COMPLETED_OR_FAILED
human_escalation_triggers: [remote_name_conflict, visibility_conflict, non_fast_forward, sensitive_file_detected]
knowledge_manifest: KM-APLS-C00-023
interaction_contract_refs: [INT-APLS-C00-TRANSFER-023]
model_binding: CURRENT_CODEX_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: CONTINUED_C00_CONTROL_SESSION
enforcement_mode: PROCEDURAL_FALLBACK
generated_from: [CURRENT_STATE.md, HUMAN_DETERMINATION_023.md, AI_CONVERSATION_ORCHESTRATION_RULES.md, EXTERNAL_AI_TRANSFER_CONFIG.yaml]
validity: TASK023_ONLY
invalidation_conditions: [remote_sync_completed, remote_sync_failed, scope_change]
readiness: ROLE_PROFILE_READY
```

## Knowledge Manifest

```yaml
manifest_id: KM-APLS-C00-023
governance_version: v0.1.5
git_anchor: 3289037bee1aab64dfa2d58188379a68dcfa601e
minimum_required_sources: [AI_START_HERE.md, ROLE_INTERACTION_EXECUTION_POLICY.md, GOVERNANCE_EXECUTION_CONTRACTS.yaml, C00_CONTROL.md]
task_specific_sources: [CURRENT_STATE.md, BASELINE_INDEX.md, DECISION_INDEX.md, ACTIVE_TASKS.md, OPEN_QUESTIONS.md, CONVERSATION_MAP.md, AI_CONVERSATION_ORCHESTRATION_RULES.md, EXTERNAL_AI_TRANSFER_CONFIG.yaml]
loaded_source_digests: BOUND_BY_TASK023_GIT_COMMITS
on_demand_search_allowed: true
searched_scopes: [handoff_contract, external_transfer_boundary, git_remote_status, github_authentication, repository_visibility]
unresolved_rule_gap: NONE
```

## Interaction Operation

```yaml
interaction_id: INT-APLS-C00-TRANSFER-023
interaction_contract_id: HUMAN_TO_C00_PRIVATE_REMOTE_SYNC_AND_HANDOFF
contract_version: "1.0"
sender_role: HUMAN_PROJECT_OWNER
receiver_role: C00
task_or_work_package_id: TASK-023
action_type: PRIVATE_REMOTE_SYNC_AND_C00_HANDOFF
scope: CURRENT_PROJECT_MAIN_AND_GOVERNANCE_HANDOFF
authority_source: HDP-APLS-023_DIRECT_USER_AUTHORIZATION
input_evidence: [C05_PASS, COMMIT_3289037BEE1AAB64DFA2D58188379A68DCFA601E]
receipt: ACCEPTED_BY_C00
status: COMPLETED
output_reference: GITHUB_LOOCOO2025_APLS_LANGUAGE_COMMIT_FA3E09F_AND_KIMI_HANDOFF
terminal_state: REMOTE_SYNC_AND_HANDOFF_COMPLETED_OR_FAILED
error_and_escalation: RECONCILE_OR_RETURN_TO_HUMAN
audit_reference: HUMAN_DIRECT_REQUEST
```
