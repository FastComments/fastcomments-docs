## 매개변수

| 이름 | 형식 | 필수 | 설명 |
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