[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Esta ruta crea un solo `FeedPost`. Cada publicación tiene un autor, por lo que `fromUserId` es obligatorio y debe ser el id de un usuario FastComments o SSO existente en la cuenta.

[inline-code-attrs-start title = 'Ejemplo cURL de creación de FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud de creación de FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** Envía la publicación a los feeds que están abiertos en un navegador en este momento. Por defecto es false. **/
    isLive?: boolean
    /** Ejecuta la publicación a través del motor de spam antes de guardarla. Por defecto es false. **/
    doSpamCheck?: boolean
    /** Omite la verificación de contenido repetido que se ejecuta como parte de doSpamCheck. Por defecto es false. **/
    skipDupCheck?: boolean
    /** Hasta 256 caracteres. Repetido a los oyentes en vivo para que un cliente pueda ignorar su propia transmisión. **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** Obligatorio. Un id de usuario FastComments o SSO. **/
    fromUserId: string
    title?: string
    /** HTML. Sanitizado al guardar. **/
    contentHTML?: string
    /** Sobrescribe el nombre para mostrar tomado del usuario. **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de respuesta de creación de FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** Incluido en caso de error. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** Incluido en caso de error. **/
    reason?: string
    feedPost?: FeedPost; // Devolvemos la publicación completa creada en caso de éxito.
}
[inline-code-end]