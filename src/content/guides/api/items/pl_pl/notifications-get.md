[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications'; creditsCost = 1; api-resource-header-end]

Ten endpoint zwraca do 30 obiektów `Notification` posortowanych według `createdAt`, od najnowszych.

Możesz filtrować po `userId`. Przy SSO identyfikator użytkownika ma format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Przykład cURL — nieprzeczytane powiadomienia użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania powiadomień'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Stronicowanie przez pominięcie rekordów. **/
    skip?: number
    /** Filtruj według użytkownika. **/
    userId?: string
    /** Filtruj według urlId. **/
    urlId?: string
    /** Filtruj według komentarza źródłowego. **/
    fromCommentId?: string
    /** Filtruj według stanu przeczytania. **/
    viewed?: 'true' | 'false'
    /** Filtruj według typu. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi powiadomień'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationsGetResponse {
    status: 'success' | 'failed'
    /** Zwracane w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Zwracane w przypadku niepowodzenia. **/
    reason?: string
    notifications?: Notification[]
}
[inline-code-end]