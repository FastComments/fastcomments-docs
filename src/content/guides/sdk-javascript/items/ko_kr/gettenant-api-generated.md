## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## 예제

[inline-code-attrs-start title = 'getTenant 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc_tenant_6b3e2a';
const id: string = 'site_42f1';
const tenantResponse: GetTenant200Response = await getTenant(tenantId, id);
const tenant: APITenant | undefined = tenantResponse.tenant;
const primaryDomain: APIDomainConfiguration | undefined = tenant?.domainConfiguration?.[0];
[inline-code-end]

---