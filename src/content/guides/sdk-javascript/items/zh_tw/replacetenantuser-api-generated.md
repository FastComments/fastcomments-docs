## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| id | string | 是 |  |
| replaceTenantUserBody | ReplaceTenantUserBody | 是 |  |
| updateComments | string | 否 |  |

## 回應

回傳：[`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'replaceTenantUser 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_5f8b9a";
const id: string = "user_92bf21";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  email: "jane.doe@acme-corp.com",
  displayName: "Jane Doe",
  externalId: "acme|12345",
  roles: ["commenter", "moderator"],
  isActive: true,
  metadata: { team: "product", location: "NYC" }
};
const updateComments: string = "Update historical comments to reflect new display name";
const result: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---