## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| replaceTenantUserBody | ReplaceTenantUserBody | 예 |  |
| updateComments | string | 아니요 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'replaceTenantUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "f3b9a2d1-8b4e-4c6a-9f2b-1d5c4e6a7b8c";
const id: string = "user_92f7c3b1";
const replaceTenantUserBody: ReplaceTenantUserBody = {
  externalId: "auth0|1234567890",
  email: "jane.doe@company.com",
  displayName: "Jane Doe",
  roles: ["moderator"],
  metadata: { department: "support" }
};
const updateComments: string = "reassign-comments-to-new-user";
const response: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]