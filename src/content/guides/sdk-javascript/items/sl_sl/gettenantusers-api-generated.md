## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| skip | number | Ne |  |

## Odgovor

Vrne: [`GetTenantUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantUsersResponse.ts)

## Primer

[inline-code-attrs-start title = 'getTenantUsers Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const skip: number = 20;

const firstPage: GetTenantUsersResponse = await getTenantUsers(tenantId);
const secondPage: GetTenantUsersResponse = await getTenantUsers(tenantId, skip);
[inline-code-end]