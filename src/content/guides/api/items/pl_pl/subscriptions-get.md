[api-resource-header-start name = 'Subscription'; route = 'GET /api/v1/subscriptions'; creditsCost = 1; api-resource-header-end]

Ta trasa zwraca do 30 obiektów `Subscription` posortowanych według `createdAt`, od najnowszych.

Możesz filtrować według `userId`. Przy SSO identyfikator użytkownika ma format `<tenant id>:<user id>`.

[inline-code-attrs-start title = 'Przykład cURL: subskrypcje użytkownika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/subscriptions?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=SOME_USER_ID'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania subskrypcji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetQueryParams {
    tenantId: string
    API_KEY: string
    /** Paginacja przez pomijanie rekordów. **/
    skip?: number
    /** Filtruj według użytkownika. **/
    userId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi subskrypcji'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SubscriptionsGetResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'unauthorized' | 'unexpected-param' | 'not-found'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    subscriptions?: Subscription[]
}
[inline-code-end]

---