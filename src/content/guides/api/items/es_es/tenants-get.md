[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

Esta API devuelve inquilinos que son gestionados por su inquilino.

La paginación se proporciona mediante el parámetro de consulta `skip`. Los inquilinos se devuelven en páginas de `100`, ordenados por `signUpDate` e `id`.

El costo se basa en el número de inquilinos devueltos, costando `1 crédito por cada 10` inquilinos devueltos.

[inline-code-attrs-start title = 'Ejemplo cURL de Tenant'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

Puede definir parámetros `meta` en los objetos `Tenant` y consultar inquilinos coincidentes. Por ejemplo, para la clave `someKey` y el valor meta `some-value`, podemos
construir un objeto JSON con este par clave/valor y luego codificarlo como URI como parámetro de consulta para filtrar:

[inline-code-attrs-start title = 'Ejemplo cURL de Consulta de Tenant por Meta'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of tenants to skip for pagination. **/
    skip?: number
    /** Filter by meta params. **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Tenant'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]
