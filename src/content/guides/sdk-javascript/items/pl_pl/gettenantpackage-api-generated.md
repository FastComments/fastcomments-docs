## Parametry

| Name | Type | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| id | string | Tak |  |

## Response

Zwraca: [`GetTenantPackage200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackage200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b2c8';
const packageId: string = 'pkg_standard_2026';
const requestOptions: { includeConfig?: boolean } = { includeConfig: true };
const packageResponse: GetTenantPackage200Response = await getTenantPackage(tenantId, packageId);
[inline-code-end]

---