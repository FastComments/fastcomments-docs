## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| updateTenantUserBody | UpdateTenantUserBody | 예 |  |
| updateComments | string | 아니요 |  |

## 응답

반환: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 예제

[inline-code-attrs-start title = 'updateTenantUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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