## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetTenantPackages200Response.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getTenantPackages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_4f3a9c2d';
const skip: number = 25;
const packagesResponse: GetTenantPackages200Response = await getTenantPackages(tenantId);
const pagedPackagesResponse: GetTenantPackages200Response = await getTenantPackages(tenantId, skip);
[inline-code-end]

---