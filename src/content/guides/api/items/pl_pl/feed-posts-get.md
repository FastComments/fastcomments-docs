[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Pobiera posty w kanale, najnowsze jako pierwsze. Paginacja oparta jest na kursorze: przekaż `_id` ostatniego otrzymanego posta jako `afterId`, aby uzyskać następną stronę.

Kosztuje jeden kredyt za dziesięć zwróconych postów, z minimum jednym kredytem.

[inline-code-attrs-start title = 'Przykład cURL FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Zwróć posty po tym identyfikatorze posta. Pomiń dla pierwszej strony. **/
    afterId?: string
    /** Domyślnie 10. Maksymalnie 1000. **/
    limit?: number
    /** Zwróć tylko posty z tymi tagami. Powtórz parametr, aby podać więcej niż jeden tag. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Zawarte w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Zawarte w przypadku niepowodzenia. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]