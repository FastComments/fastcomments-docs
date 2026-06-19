## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| skip | number | Hayır |  |

## Yanıt

Döndürür: [`GetTenantPackagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackagesResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getTenantPackages Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-9f3b";
const packagesPage1: GetTenantPackagesResponse = await getTenantPackages(tenantId);
const packagesPage2: GetTenantPackagesResponse = await getTenantPackages(tenantId, 10);
[inline-code-end]

---