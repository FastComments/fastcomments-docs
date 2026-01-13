[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Ova ruta vraća objekt koji sadrži broj obavijesti u parametru `count`.

Sporija je od `/notification-count/` i košta dvostruko više kredita, ali omogućava filtriranje po više dimenzija.

Možete filtrirati istim parametrima kao i endpoint `/notifications`, kao što je `userId`. S SSO-om, identifikator korisnika ima format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: Broj nepročitanih obavijesti za korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Primjer cURL zahtjeva: Broj nepročitanih obavijesti za korisnika za određenu stranicu'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za broj obavijesti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filtriraj po korisniku. **/
    userId?: string
    /** Filtriraj po urlId-u. **/
    urlId?: string
    /** Filtriraj po izvoru komentara. **/
    fromCommentId?: string
    /** Filtriraj po pročitano/nepročitano. **/
    viewed?: 'true' | 'false'
    /** Filtriraj po vrsti. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za broj obavijesti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    count?: number
}
[inline-code-end]