[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de crear páginas.

Un caso de uso común es el control de acceso.

Notas:

- Si ha comentado en un hilo de comentarios, o llamado a la API para crear un `Comment`, ¡ya ha creado un objeto `Page`! Puede intentar obtenerlo vía
  la ruta de `Page` `/by-url-id`, pasando el mismo `urlId` pasado al widget de comentarios.
- La estructura de `Page` contiene algunos valores **calculados**.
  Actualmente, estos son `commentCount` y `rootCommentCount`.
  Se llenan automáticamente y no pueden ser establecidos por la API. Intentar hacerlo causará que la API devuelva un error.

[inline-code-attrs-start title = 'Ejemplo cURL de POST de Page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de POST de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de POST de Page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';
    /** Included on failure. **/
    reason?: string
    /** The created page. **/
    page?: Page
}
[inline-code-end]
