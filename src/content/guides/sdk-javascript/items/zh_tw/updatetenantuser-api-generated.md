## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| updateTenantUserBody | UpdateTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'updateTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b2a9d";
const id: string = "user_52c9f1ab";
const updateTenantUserBody: UpdateTenantUserBody = {
  email: "jane.doe@example.com",
  displayName: "Jane Doe",
  roles: ["moderator"],
  isActive: true,
  metadata: { signupSource: "sso", locale: "en-US" }
};
const updateComments: string = "Promoted to moderator and updated display name";
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]

---