## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3a9c2d';
const skip: number = 25;
const packagesResponse: GetTenantPackages200Response = await getTenantPackages(tenantId);
const pagedPackagesResponse: GetTenantPackages200Response = await getTenantPackages(tenantId, skip);
[inline-code-end]

---