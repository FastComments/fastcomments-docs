[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Ta ruta vrača objekt, ki vsebuje število obvestil v parametru `count`.

Je počasnejša od `/notification-count/` in stane dvakrat več kreditov, vendar omogoča filtriranje po več dimenzijah.

Filtrirate lahko z istimi parametri kot pri končni točki `/notifications`, na primer `userId`. Pri SSO je ID uporabnika v formatu `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Primer cURL zahteve za število neprebranih obvestil uporabnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Primer cURL zahteve za število neprebranih obvestil uporabnika za določeno stran'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za število obvestil'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filtriraj po uporabniku. **/
    userId?: string
    /** Filtriraj po urlId. **/
    urlId?: string
    /** Filtriraj po izvirnem komentarju. **/
    fromCommentId?: string
    /** Filtriraj po prebrano/neprebrano. **/
    viewed?: 'true' | 'false'
    /** Filtriraj po tipu. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za število obvestil'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Vključeno v primeru napake. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Vključeno v primeru napake. **/
    reason?: string
    count?: number
}
[inline-code-end]