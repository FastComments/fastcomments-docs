## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| meta | string | Nee |  |
| skip | number | Nee |  |

## Respons

Retourneert: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getTenants Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8421e7';
const meta: string = 'include=domains,billing,customConfig';
const skip: number = 20;

const tenantsBasic: GetTenants200Response = await getTenants(tenantId);
const tenantsWithOptions: GetTenants200Response = await getTenants(tenantId, meta, skip);
[inline-code-end]

---