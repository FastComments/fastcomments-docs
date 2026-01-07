[api-resource-header-start name = 'TenantPackage'; route = 'GET /api/v1/tenant-packages'; creditsCost = 1; api-resource-header-end]

Esta API utiliza paginación, proporcionada por el parámetro de consulta `skip`. Los TenantPackages se devuelven en páginas de `100`, ordenados por `createdAt` e `id`.

El costo se basa en el número de paquetes de inquilino devueltos, costando `1 crédito por cada 10` paquetes de inquilino devueltos.

[inline-code-attrs-start title = 'Ejemplo cURL de TenantPackage'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-packages?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenant packages to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de TenantPackage'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantPackagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenantPackages?: TenantPackage[]
}
[inline-code-end]
