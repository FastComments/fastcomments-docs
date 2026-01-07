[api-resource-header-start name = 'Moderator'; route = 'GET /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Esta API usa paginación, proporcionada por el parámetro de consulta `skip`. Los Moderators se devuelven en páginas de `100`, ordenados por `createdAt` e `id`.

El costo se basa en el número de moderadores devueltos, costando `1 crédito por cada 10` moderadores devueltos.

[inline-code-attrs-start title = 'Ejemplo cURL de Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** The number of moderators to skip for pagination. **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Moderator'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    moderators?: Moderator[]
}
[inline-code-end]
