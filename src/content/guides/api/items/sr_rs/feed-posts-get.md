[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Преузима постове у фиду, најновије прво. Пагинација је заснована на курсору: проследите `_id` последњег поста који сте добили као `afterId` да бисте добили следећу страницу.

Кошт је један кредит по десет постова који се враћају, са минимумом од једног кредита.

[inline-code-attrs-start title = 'FeedPost cURL пример'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost структура захтева'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Враћа постове након овог ID поста. Изоставите за прву страницу. **/
    afterId?: string
    /** Подразумевано је 10. Максимум је 1000. **/
    limit?: number
    /** Враћа само постове са овим ознакама. Поновите параметар за више од једне ознаке. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost структура одговора'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Укључено у случају неуспеха. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Укључено у случају неуспеха. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---