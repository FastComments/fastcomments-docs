## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackage200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getTenantPackage Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4b8c2a9f';
const packageId: string = 'pkg_7d3e1b5c';
const includeMetadata: boolean | undefined = true;
const packageResponse: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
[inline-code-end]

---