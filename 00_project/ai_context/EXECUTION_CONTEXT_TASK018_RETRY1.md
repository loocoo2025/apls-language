# TASK-018 C03 Retry 1 受控执行上下文

> 本上下文只授权关闭 `IIR-APLS-TASK018-IMPLEMENTATION-001` 的 `IO-01～IO-07`，不得扩张为一般重构、设计改写或发布工作。

## Dynamic Role Profile

```yaml
profile_id: DRP-APLS-C03-018-R1
schema_version: "1.0"
role_id: C03
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C03_IMPLEMENTATION.md
project_ref: APLS
task_or_work_package_id: TASK-018-RETRY1
current_or_applicable_gate_binding:
  gate_id: C03_TASK018_IO01_IO07_REMEDIATION_IN_PROGRESS
  gate_status: IN_PROGRESS
  authority_source: 00_project/ai_context/CURRENT_STATE.md
applicable_fact_owner_bindings:
  - { fact_scope: project_execution_state, fact_owner_ref: 00_project/ai_context/CURRENT_STATE.md, authority_source: AI_START_HERE.md }
  - { fact_scope: task_state, fact_owner_ref: 00_project/ai_context/ACTIVE_TASKS.md, authority_source: AI_START_HERE.md }
  - { fact_scope: approved_retry_and_feature_closure, fact_owner_ref: 00_project/ai_context/DECISION_INDEX.md, authority_source: DEC-024 }
upstream_roles: [HUMAN_PROJECT_OWNER, C00, INDEPENDENT_REVIEWER]
downstream_roles: [C00, INDEPENDENT_REVIEWER]
inputs: [DEC-017, DEC-019, DEC-020, DEC-022, DEC-023, DEC-024, IIR-APLS-TASK018-IMPLEMENTATION-001]
outputs: [io_01_to_io_07_remediation, minimal_closure_tests, corrected_c03_validation_record]
allowed_process_starts: [targeted_implementation_retry, targeted_validation, self_review]
allowed_tools_and_actions: [read_project_files, edit_exact_retry_files, run_fmt_check_locked_build_and_targeted_tests]
forbidden_actions_and_side_effects: [unrelated_refactor, public_language_change, canonical_ir_semantic_change, resource_limit_change, dependency_or_lock_change, commit, push, baseline_adoption, formal_c04, c05, release, formal_seal]
authority_source: HDP-APLS-019_OPTION_A_DEC-024
authorized_until: TASK018_RETRY1_OUTPUT_READY_OR_BLOCKED
human_escalation_triggers: [approved_contract_conflict, dependency_version_or_lock_change_required, public_behavior_change, retry_scope_expansion]
knowledge_manifest: KM-APLS-C03-018-R1
interaction_contract_refs: [INT-APLS-C00-C03-018-R1]
model_binding: CURRENT_CODEX_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: CONTROLLED_C03_WORKER_SESSION
enforcement_mode: PROCEDURAL_FALLBACK
generated_from: [CURRENT_STATE.md, ACTIVE_TASKS.md, HUMAN_DETERMINATION_019.md, INFORMAL_INDEPENDENT_REVIEW_TASK018_IMPLEMENTATION.md]
validity: TASK018_RETRY1_ONLY
invalidation_conditions: [retry_output_ready, retry_blocked, contract_conflict, authority_change, target_change]
readiness: ROLE_PROFILE_READY
```

## Knowledge Manifest

```yaml
manifest_id: KM-APLS-C03-018-R1
governance_version: v0.1.5
git_anchor: PROJECT_GIT_NO_COMMIT
minimum_required_sources:
  - AI_START_HERE.md@6fa9b80fb63fe92189f59d684417ff4132c9a7b99678feaa21484109e473038e
  - 00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md@67dc4600cd1cc6da0c683bb4420f2305553eb0c0fabe6b9ca0c5f925d88067c8
  - 00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml@51ef929daade9c3e980f2e6f8ee3f6bd1bf02c1a4b451c4ceb8cbbcc293f05ac
  - 00_project/governance/AI_TESTING_GOVERNANCE_RULES.md@385b7867c7a9ac88adfde6685c1a1ad484db2ebc1624e1c1fafe989a5a3a8883
  - 00_project/ai_context/ROLE_BRIEFS/C03_IMPLEMENTATION.md
task_specific_sources:
  - 00_project/ai_context/CURRENT_STATE.md@ab61a755f8e175c60e03b5ddc64c115c22b3ace476f8fc245a0bfb7612a6d325
  - 00_project/ai_context/ACTIVE_TASKS.md@7a152d92f9e8cc0f38f840b8fbf8178027a1180a67c2501e4c8ffdd47aef186a
  - 00_project/ai_context/DECISION_INDEX.md@d3e6540e0e7e6a838c4648f081c8db7912802045d819d55c21fb55f24c75e109
  - 00_project/ai_context/HUMAN_DETERMINATION_019.md@b32b9a9bc4228691ad6137412760776e36f3bb18a730099124cf5300b2784da0
  - 05_reviews/INFORMAL_INDEPENDENT_REVIEW_TASK018_IMPLEMENTATION.md@71932b98f1b7fb2b518912354e6f4bac857610b82cdde156e3b39581a6b7ae72
  - 04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md@2cc8ac836fb82b41deef09841903506c38804106daa09a2884be04a1663e0818
  - 04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md@d0b62e4e75017ce633dab91e8f0e2597b883a9d5a1ee0bc5fb6695b999063e65
loaded_source_digests: EMBEDDED_IN_REFERENCES_ABOVE
on_demand_search_allowed: true
searched_scopes: [io_01_to_io_07_direct_contracts_and_implementation]
unresolved_rule_gap: NONE
```

## Interaction Operation

```yaml
interaction_id: INT-APLS-C00-C03-018-R1
interaction_contract_id: C00_TO_C03_APPROVED_IMPLEMENTATION_RETRY
contract_version: "1.0"
sender_role: C00
receiver_role: C03
task_or_work_package_id: TASK-018-RETRY1
action_type: REMEDIATE_INDEPENDENT_REVIEW_OBSERVATIONS
scope: IO-01_THROUGH_IO-07_DIRECT_CLOSURE_ONLY
authority_source: HDP-APLS-019_OPTION_A_DEC-024
input_evidence: [IIR-APLS-TASK018-IMPLEMENTATION-001, DEC-024]
receipt: ACCEPTED_BY_CONTROLLED_C03_WORKER_SESSION
status: OUTPUT_READY
output_reference: TASK018_RETRY1_OUTPUT
terminal_state: OUTPUT_READY_OR_BLOCKED
error_and_escalation: C00_ON_CONTRACT_CONFLICT_OR_SCOPE_EXPANSION
audit_reference: HDP-APLS-019
```

## Authorization Contract

```yaml
authorization_id: AUTH-APLS-C03-018-R1
authority_owner: HUMAN_PROJECT_OWNER
action: REMEDIATE_IO_01_TO_IO_07
scope: IIR_APLS_TASK018_IMPLEMENTATION_001_DIRECT_CLOSURE
target:
  - 04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md
  - 07_src/crates/apls-compiler/src/cnl_lexer.rs
  - 07_src/crates/apls-compiler/src/apls_grammar.lalrpop
  - 07_src/crates/apls-compiler/src/cnl_pipeline.rs
  - 07_src/crates/apls-compiler/src/resource.rs
  - 07_src/crates/apls-compiler/src/diagnostic.rs
  - 07_src/crates/apls-cli/src/lib.rs
  - 07_src/crates/apls-cli/src/main.rs
  - 11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md
allowed_side_effects: [exact_file_edits, embedded_minimal_test_edits, local_build_outputs, validation_record_correction]
forbidden_side_effects: [cargo_manifest_change, cargo_lock_change, dependency_change, unrelated_source_edit, design_semantic_change, governance_state_edit, commit, push, baseline_adoption, formal_c04, c05, release, formal_seal]
validity: TASK018_RETRY1_ONLY
consumption_event: C03_RETRY_DISPATCH
terminal_state: TASK018_RETRY1_OUTPUT_READY_OR_BLOCKED
retry_policy: RETURN_TO_C00_ON_FAILED_CLOSURE_OR_SCOPE_CONFLICT
escalation: C00_AND_HUMAN_PROJECT_OWNER_WHEN_RESERVED_DECISION_REQUIRED
status: EXECUTION_COMPLETED_OUTPUT_READY
audit_reference: HDP-APLS-019_OPTION_A_DEC-024
```
