# HDP-APLS-023 — GitHub 私有同步与 Kimi 交接授权

```yaml
determination_id: HDP-APLS-023
status: APPROVED
selected_option: DIRECT_USER_AUTHORIZATION
decided_by: HUMAN_PROJECT_OWNER
decision_date: 2026-09-04
authorization_text: "你先同步到GitHub上吧，私有仓库，然后告诉我地址，我让kimi接手你接下来的工作，给我写个交接指令"
```

## 1. 授权范围

项目负责人明确授权 C00：

1. 在 GitHub 账号 `loocoo2025` 下创建 APLS 私有仓库；
2. 将本地 `main` 的当前项目内容提交并推送到该私有仓库；
3. 形成供 Kimi 接管逻辑 C00 后续工作的正式 HANDOFF 和可复制启动指令；
4. 只为完成同步与交接而更新 Current Truth、Task、Decision、Conversation Map 和执行回执。

远程仓库已经创建并通过 GitHub 页面确认：

```yaml
repository: https://github.com/loocoo2025/apls-language
visibility: PRIVATE
default_branch: main
remote_name: origin
transport: SSH
```

## 2. 独立动作授权

```yaml
authorization_id: AUTH-APLS-C00-023-REMOTE
authority_owner: HUMAN_PROJECT_OWNER
action: CREATE_PRIVATE_GITHUB_REPOSITORY
scope: GITHUB_ACCOUNT_LOOCOO2025
target: loocoo2025/apls-language
allowed_side_effects: [private_repository_creation, repository_description]
forbidden_side_effects: [public_visibility, collaborator_invitation, release, tag, baseline_adoption, formal_c04, formal_seal]
validity: TASK023_ONLY
consumption_event: PRIVATE_REPOSITORY_CREATED
terminal_state: REPOSITORY_CREATED_OR_FAILED
retry_policy: NO_AUTOMATIC_RETRY
escalation: HUMAN_PROJECT_OWNER_ON_NAME_OR_VISIBILITY_CONFLICT
status: EXECUTION_COMPLETED
audit_reference: HUMAN_DIRECT_REQUEST_AND_GITHUB_PRIVATE_BADGE
```

```yaml
authorization_id: AUTH-APLS-C00-023-COMMIT
authority_owner: HUMAN_PROJECT_OWNER
action: COMMIT
scope: GOVERNANCE_RECEIPTS_CURRENT_TRUTH_AND_KIMI_HANDOFF_ONLY
target: LOCAL_MAIN
allowed_side_effects: [git_index_write, minimal_local_commits]
forbidden_side_effects: [compiler_source_change, test_change, design_change, dependency_change, history_rewrite]
validity: UNTIL_TASK023_REMOTE_SYNC_COMPLETED
consumption_event: EACH_TASK023_GOVERNANCE_COMMIT
terminal_state: HANDOFF_SNAPSHOT_COMMITTED_OR_FAILED
retry_policy: RETURN_TO_C00_ON_FAILURE
escalation: HUMAN_PROJECT_OWNER_ON_SCOPE_CHANGE
status: ISSUED_UNUSED
audit_reference: HUMAN_DIRECT_REQUEST
```

```yaml
authorization_id: AUTH-APLS-C00-023-PUSH
authority_owner: HUMAN_PROJECT_OWNER
action: PUSH
scope: TASK023_HANDOFF_SNAPSHOT_AND_COMPLETION_RECEIPT
target: git@github.com:loocoo2025/apls-language.git refs/heads/main
allowed_side_effects: [add_origin_remote, push_local_main, set_upstream, verify_remote_ref]
forbidden_side_effects: [force_push, tag, release, pull_request, visibility_change, collaborator_invitation]
validity: UNTIL_TASK023_REMOTE_SYNC_COMPLETED
consumption_event: EACH_REQUIRED_NON_FORCE_PUSH_FOR_TASK023
terminal_state: REMOTE_MAIN_SYNCHRONIZED_OR_FAILED
retry_policy: RECONCILE_REMOTE_BEFORE_ANY_RETRY
escalation: HUMAN_PROJECT_OWNER_ON_NON_FAST_FORWARD_OR_REMOTE_CONFLICT
status: ISSUED_UNUSED
audit_reference: HUMAN_DIRECT_REQUEST
```

## 3. 明确不授权

- 不授权公开仓库、邀请协作者或向 Kimi 账号授予 GitHub 权限；
- 不授权 Formal C04、Baseline Adoption、Release 或 Formal Seal；
- 不授权修改 Compiler/CLI Source、测试、语言规范、IR Schema、依赖或产品行为；
- 本次交接不构成 `HDP-APLS-022` 的批准。

## 4. 结果

`PRIVATE_REPOSITORY_CREATED_HANDOFF_COMMIT_AND_PUSH_IN_PROGRESS`
