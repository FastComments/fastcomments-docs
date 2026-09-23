[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Elimina el like del usuario actual de una página. Si el usuario no ha dado like a la página, la solicitud tiene éxito con el código `not-liked` y no cambia el recuento.

[inline-code-attrs-start title = 'Ejemplo cURL de Quitar Like de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Solicitud de Quitar Like de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Respuesta de Quitar Like de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' when the user had not liked the page. Otherwise included on failure. **/
    code?: 'not-liked' | string
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]