# TASK-018 C03 Retry 2 受控执行上下文

> 只关闭 `NB-01 / IO-02 PARTIAL`；其余 IO 已由独立再复审关闭，不得重做。

## Dynamic Role Profile

```yaml
profile_id: DRP-APLS-C03-018-R2
schema_version: "1.0"
role_id: C03
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C03_IMPLEMENTATION.md
project_ref: APLS
task_or_work_package_id: TASK-018-RETRY2
current_or_applicable_gate_binding: { gate_id: C03_TASK018_RETRY2_NB01_IN_PROGRESS, gate_status: IN_PROGRESS, authority_source: 00_project/ai_context/CURRENT_STATE.md }
applicable_fact_owner_bindings:
  - { fact_scope: project_execution_state, fact_owner_ref: 00_project/ai_context/CURRENT_STATE.md, authority_source: AI_START_HERE.md }
  - { fact_scope: task_state, fact_owner_ref: 00_project/ai_context/ACTIVE_TASKS.md, authority_source: AI_START_HERE.md }
  - { fact_scope: retry_scope, fact_owner_ref: 05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK018_IMPLEMENTATION_R1.md, authority_source: DEC-024 }
upstream_roles: [C00, INDEPENDENT_REVIEWER]
downstream_roles: [C00, INDEPENDENT_REVIEWER]
inputs: [DEC-017, DEC-022, DEC-024, NB-01]
outputs: [nb_01_fix, minimal_spacing_tests, corrected_c03_validation_record]
allowed_process_starts: [targeted_implementation_retry, targeted_validation, self_review]
allowed_tools_and_actions: [read_direct_contracts, edit_three_authorized_files, run_fmt_locked_offline_check_and_tests]
forbidden_actions_and_side_effects: [unrelated_refactor, other_io_rework, contract_change, dependency_or_lock_change, commit, push, baseline_adoption, formal_c04, c05, release, formal_seal]
authority_source: HDP-APLS-019_OPTION_A_DEC-024_IO02_DIRECT_CLOSURE
authorized_until: TASK018_RETRY2_OUTPUT_READY_OR_BLOCKED
human_escalation_triggers: [contract_conflict, public_behavior_change_beyond_ebnf, scope_expansion]
knowledge_manifest: KM-APLS-C03-018-R2
interaction_contract_refs: [INT-APLS-C00-C03-018-R2]
model_binding: CURRENT_CODEX_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: CONTINUED_CONTROLLED_C03_WORKER_SESSION
enforcement_mode: PROCEDURAL_FALLBACK
generated_from: [CURRENT_STATE.md, ACTIVE_TASKS.md, INFORMAL_INDEPENDENT_REREVIEW_TASK018_IMPLEMENTATION_R1.md]
validity: TASK018_RETRY2_ONLY
invalidation_conditions: [retry_output_ready, retry_blocked, contract_conflict, scope_change]
readiness: ROLE_PROFILE_READY
```

## Knowledge Manifest

```yaml
manifest_id: KM-APLS-C03-018-R2
governance_version: v0.1.5
git_anchor: PROJECT_GIT_NO_COMMIT
minimum_required_sources:
  - AI_START_HERE.md@6fa9b80fb63fe92189f59d684417ff4132c9a7b99678feaa21484109e473038e
  - 00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md@67dc4600cd1cc6da0c683bb4420f2305553eb0c0fabe6b9ca0c5f925d88067c8
  - 00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml@51ef929daade9c3e980f2e6f8ee3f6bd1bf02c1a4b451c4ceb8cbbcc293f05ac
  - 00_project/governance/AI_TESTING_GOVERNANCE_RULES.md@385b7867c7a9ac88adfde6685c1a1ad484db2ebc1624e1c1fafe989a5a3a8883
  - 00_project/ai_context/ROLE_BRIEFS/C03_IMPLEMENTATION.md
task_specific_sources:
  - 00_project/ai_context/CURRENT_STATE.md@cc7305a34f25dce8f3f41398cb71819707440c1916cb93f81ab74b9cf861ac98
  - 00_project/ai_context/ACTIVE_TASKS.md@3d11c0a1588ec5377622d36303292c5927e18a752e4d50f62b5e04434ce1bc57
  - 00_project/ai_context/DECISION_INDEX.md@d3e6540e0e7e6a838c4648f081c8db7912802045d819d55c21fb55f24c75e109
  - 00_project/ai_context/HUMAN_DETERMINATION_019.md@b32b9a9bc4228691ad6137412760776e36f3bb18a730099124cf5300b2784da0
  - 05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK018_IMPLEMENTATION_R1.md@73121dd9ea24a3edc72ed596b91e07d2f0c556fc06ec03ffbae0ac1c6c285d55
  - 07_src/crates/apls-compiler/src/cnl_pipeline.rs@8040b385cc6698c32b3eccc3810f817ef7729fbdc379b18d89ecc54daea93ee5
  - 07_src/crates/apls-cli/src/lib.rs@9d4815ff86960979ed3e33718db68fd655600f6fde7956caafff49d80e924d55
  - 11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md@53df049731310b8ab5eb643a42482645a1b94048aad4c0136e7b55cdcb45516e
loaded_source_digests: EMBEDDED_IN_REFERENCES_ABOVE
on_demand_search_allowed: true
searched_scopes: [nb_01_io_02_spacing_direct_closure]
unresolved_rule_gap: NONE
```

## Interaction Operation

```yaml
interaction_id: INT-APLS-C00-C03-018-R2
interaction_contract_id: C00_TO_C03_APPROVED_IMPLEMENTATION_RETRY
contract_version: "1.0"
sender_role: C00
receiver_role: C03
task_or_work_package_id: TASK-018-RETRY2
action_type: REMEDIATE_NB_01
scope: IO_02_SPACING_PRODUCTION_BINDING_ONLY
authority_source: HDP-APLS-019_OPTION_A_DEC-024
input_evidence: [IIR-APLS-TASK018-IMPLEMENTATION-002]
receipt: ACCEPTED_BY_CONTROLLED_C03_WORKER_SESSION
status: COMPLETED
output_reference: TASK018_RETRY2_OUTPUT_AND_IIR_APLS_TASK018_IMPLEMENTATION_003
terminal_state: OUTPUT_READY_OR_BLOCKED
error_and_escalation: C00_ON_CONTRACT_CONFLICT_OR_SCOPE_EXPANSION
audit_reference: IIR-APLS-TASK018-IMPLEMENTATION-002
```

## Authorization Contract

```yaml
authorization_id: AUTH-APLS-C03-018-R2
authority_owner: HUMAN_PROJECT_OWNER
action: REMEDIATE_NB_01
scope: IO_02_NUMBER_WITH_DECLARED_UNIT_WS1_ONLY
target:
  - 07_src/crates/apls-compiler/src/cnl_pipeline.rs
  - 07_src/crates/apls-cli/src/lib.rs
  - 11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md
allowed_side_effects: [exact_file_edits, embedded_minimal_test_edits, local_build_outputs, validation_record_update]
forbidden_side_effects: [other_source_edit, cargo_or_lock_change, dependency_change, contract_change, governance_state_edit, commit, push, baseline_adoption, formal_c04, c05, release, formal_seal]
validity: TASK018_RETRY2_ONLY
consumption_event: C03_RETRY2_DISPATCH
terminal_state: TASK018_RETRY2_OUTPUT_READY_OR_BLOCKED
retry_policy: RETURN_TO_C00_ON_FAILED_CLOSURE_OR_SCOPE_CONFLICT
escalation: C00_AND_HUMAN_PROJECT_OWNER_WHEN_RESERVED_DECISION_REQUIRED
status: EXECUTION_COMPLETED_OUTPUT_READY
audit_reference: HDP-APLS-019_OPTION_A_DEC-024
```
