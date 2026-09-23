[api-resource-header-start name = 'Page Reacts'; route = 'DELETE /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Elimina una de las reacciones del usuario actual de una página. Si el usuario no ha añadido esa reacción, la solicitud tiene éxito con el código `no-react` y no cambia el recuento.

[inline-code-attrs-start title = 'Ejemplo cURL de eliminación de reacción de página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud de eliminación de reacción de página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteRequestQueryParams {
    urlId: string
    /** El id de la reacción. **/
    id: string
    /** JSON codificado en URI de su objeto SSO. Omitir para usuarios anónimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de respuesta de eliminación de reacción de página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactDeleteResponse {
    status: 'success' | 'failed'
    /** 'no-react' cuando el usuario no había añadido esta reacción. De lo contrario, incluido en caso de error. **/
    code?: 'no-react' | string
    /** Incluido en caso de error. **/
    reason?: string
}
[inline-code-end]

---