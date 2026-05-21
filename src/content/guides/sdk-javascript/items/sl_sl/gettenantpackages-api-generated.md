## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vrne: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Primer

[inline-code-attrs-start title = 'getTenantPackages Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-7b3c2f';
const skipCount: number = 10;
const packages: GetTenantPackages200Response = await getTenantPackages(tenantId, skipCount);
const packagesFromStart: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]