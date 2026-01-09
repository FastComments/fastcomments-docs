---
[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Ovaj endpoint vraća objekat koji sadrži broj notifikacija u parametru `count`.

Sporiji je od `/notification-count/` i dvostruko je skuplji po cenama kredita, ali omogućava filtriranje po više dimenzija.

Možete filtrirati istim parametrima kao i endpoint `/notifications`, kao što je `userId`. Kod SSO-a, `userId` je u formatu `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'cURL primer: broj nepročitanih notifikacija za korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'cURL primer: broj nepročitanih notifikacija korisnika za određenu stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za broj notifikacija'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Struktura odgovora za broj notifikacija'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Uključeno pri neuspehu. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Uključeno pri neuspehu. **/
    reason?: string
    count?: number
}
[inline-code-end]

---