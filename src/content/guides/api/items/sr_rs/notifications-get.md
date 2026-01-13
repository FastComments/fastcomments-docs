[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Ova ruta vraća do 30 `Notification` objekata sortirano po `createdAt`, najnoviji prvi.

Možete filtrirati po `userId`. Sa SSO, korisnički id je u formatu `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Primer cURL za nepročitane notifikacije korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za notifikacije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginacija preskakanjem zapisa. **/
    skip?: number
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

[inline-code-attrs-start title = 'Struktura odgovora za notifikacije'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Uključeno u slučaju greške. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]