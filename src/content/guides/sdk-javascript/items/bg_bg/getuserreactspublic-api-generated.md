---
## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postIds | Array<string> | Не |  |
| sso | string | Не |  |

## Отговор

Връща: [`GetUserReactsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserReactsPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getUserReactsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-4f2b9c';
const postIds: Array<string> = ['post_7a1f9e', 'post_2b3c88'];
const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiI1Njc4OSIsImlhdCI6MTYwOTQ0MDAwMH0.signature';

const response: GetUserReactsPublic200Response = await getUserReactsPublic(tenantId, postIds, sso);
[inline-code-end]

---