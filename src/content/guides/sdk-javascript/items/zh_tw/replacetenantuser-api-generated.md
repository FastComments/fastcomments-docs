## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| replaceTenantUserBody | ReplaceTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'replaceTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acmeCorp";
const id: string = "user_84b2";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  email: "alice.jenkins@acmecorp.com",
  displayName: "Alice Jenkins",
  roles: ["moderator", "editor"],
  disabled: false
} as ReplaceTenantUserBody;
const updateComments: string = "Migrated user account and reattributed historical comments";

const result: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---