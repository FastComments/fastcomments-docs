[api-resource-header-start name = 'Notification'; route = 'GET /api/v1/notifications/count'; creditsCost = 2; api-resource-header-end]

Ta trasa zwraca obiekt zawierający liczbę powiadomień w parametrze `count`.

Jest wolniejsza niż `/notification-count/` i kosztuje dwukrotnie więcej kredytów, ale umożliwia filtrowanie po większej liczbie wymiarów.

Możesz filtrować za pomocą tych samych parametrów co endpoint `/notifications`, takich jak `userId`. Przy SSO identyfikator użytkownika ma format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Przykład cURL: liczba nieprzeczytanych powiadomień dla użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false'
[inline-code-end]

[inline-code-attrs-start title = 'Przykład cURL: liczba nieprzeczytanych powiadomień dla użytkownika dla określonej strony'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/notifications/count?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID&viewed=false&urlId=some-article-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania liczby powiadomień'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Filtruj według użytkownika. **/
    userId?: string
    /** Filtruj według urlId. **/
    urlId?: string
    /** Filtruj według komentarza źródłowego. **/
    fromCommentId?: string
    /** Filtruj według stanu (odczytane/nieodczytane). **/
    viewed?: 'true' | 'false'
    /** Filtruj według typu. **/
    type?: NotificationType
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi liczby powiadomień'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface NotificationCountGetResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    count?: number
}
[inline-code-end]