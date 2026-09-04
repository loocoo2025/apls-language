# HDP-APLS-021 — APLS 0.1 实现候选初始 Commit 授权

```yaml
determination_id: HDP-APLS-021
status: APPROVED
selected_option: A
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-04
authorization_text: "授权"
```

## 1. 授权解释

项目负责人在 C05 PASS 后、当前唯一下一步为“建立不可变 Git Commit Anchor”的上下文中明确回复“授权”。C00 将其解释为 Option A：创建一个本地初始 Commit，冻结当前已通过 C05 的 APLS 0.1 实现候选。

## 2. 精确范围

- Target：项目根目录中最终全部非 Git-ignored 普通项目文件；
- 排除 `.git/`、`07_src/target/`、`/private/tmp` 数据和其他 `.gitignore` 命中对象；
- 允许增加本授权记录、`DEC-026`、TASK-021 执行上下文和对应 Current Truth；
- 允许补充 `.gitignore` 的 `/07_src/target/` 规则，不删除该缓存；
- Commit 数量：`1`；
- Commit Message：`chore: freeze APLS 0.1 implementation candidate`；
- Commit 前检查 staged 文件集、`git diff --cached --check`、大文件和明显敏感文件；Commit 后读取精确 Hash。

## 3. 明确不授权

- 不修改产品行为、Compiler/CLI Source、现有测试、Grammar、设计、Schema、Cargo Manifest 或 Lock；
- 不 Push，不创建 Tag、Branch 或 PR；
- 不授权 Formal C04、Baseline Adoption、Release 或 Formal Seal；
- 本 Commit 只建立候选 Anchor，不自动成为产品 Baseline。

## 4. 授权合同

```yaml
authorization_id: AUTH-APLS-C00-021
authority_owner: HUMAN_PROJECT_OWNER
action: CREATE_ONE_LOCAL_INITIAL_COMMIT
scope: CURRENT_C05_PASSED_APLS_0_1_CANDIDATE_PLUS_COMMIT_GOVERNANCE_RECEIPT
target: FINAL_NON_IGNORED_STAGED_SET
allowed_side_effects: [git_index_write, one_local_commit, git_object_creation, precommit_governance_receipt, ignore_build_cache_rule]
forbidden_side_effects: [product_or_source_change, second_commit, push, tag, branch, pr, baseline_adoption, formal_c04, release, formal_seal]
validity: TASK021_ONLY
consumption_event: GIT_COMMIT_INVOCATION
terminal_state: COMMIT_CREATED_OR_FAILED
retry_policy: RETURN_TO_C00_ON_FAILURE_NO_AUTOMATIC_SECOND_COMMIT
escalation: HUMAN_PROJECT_OWNER_ON_SCOPE_OR_TARGET_CHANGE
status: EXECUTION_COMPLETED
audit_reference: HUMAN_REPLY_AUTHORIZATION_AFTER_TASK020_PASS
```

## 5. 结果

执行成功：

- Commit：`3289037bee1aab64dfa2d58188379a68dcfa601e`；
- Tree：`51edf42da73237bcb3408e234d2cbb2095758fa1`；
- Message：`chore: freeze APLS 0.1 implementation candidate`；
- 文件数：`249`；Commit 数：`1`；
- `07_src/target/` 未进入 Commit；未 Push、Tag、Branch、PR、Baseline Adoption、Formal C04、Release 或 Formal Seal。
