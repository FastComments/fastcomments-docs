[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Devuelve el recuento de cada reacción en una página y qué reacciones ha añadido el usuario actual.

Pase `includeUserInfo=public` para también obtener los usuarios que reaccionaron, como una entrada por reacción con su id de reacción, más reciente primero. Los usuarios con actividad de perfil privado y los usuarios anónimos se omiten, y se buscan hasta 100 reacciones. Sólo se acepta `public`; `include-private` devuelve un `422` y está disponible desde [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) con una clave API. En páginas que requieren SSO, `includeUserInfo` devuelve un `403` para los usuarios que no pueden ver la página.

[inline-code-attrs-start title = 'Ejemplo cURL de Reacciones de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Solicitud de Reacciones de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON codificado en URI de su objeto SSO. Omitir para usuarios anónimos. **/
    sso?: string
    /** Establecer a 'public' para incluir a los usuarios que reaccionaron. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Respuesta de Reacciones de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Incluido en caso de error. **/
    reason?: string
    /** Recuento por id de reacción, por ejemplo {"heart": 12, "laugh": 3}. No se establece cuando la página no tiene reacciones. **/
    counts?: Record<string, number>
    /** Los ids de reacción que el usuario que realiza la solicitud ha añadido. No se establece cuando no ha añadido ninguno. **/
    reactedIds?: string[]
    /** Una entrada por reacción, más reciente primero. Sólo se establece cuando se pasa includeUserInfo. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** El id de reacción, por ejemplo "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]