[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

Devuelve los nombres de los usuarios que añadieron una reacción a una página, ordenados alfabéticamente. Se buscan hasta 100 reacciones, y los usuarios anónimos no se incluyen.

[inline-code-attrs-start title = 'Ejemplo cURL de usuarios de reacciones de página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud de usuarios de reacciones de página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** The reaction id. **/
    id: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de respuesta de usuarios de reacciones de página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: string
    /** Included on failure. **/
    reason?: string
    userNames: string[]
}
[inline-code-end]

---