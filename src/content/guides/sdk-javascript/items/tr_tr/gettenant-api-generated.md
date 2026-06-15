## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetTenant200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenant200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getTenant Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fc_tenant_6b3e2a';
const id: string = 'site_42f1';
const tenantResponse: GetTenant200Response = await getTenant(tenantId, id);
const tenant: APITenant | undefined = tenantResponse.tenant;
const primaryDomain: APIDomainConfiguration | undefined = tenant?.domainConfiguration?.[0];
[inline-code-end]

---