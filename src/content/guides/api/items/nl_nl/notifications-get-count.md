[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Deze route retourneert een object met het aantal meldingen onder de parameter `count`.

Het is trager dan `/notification-count/` en kost twee keer zoveel credits, maar maakt filteren op meer dimensies mogelijk.

Je kunt filteren op dezelfde parameters als de `/notifications` endpoint, zoals `userId`. Bij SSO heeft de gebruikers-id het formaat `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Voorbeeld cURL: Aantal ongelezen meldingen voor gebruiker'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Voorbeeld cURL: Aantal ongelezen meldingen voor gebruiker op specifieke pagina'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van aanvraag voor meldingsaantal'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
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

[inline-code-attrs-start title = 'Structuur van antwoord voor meldingsaantal'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Opgenomen bij mislukking. **/
    reason?: string
    count?: number
}
[inline-code-end]