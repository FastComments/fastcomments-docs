[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

Actualmente solo puede obtener todas las páginas (o una sola página vía `/by-url-id`) asociadas con su cuenta. Si desea búsqueda más detallada, [contáctenos](https://fastcomments.com/auth/my-account/help).

[inline-code-attrs-start title = 'Ejemplo cURL de Pages'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Pages'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** Included on failure. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### Consejo Útil

La API de `Comment` requiere un `urlId`. Puede llamar primero a la API de `Pages`, para ver cómo lucen los valores de `urlId` disponibles para usted.
