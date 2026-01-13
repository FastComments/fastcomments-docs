[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Ta pot vrne do 30 `Notification` objektov, razvrščenih po `createdAt`, najnovejši prvi.

Lahko filtrirate po `userId`. Pri SSO je ID uporabnika v formatu `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Primer cURL: neprebrana obvestila uporabnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteve za obvestila'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Straničenje s preskočitvijo zapisov. **/
    skip?: number
    /** Filtriraj po uporabniku. **/
    userId?: string
    /** Filtriraj po urlId. **/
    urlId?: string
    /** Filtriraj po izvorni komentar. **/
    fromCommentId?: string
    /** Filtriraj po prebranem/neprebranem. **/
    viewed?: 'true' | 'false'
    /** Filtriraj po tipu. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za obvestila'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Vključeno ob napaki. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Vključeno ob napaki. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]

---