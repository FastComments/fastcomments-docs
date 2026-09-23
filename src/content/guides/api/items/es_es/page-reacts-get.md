[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Devuelve el recuento de cada reacción en una página, y qué reacciones ha añadido el usuario actual.

[inline-code-attrs-start title = 'Ejemplo cURL de Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la solicitud de Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** JSON codificado en URI de su objeto SSO. Omitir para usuarios anónimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la respuesta de Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: string
    /** Incluido en caso de error. **/
    reason?: string
    /** Recuento por id de reacción, por ejemplo {"heart": 12, "laugh": 3}. No se establece cuando la página no tiene reacciones. **/
    counts?: Record<string, number>
    /** Los ids de reacción que el usuario que realiza la solicitud ha añadido. No se establece cuando no ha añadido ninguno. **/
    reactedIds?: string[]
}
[inline-code-end]

---