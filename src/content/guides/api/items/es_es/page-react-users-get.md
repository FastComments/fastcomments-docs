[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Devuelve los nombres de los usuarios que añadieron una reacción a una página, ordenados alfabéticamente. Se buscan hasta 100 reacciones, y no se incluyen usuarios anónimos ni usuarios con actividad de perfil privado. En páginas que requieren SSO, los usuarios que no pueden ver la página reciben un `403` con el código `sso-required` o `page-forbidden`.

Para obtener también avatares e IDs de usuario, use [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) con `includeUserInfo=public`.

[inline-code-attrs-start title = 'Ejemplo cURL de Usuarios de Reacciones de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Usuarios de Reacciones de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** El id de la reacción. **/
    id: string
    /** JSON codificado en URI de su objeto SSO. Omitir para usuarios anónimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Usuarios de Reacciones de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: string
    /** Incluido en caso de error. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---