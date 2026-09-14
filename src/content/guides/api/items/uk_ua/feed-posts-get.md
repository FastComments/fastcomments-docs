[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

Отримує пости у стрічці, спочатку найновіші. Пагінація базується на курсорі: передайте `_id` останнього отриманого поста як
`afterId`, щоб отримати наступну сторінку.

Вартує один кредит за кожні десять повернутих постів, мінімум один кредит.

[inline-code-attrs-start title = 'Приклад cURL для FeedPost'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запиту FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** Return posts after this post id. Omit for the first page. **/
    afterId?: string
    /** Defaults to 10. Maximum 1000. **/
    limit?: number
    /** Only return posts with these tags. Repeat the parameter for more than one tag. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура відповіді FeedPost'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** Included on failure. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]