[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Ovaj endpoint vraća objekat koji sadrži broj obavještenja u parametru `count`.

Ova ruta je sporija od `/notification-count/` i košta dvostruko više kredita, ali omogućava filtriranje po više dimenzija.

Možete filtrirati istim parametrima kao i endpoint `/notifications`, npr. `userId`. Kod SSO, user id ima format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za broj nepročitanih obavještenja korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za broj nepročitanih obavještenja korisnika za određenu stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za broj obavještenja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filtriraj po korisniku. **/
    userId?: string
    /** Filtriraj po urlId. **/
    urlId?: string
    /** Filtriraj po izvornom komentaru. **/
    fromCommentId?: string
    /** Filtriraj po pročitanom/nepročitanom. **/
    viewed?: 'true' | 'false'
    /** Filtriraj po tipu. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za broj obavještenja'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Uključeno pri grešci. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Uključeno pri grešci. **/
    reason?: string
    count?: number
}
[inline-code-end]