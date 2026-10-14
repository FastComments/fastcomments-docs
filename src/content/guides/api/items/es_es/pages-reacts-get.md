[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Devuelve el recuento de cada reacción en una página, y opcionalmente los usuarios que reaccionaron, como una entrada por reacción con su id de reacción, la más reciente primero. Esta es la versión del lado del servidor de [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` controla qué usuarios se devuelven:

- `public` omite a los usuarios que han hecho privada la actividad de su perfil.
- `include-private` los incluye. Úselo solo cuando la lista no se muestra al público.

Los usuarios anónimos nunca se incluyen, y se buscan hasta 100 reacciones. Las páginas que no existen devuelven sin `counts` y una lista `users` vacía.

[inline-code-attrs-start title = 'Ejemplo cURL de Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud de Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Omitir para obtener solo los recuentos. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta de Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Incluido en caso de error. **/
    reason?: string
    /** Recuento por id de reacción, por ejemplo {"heart": 12, "laugh": 3}. No se establece cuando la página no tiene reacciones. **/
    counts?: Record<string, number>
    /** Una entrada por reacción, la más reciente primero. Solo se establece cuando se pasa includeUserInfo. **/
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

#### Consejo útil

Recuerde codificar en URI los valores como `urlId`.