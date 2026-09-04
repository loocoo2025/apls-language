# TASK-021 本地初始 Commit 执行上下文

## Dynamic Role Profile

```yaml
profile_id: DRP-APLS-C00-021
schema_version: "1.0"
role_id: C00
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C00_CONTROL.md
project_ref: APLS
task_or_work_package_id: TASK-021
current_or_applicable_gate_binding: { gate_id: TASK021_LOCAL_INITIAL_COMMIT_IN_PROGRESS, gate_status: IN_PROGRESS, authority_source: 00_project/ai_context/CURRENT_STATE.md }
applicable_fact_owner_bindings:
  - { fact_scope: project_execution_state, fact_owner_ref: 00_project/ai_context/CURRENT_STATE.md, authority_source: AI_START_HERE.md }
  - { fact_scope: task_state, fact_owner_ref: 00_project/ai_context/ACTIVE_TASKS.md, authority_source: AI_START_HERE.md }
  - { fact_scope: commit_authorization, fact_owner_ref: 00_project/ai_context/HUMAN_DETERMINATION_021.md, authority_source: DEC-026 }
upstream_roles: [HUMAN_PROJECT_OWNER, C05]
downstream_roles: [C00, C04]
inputs: [TASK018_DONE, TASK020_DONE, VAL-APLS-C05-020_PASS, DEC-026]
outputs: [one_local_initial_commit, exact_commit_hash, staged_set_evidence]
allowed_process_starts: [commit_target_inventory, git_stage, one_git_commit, post_commit_reconciliation]
allowed_tools_and_actions: [read_project_files, edit_commit_governance_receipt, edit_gitignore_for_build_cache, git_add, git_diff_cached, git_commit, git_rev_parse]
forbidden_actions_and_side_effects: [product_or_source_change, second_commit, push, tag, branch, pr, baseline_adoption, formal_c04, release, formal_seal]
authority_source: HDP-APLS-021_OPTION_A_DEC-026
authorized_until: TASK021_COMMIT_CREATED_OR_FAILED
human_escalation_triggers: [unexpected_sensitive_file, staged_scope_change, commit_failure_requiring_retry, git_identity_missing]
knowledge_manifest: KM-APLS-C00-021
interaction_contract_refs: [INT-APLS-C00-GIT-021]
model_binding: CURRENT_CODEX_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: CONTINUED_C00_CONTROL_SESSION
enforcement_mode: PROCEDURAL_FALLBACK
generated_from: [CURRENT_STATE.md, ACTIVE_TASKS.md, HUMAN_DETERMINATION_021.md, TASK020_C05_CNL_VERIFICATION.md]
validity: TASK021_ONLY
invalidation_conditions: [commit_created, commit_failed, scope_change, sensitive_file_found]
readiness: ROLE_PROFILE_READY
```

## Knowledge Manifest

```yaml
manifest_id: KM-APLS-C00-021
governance_version: v0.1.5
git_anchor: PROJECT_GIT_NO_COMMIT_AT_START
minimum_required_sources: [AI_START_HERE.md, ROLE_INTERACTION_EXECUTION_POLICY.md, GOVERNANCE_EXECUTION_CONTRACTS.yaml, C00_CONTROL.md]
task_specific_sources: [CURRENT_STATE.md, ACTIVE_TASKS.md, BASELINE_INDEX.md, DECISION_INDEX.md, HUMAN_DETERMINATION_021.md, TASK020_C05_CNL_VERIFICATION.md, .gitignore]
loaded_source_digests: TO_BE_BOUND_BY_FINAL_STAGED_TREE_AND_COMMIT
on_demand_search_allowed: true
searched_scopes: [non_ignored_file_inventory, build_cache_exclusion, large_files, obvious_private_key_patterns, git_identity]
unresolved_rule_gap: NONE
```

## Interaction Operation

```yaml
interaction_id: INT-APLS-C00-GIT-021
interaction_contract_id: HUMAN_TO_C00_APPROVED_LOCAL_COMMIT
contract_version: "1.0"
sender_role: HUMAN_PROJECT_OWNER
receiver_role: C00
task_or_work_package_id: TASK-021
action_type: CREATE_ONE_LOCAL_INITIAL_COMMIT
scope: CURRENT_C05_PASSED_NON_IGNORED_PROJECT_SET
authority_source: HDP-APLS-021_OPTION_A_DEC-026
input_evidence: [VAL-APLS-C05-020_PASS]
receipt: ACCEPTED_BY_C00
status: COMPLETED
output_reference: COMMIT_3289037bee1aab64dfa2d58188379a68dcfa601e
terminal_state: COMMIT_CREATED_OR_FAILED
error_and_escalation: RETURN_TO_HUMAN_ON_FAILURE_NO_AUTOMATIC_RETRY
audit_reference: HUMAN_REPLY_AUTHORIZATION
```
