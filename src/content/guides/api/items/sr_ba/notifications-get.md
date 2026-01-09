[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

This route returns up to 30 `Notification` objects sorted by `createdAt`, newest first.

You can filter by `userId`. With SSO, the user id is in the format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za nepročitane obavijesti korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za obavijesti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginacija preskakanjem zapisa. **/
    skip?: number
    /** Filtriranje po korisniku. **/
    userId?: string
    /** Filtriranje po urlId. **/
    urlId?: string
    /** Filtriranje po izvornom komentaru. **/
    fromCommentId?: string
    /** Filtriranje po pročitanom/nepročitanom. **/
    viewed?: 'true' | 'false'
    /** Filtriranje po tipu. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za obavijesti'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---