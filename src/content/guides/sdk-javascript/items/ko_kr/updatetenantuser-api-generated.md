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
const tenantId: string = 'tenant_84f3b2';
const id: string = 'user_7a9d1c';
const updateComments: string = 'Promoted to moderator and updated contact email';
const updateTenantUserBody: UpdateTenantUserBody = {
  email: 'jane.doe+mod@example.com',
  displayName: 'Jane D.',
  roles: ['moderator'],
  isBanned: false,
  metadata: { department: 'community' }
};
const result: FlagCommentPublic200Response = await updateTenantUser(tenantId, id, updateTenantUserBody, updateComments);
[inline-code-end]

---