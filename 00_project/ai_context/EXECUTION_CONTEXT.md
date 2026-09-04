# 当前受控执行上下文

> `TASK-018` 已达 `OUTPUT_READY`，`AUTH-APLS-C03-018` 已消费至终态。本上下文保留为 C03 实施记录，不授权冒充独立复审。

## Dynamic Role Profile

```yaml
profile_id: DRP-APLS-C03-018
schema_version: "1.0"
role_id: C03
stable_role_brief_ref: 00_project/ai_context/ROLE_BRIEFS/C03_IMPLEMENTATION.md
project_ref: APLS
task_or_work_package_id: TASK-018
current_or_applicable_gate_binding:
  gate_id: C03_CNL_FRONTEND_FRAME_IR_IMPLEMENTATION_OUTPUT_READY
  gate_status: OUTPUT_READY
  authority_source: 00_project/ai_context/CURRENT_STATE.md
applicable_fact_owner_bindings:
  - { fact_scope: project_execution_state, fact_owner_ref: 00_project/ai_context/CURRENT_STATE.md, authority_source: AI_START_HERE.md }
  - { fact_scope: implementation_scope, fact_owner_ref: 00_project/ai_context/ACTIVE_TASKS.md, authority_source: DEC-020_AND_DEC-023 }
  - { fact_scope: approved_contracts, fact_owner_ref: 00_project/ai_context/DECISION_INDEX.md, authority_source: DEC-014_THROUGH_DEC-023 }
upstream_roles: [HUMAN_PROJECT_OWNER, C00, C02]
downstream_roles: [C00, C05, INDEPENDENT_REVIEWER]
inputs: [DEC-010, DEC-011, DEC-014, DEC-015, DEC-017, DEC-018, DEC-019, DEC-020, DEC-022, DEC-023, WP-APLS-CNL-C03-001]
outputs: [complete_cnl_compiler_vertical_slice, minimal_targeted_tests, dependency_lock_evidence, lite_implementation_record]
allowed_process_starts: [cnl_implementation, cargo_lock_generation, targeted_validation, self_review]
allowed_tools_and_actions: [read_project_files, modify_authorized_source_and_tests, modify_approved_dependency_manifest_and_lock, run_fmt_check_and_targeted_tests]
forbidden_actions_and_side_effects: [public_contract_change, resource_limit_change, extra_dependency_change, commit, push, baseline_adoption, formal_c04, release, formal_seal]
authority_source: HDP-APLS-016_OPTION_A_AND_HDP-APLS-018_OPTION_A
authorized_until: TASK018_OUTPUT_READY
human_escalation_triggers: [contract_conflict, dependency_closure_mismatch, msrv_or_license_failure, public_behavior_change, task_scope_expansion]
knowledge_manifest: KM-APLS-C03-018
interaction_contract_refs: [INT-APLS-C00-C03-018]
model_binding: CURRENT_CODEX_MODEL
runtime_binding: CODEX_DESKTOP_LOCAL
harness_binding: CODEX
session_binding: CURRENT_PHYSICAL_SESSION
enforcement_mode: PROCEDURAL_FALLBACK
generated_from: [CURRENT_STATE.md, ACTIVE_TASKS.md, HUMAN_DETERMINATION_018.md, APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md]
validity: CURRENT_GATE_ONLY
invalidation_conditions: [task018_output, task_blocked, contract_conflict, dependency_validation_failure, authority_change, baseline_change, session_change]
readiness: ROLE_PROFILE_READY
```

## Knowledge Manifest

```yaml
manifest_id: KM-APLS-C03-018
governance_version: v0.1.5
git_anchor: PROJECT_GIT_NO_COMMIT
minimum_required_sources:
  - AI_START_HERE.md@6fa9b80fb63fe92189f59d684417ff4132c9a7b99678feaa21484109e473038e
  - 00_project/governance/ROLE_INTERACTION_EXECUTION_POLICY.md@67dc4600cd1cc6da0c683bb4420f2305553eb0c0fabe6b9ca0c5f925d88067c8
  - 00_project/governance/GOVERNANCE_EXECUTION_CONTRACTS.yaml@51ef929daade9c3e980f2e6f8ee3f6bd1bf02c1a4b451c4ceb8cbbcc293f05ac
  - 00_project/governance/AI_TESTING_GOVERNANCE_RULES.md@385b7867c7a9ac88adfde6685c1a1ad484db2ebc1624e1c1fafe989a5a3a8883
  - 00_project/ai_context/ROLE_BRIEFS/C03_IMPLEMENTATION.md
task_specific_sources:
  - 00_project/ai_context/HUMAN_DETERMINATION_018.md@e791be7bd0f1f7b0622e1b2baebfddf5fb1d1a846a8a022b17e4db277963330b
  - 04_design/compiler/APLS_0.1_CNL_IMPLEMENTATION_WORK_PACKAGE.md@805159ebcaa52797c8c6a1fbcfd41a5949347ba88a3091bb6d4f1124ac28f7c4
  - 04_design/language/APLS_0.1_ZH_CN_LANGUAGE_PROFILE.md@916165b00a4c7b366d8fe1f5e6b7bf54537de437cf6d3289fcf7ea5458e1c796
  - 04_design/language/APLS_0.1_ZH_CN_GRAMMAR.ebnf@5fde2efa389d770e94fc6723fb31d549783d55e33fc849ce131eddb5b8cab8f0
  - 04_design/language/APLS_0.1_SEMANTIC_FRAME_PROFILE.md@e080e01844b9bd8d69806e90dfb2b102aa20b60952d68ea7e4ad35a5cd86f01a
  - 04_design/language/APLS_0.1_CNL_SEMANTIC_VALIDATION_PROFILE.md@f0f0a21c24a92bd6a48979d3b8244af5cb112ffa1415aacb23ba7b52b4a147cc
  - 04_design/compiler/APLS_0.1_CNL_RESOURCE_AND_ORDERING_PROFILE.md@340023bfe603a461209cb39b0bca2375ebea9540353fa101b5295841cc715cf7
  - 04_design/compiler/APLS_0.1_CNL_UNICODE_NFC_PROFILE.md@2cc8ac836fb82b41deef09841903506c38804106daa09a2884be04a1663e0818
  - 04_design/ir/APLS_0.1_CNL_CANONICAL_IR.md@3f43bd796b57e605326397e7d51c96cfffb88f224fd73c7897a25ed4d1a60469
  - 04_design/ir/apls-cnl-ir-0.1.schema.json@745a7f9bbeccec5c4fb03b31a6cc059cea972986b2bca7dd9fd77db210d359d6
  - 04_design/diagnostics/APLS_0.1_CNL_DIAGNOSTICS.md@78c4a675433fc6f8cd551ff8808e99fe8043693fb4bb154b69895b0b206f1099
  - 04_design/diagnostics/apls-cnl-diagnostic-0.1.schema.json@66ad276942776a7d184e82ba39a188f98b6177d2dd67256994a51f6a2c271e26
loaded_source_digests: EMBEDDED_IN_REFERENCES_ABOVE
on_demand_search_allowed: true
searched_scopes: [task018_approved_contracts, existing_legacy_implementation, compiler_cli_contract]
unresolved_rule_gap: NONE
```

## Interaction Operation

```yaml
interaction_id: INT-APLS-C00-C03-018
interaction_contract_id: C00_TO_C03_APPROVED_IMPLEMENTATION
contract_version: "1.0"
sender_role: C00
receiver_role: C03
task_or_work_package_id: TASK-018
action_type: EXECUTE_APPROVED_CNL_VERTICAL_SLICE
scope: WP_APLS_CNL_C03_001
authority_source: HDP-APLS-018_OPTION_A_DEC-023
input_evidence: [DEC-020, DEC-023, IIR-APLS-TASK019-REREVIEW-003]
receipt: ACCEPTED
status: OUTPUT_READY
output_reference: TASK018_IMPLEMENTATION_OUTPUT
terminal_state: OUTPUT_READY
error_and_escalation: HUMAN_PROJECT_OWNER_ON_CONTRACT_OR_DEPENDENCY_MISMATCH
audit_reference: HDP-APLS-018
```

## Authorization Contract

```yaml
authorization_id: AUTH-APLS-C03-018
authority_owner: HUMAN_PROJECT_OWNER
action: IMPLEMENT_COMPLETE_CNL_COMPILER_VERTICAL_SLICE
scope: WP_APLS_CNL_C03_001_AND_EXACT_UNICODE_DEPENDENCY
target: [07_src_apls_compiler, 07_src_apls_cli, 07_src_cargo_lock, 08_tests_minimal, lite_governance_evidence]
allowed_side_effects: [source_edit, minimal_test_edit, compiler_manifest_edit, approved_lock_generation, local_build_and_test, lite_status_update]
forbidden_side_effects: [unapproved_contract_change, extra_dependency_change, commit, push, baseline_adoption, formal_c04, release, formal_seal]
validity: CURRENT_TASK018_GATE
consumption_event: FIRST_AUTHORIZED_IMPLEMENTATION_WRITE
terminal_state: TASK018_OUTPUT_READY_OR_BLOCKED
retry_policy: FIX_IMPLEMENTATION_DEFECTS_WITHIN_SAME_SCOPE
escalation: HUMAN_PROJECT_OWNER_FOR_CONTRACT_OR_DEPENDENCY_MISMATCH
status: CONSUMED_OUTPUT_READY
audit_reference: HDP-APLS-018_OPTION_A
```
