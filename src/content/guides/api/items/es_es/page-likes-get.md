[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Devuelve el número de likes en una página y si el usuario actual le ha dado like. Las páginas que aún no existen devuelven un `likeCount` de `0`.

[inline-code-attrs-start title = 'Ejemplo cURL de Page Likes'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud de Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta de Page Likes'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
}
[inline-code-end]