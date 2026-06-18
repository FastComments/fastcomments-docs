## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Primjer

[inline-code-attrs-start title = 'getTenantPackages Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8421';
const packagesWithSkip: GetTenantPackages200Response = await getTenantPackages(tenantId, 25);
const packagesWithoutSkip: GetTenantPackages200Response = await getTenantPackages(tenantId);
[inline-code-end]

---