---
[api-resource-header-start name = 'FeedPost'; route = 'GET /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

피드의 게시물을 최신 순으로 가져옵니다. 페이지네이션은 커서 기반이며, 마지막으로 받은 게시물의 `_id`를 `afterId`로 전달하여 다음 페이지를 가져옵니다.

반환된 10개의 게시물당 1크레딧이 소모되며, 최소 1크레딧이 차감됩니다.

[inline-code-attrs-start title = 'FeedPost cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&limit=10&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** 이 게시물 ID 이후의 게시물을 반환합니다. 첫 페이지의 경우 생략합니다. **/
    afterId?: string
    /** 기본값은 10이며, 최대값은 1000입니다. **/
    limit?: number
    /** 이 태그가 있는 게시물만 반환합니다. 여러 태그를 지정하려면 매개변수를 반복합니다. **/
    tags?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostsResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'limit-invalid' | 'internal'
    /** 실패 시 포함됩니다. **/
    reason?: string
    feedPosts: FeedPost[]
}
[inline-code-end]

---