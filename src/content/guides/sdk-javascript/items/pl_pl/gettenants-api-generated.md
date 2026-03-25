## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| meta | string | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetTenants200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenants200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenants'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8421e7';
const meta: string = 'include=domains,billing,customConfig';
const skip: number = 20;

const tenantsBasic: GetTenants200Response = await getTenants(tenantId);
const tenantsWithOptions: GetTenants200Response = await getTenants(tenantId, meta, skip);
[inline-code-end]

---