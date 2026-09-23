[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Agrega una reacción a una página como el usuario actual. Un usuario puede agregar cada id de reacción una sola vez: volver a agregarla tiene éxito con el código `already-reacted` y no cambia el recuento. Un usuario puede agregar varias reacciones diferentes a la misma página.

Los ids de reacción los elige usted y pueden tener hasta 36 caracteres. La página se crea si aún no existe. Pase `title` para establecer o actualizar el título de la página.

[inline-code-attrs-start title = 'Ejemplo cURL de Reacción de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de Reacción de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** El id de la reacción, hasta 36 caracteres. **/
    id: string
    /** Establece el título de la página. **/
    title?: string
    /** JSON codificado en URI de su objeto SSO. Omitir para usuarios anónimos. **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de Reacción de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' cuando el usuario ya había añadido esta reacción. 'react-id-too-long' (HTTP 422) cuando el id supera los 36 caracteres. De lo contrario incluido en caso de error. **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** Incluido en caso de error. **/
    reason?: string
}
[inline-code-end]