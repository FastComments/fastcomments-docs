## 매개변수

| 이름 | 유형 | 필수 | 설명 |
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
const tenantId: string = 'tenant_9d8f4b2c';
const id: string = 'user_f47ac10b';
const replaceTenantUserBody: ReplaceTenantUserBody = {
  externalId: 'ext-5234',
  email: 'jane.doe@acme.com',
  displayName: 'Jane Doe',
  roles: ['moderator'],
  metadata: { department: 'product', region: 'us-east-1' }
};
const updateComments: string = 'propagate-display-name-to-comments';

const result: FlagCommentPublic200Response = await replaceTenantUser(tenantId, id, replaceTenantUserBody, updateComments);
[inline-code-end]

---