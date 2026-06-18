## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetTenantUser200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUser200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTenantUser 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_ab12c3';
const id: string = 'user_9f8e7d';
const response: GetTenantUser200Response = await getTenantUser(tenantId, id);
console.log(response);
[inline-code-end]

---