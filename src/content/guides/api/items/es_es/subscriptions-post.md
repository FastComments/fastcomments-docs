[api-resource-header-start name = 'Subscription'; route = 'POST /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Este endpoint de API proporciona la capacidad de crear una `Subscription`. Tenga en cuenta que un usuario solo puede tener una suscripción por página, ya que más es redundante, e intentar
crear más de una suscripción para el mismo usuario para la misma página resultará en un error.

Crear una suscripción resultará en objetos `Notification` siendo creados cuando se deje un nuevo comentario en la raíz del `urlId` suscrito (cuando `parentId` del comentario es `null`).

[inline-code-attrs-start title = 'Ejemplo cURL de POST de Subscription'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "userId": "tenantId:test-user-id",
    "urlId": "some-page-id-or-url",
    "url": "https://example.com/page",
    "pageTitle": "Some Example Page!"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Solicitud de POST de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de Respuesta de POST de Subscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface SubscriptionPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unexpected-param' | 'unauthorized' | 'not-found';
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
