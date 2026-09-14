[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Obtiene publicaciones en un feed, de más reciente a más antiguo. La paginación se basa en cursor: pase el `_id` de la última publicación que recibió como
`afterId` para obtener la página siguiente.

Cuesta un crédito por cada diez publicaciones devueltas, con un mínimo de un crédito.

[inline-code-attrs-start title = 'Ejemplo cURL de FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Devuelve publicaciones después de este id de publicación. Omitir para la primera página. **/
    afterId?: string
    /** Predeterminado a 10. Máximo 1000. **/
    limit?: number
    /** Solo devuelve publicaciones con estas etiquetas. Repita el parámetro para más de una etiqueta. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Incluido en caso de error. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---