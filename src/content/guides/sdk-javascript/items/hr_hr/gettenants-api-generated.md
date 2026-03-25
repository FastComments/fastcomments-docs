## Parametri

| Naziv | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| meta | string | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Primjer

[inline-code-attrs-start title = 'getTenants Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8421e7';
const meta: string = 'include=domains,billing,customConfig';
const skip: number = 20;

const tenantsBasic: GetTenants200Response = await getTenants(tenantId);
const tenantsWithOptions: GetTenants200Response = await getTenants(tenantId, meta, skip);
[inline-code-end]

---