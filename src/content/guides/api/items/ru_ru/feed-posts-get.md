[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Получает посты в ленте, начиная с самых новых. Пагинация основана на курсоре: передайте `_id` последнего полученного поста в параметр `afterId`, чтобы получить следующую страницу.

Стоимость — один кредит за каждые десять возвращённых постов, минимум один кредит.

[inline-code-attrs-start title = 'Пример cURL для FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Возвращать посты после этого идентификатора поста. Пропустите для первой страницы. **/
    afterId?: string
    /** По умолчанию 10. Максимум 1000. **/
    limit?: number
    /** Возвращать только посты с этими тегами. Повторите параметр для более чем одного тега. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Включается при ошибке. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Включается при ошибке. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---