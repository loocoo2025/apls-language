# TASK-020 C05 最小风险驱动验证执行上下文

## Dynamic Role Profile

```yaml
profile_id: DRP-APLS-C05-020
schema_version: "1.0"
role_id: C05
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C05_VERIFICATION_RELEASE.md
project_ref: APLS
task_or_work_package_id: TASK-020
current_or_applicable_gate_binding: { gate_id: C05_TASK020_VERIFICATION_IN_PROGRESS, gate_status: IN_PROGRESS, authority_source: 00_project/ai_context/CURRENT_STATE.md }
applicable_fact_owner_bindings:
  - { fact_scope: project_execution_state, fact_owner_ref: 00_project/ai_context/CURRENT_STATE.md, authority_source: AI_START_HERE.md }
  - { fact_scope: task_state, fact_owner_ref: 00_project/ai_context/ACTIVE_TASKS.md, authority_source: AI_START_HERE.md }
  - { fact_scope: verification_scope, fact_owner_ref: 00_project/ai_context/HUMAN_DETERMINATION_020.md, authority_source: DEC-025 }
upstream_roles: [C00, C03, INDEPENDENT_REVIEWER]
downstream_roles: [C00]
inputs: [TASK-018_READY_FOR_REVIEW, IIR-APLS-TASK018-IMPLEMENTATION-003, CNL-C001_TO_C016, DEC-025]
outputs: [minimal_verification_plan, c05_verification_report, residual_risk]
allowed_process_starts: [risk_driven_test_planning, locked_offline_validation, approved_high_risk_conformance]
allowed_tools_and_actions: [read_direct_contracts_and_implementation, run_approved_cargo_commands, download_unicode_17_official_data_to_private_tmp, write_two_evidence_files, update_current_truth]
forbidden_actions_and_side_effects: [compiler_or_test_edit, contract_or_schema_edit, cargo_or_lock_change, dependency_install_or_upgrade, unrelated_test_expansion, commit, push, baseline_adoption, formal_c04, release, formal_seal]
authority_source: HDP-APLS-020_OPTION_A_DEC-025
authorized_until: TASK020_OUTPUT_READY_OR_BLOCKED
human_escalation_triggers: [acceptance_threshold_change, source_or_test_fix_required, official_dataset_identity_unconfirmed, release_or_baseline_decision]
knowledge_manifest: KM-APLS-C05-020
interaction_contract_refs: [INT-APLS-C00-C05-020]
model_binding: CURRENT_CODEX_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: CONTINUED_PRIMARY_EXECUTOR_SESSION
enforcement_mode: PROCEDURAL_FALLBACK
generated_from: [CURRENT_STATE.md, ACTIVE_TASKS.md, HUMAN_DETERMINATION_020.md, C05_VERIFICATION_RELEASE.md]
validity: TASK020_ONLY
invalidation_conditions: [output_ready, blocked, scope_change, implementation_or_test_change]
readiness: ROLE_PROFILE_READY
```

## Knowledge Manifest

```yaml
manifest_id: KM-APLS-C05-020
governance_version: v0.1.5
git_anchor: PROJECT_GIT_NO_COMMIT
minimum_required_sources:
  - AI_START_HERE.md@6fa9b80fb63fe92189f59d684417ff4132c9a7b99678feaa21484109e473038e
  - 00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md@67dc4600cd1cc6da0c683bb4420f2305553eb0c0fabe6b9ca0c5f925d88067c8
  - 00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml@51ef929daade9c3e980f2e6f8ee3f6bd1bf02c1a4b451c4ceb8cbbcc293f05ac
  - 00_project/governance/AI_TESTING_GOVERNANCE_RULES.md@385b7867c7a9ac88adfde6685c1a1ad484db2ebc1624e1c1fafe989a5a3a8883
  - 00_project/ai_context/ROLE_BRIEFS/C05_VERIFICATION_RELEASE.md@6990bbbbd94c6b5b08fab1f7cd153b10565f92ed7953d69b55aeae4646a3911a
task_specific_sources:
  - 00_project/ai_context/CURRENT_STATE.md@2520be15cc1d54c1d074e152560efc3fdda76c6dd75c817c86fc43ce29c98b15
  - 00_project/ai_context/ACTIVE_TASKS.md@a5608f21806a512df219b8b9e6208ad2a9d06ddd415d7a6f6fde83f54cb72b9e
  - 00_project/ai_context/BASELINE_INDEX.md@04a2598585191c98ec15643631a7b40e3bc3a430ac58d5cf2fe14ba1c9154620
  - 00_project/ai_context/DECISION_INDEX.md@ed60dce980e0ec99b76835dd152ae53da261a8395292e3e41a7402dfd740b6c8
  - 00_project/ai_context/HUMAN_DETERMINATION_020.md@5b57ff8cf47a10660524471bd3ba7f1fc7b9dd976f93422c27d96cdf312cb436
  - 06_test_design/APLS_0.1_CNL_PREIMPLEMENTATION_CONFORMANCE.md@99cc305bc9a0c40b9cda5438d8371eb50faede399ba4c7bf80ef3511fa603550
  - 04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md@d0b62e4e75017ce633dab91e8f0e2597b883a9d5a1ee0bc5fb6695b999063e65
  - 11_validation/validation_reports/TASK018_C03_IMPLEMENTATION_VALIDATION.md@d7934bd6ddce1c293a7308dd0d16a0c997ab5d2adb6950c2941437a2c785a7d7
  - 05_reviews/INFORMAL_INDEPENDENT_REREVIEW_TASK018_IMPLEMENTATION_R2.md@b41f880e39daa226fbfb80ff0c18d65df8fbbbad53c58d4e5d679f3933de0647
loaded_source_digests: EMBEDDED_IN_REFERENCES_ABOVE
on_demand_search_allowed: true
searched_scopes: [cnl_c001_to_c016_test_mapping, approved_t0_t1_commands, two_ignored_high_risk_tests]
unresolved_rule_gap: NONE
```

## Interaction Operation

```yaml
interaction_id: INT-APLS-C00-C05-020
interaction_contract_id: C00_TO_C05_APPROVED_VERIFICATION
contract_version: "1.0"
sender_role: C00
receiver_role: C05
task_or_work_package_id: TASK-020
action_type: VERIFY_TASK018_CNL_IMPLEMENTATION
scope: MINIMAL_RISK_DRIVEN_CNL_C001_TO_C016_AND_TWO_HIGH_RISK_CASES
authority_source: HDP-APLS-020_OPTION_A_DEC-025
input_evidence: [TASK018_READY_FOR_REVIEW, IIR-APLS-TASK018-IMPLEMENTATION-003]
receipt: ACCEPTED_BY_PRIMARY_C05_EXECUTOR
status: COMPLETED
output_reference: TASK020_C05_OUTPUT
terminal_state: OUTPUT_READY_OR_BLOCKED
error_and_escalation: RETURN_TO_C00_WITHOUT_SOURCE_OR_TEST_REMEDIATION
audit_reference: HDP-APLS-020
```

## Authorization Contract

```yaml
authorization_id: AUTH-APLS-C05-020
authority_owner: HUMAN_PROJECT_OWNER
action: EXECUTE_MINIMAL_RISK_DRIVEN_VERIFICATION
scope: CNL_C001_TO_C016_TRACEABILITY_LOCKED_NORMAL_TESTS_PUBLIC_RESOURCE_BOUNDARY_UNICODE17_CONFORMANCE
target:
  - 06_test_design/APLS_0.1_CNL_IMPLEMENTATION_VERIFICATION_PLAN.md
  - 11_validation/validation_reports/TASK020_C05_CNL_VERIFICATION.md
  - /private/tmp/NormalizationTest-17.0.0.txt
allowed_side_effects: [two_evidence_file_writes, private_tmp_dataset, cargo_target_outputs, minimum_current_truth_update]
forbidden_side_effects: [compiler_or_cli_source_edit, existing_test_edit, grammar_design_schema_cargo_or_lock_edit, dependency_change, extra_test_classes, commit, push, baseline_adoption, formal_c04, release, formal_seal]
validity: TASK020_ONLY
consumption_event: C05_VERIFICATION_DISPATCH
terminal_state: TASK020_OUTPUT_READY_OR_BLOCKED
retry_policy: RETURN_TO_C00_ON_FAILURE_OR_UNCONFIRMED_DATASET
escalation: HUMAN_PROJECT_OWNER_FOR_SCOPE_OR_ACCEPTANCE_CHANGE
status: EXECUTION_COMPLETED
audit_reference: HDP-APLS-020_OPTION_A_DEC-025
```
