[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Da me gusta a una página como el usuario actual. Cada usuario puede dar me gusta a una página una sola vez: volver a dar me gusta nuevamente tiene éxito con el código `already-liked` y no cambia el recuento.

La página se crea si aún no existe. Pase `title` para establecer o actualizar el título de la página.

[inline-code-attrs-start title = 'Ejemplo cURL de Me Gusta de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Solicitud de Me Gusta de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** Establece el título de la página. **/
    title?: string
    /** JSON codificado en URI de su objeto SSO. Omitir para usuarios anónimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Respuesta de Me Gusta de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' cuando el usuario ya había dado me gusta a la página. De lo contrario, se incluye en caso de error. **/
    code?: 'already-liked' | string
    /** Incluido en caso de error. **/
    reason?: string
}
[inline-code-end]