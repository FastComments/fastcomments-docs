[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Deze route retourneert maximaal 30 `Notification` objecten, gesorteerd op `createdAt`, nieuwste eerst.

U kunt filteren op `userId`. Bij SSO is de gebruikers-id in het formaat `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Ongelezen meldingen voor gebruiker cURL-voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van notificatieverzoek'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Pagineren door records over te slaan. **/
    skip?: number
    /** Filteren op gebruiker. **/
    userId?: string
    /** Filteren op urlId. **/
    urlId?: string
    /** Filteren op bronreactie. **/
    fromCommentId?: string
    /** Filteren op gelezen/ongelezen. **/
    viewed?: 'true' | 'false'
    /** Filteren op type. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het antwoord voor notificaties'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Wordt opgenomen bij een fout. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Wordt opgenomen bij een fout. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]