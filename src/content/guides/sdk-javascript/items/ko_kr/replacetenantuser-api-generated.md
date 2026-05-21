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