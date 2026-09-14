[api-resource-header-start name = 'FeedPost'; route = 'PATCH /api/v1/feed-posts/:id'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 `FeedPost`를 업데이트합니다. 변경하려는 필드만 전송하세요.

[inline-code-attrs-start title = 'FeedPost 업데이트 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/feed-posts/some-post-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "title": "Release 2.0.1 is out",
    "tags": ["releases", "hotfix"]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 업데이트 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchQueryParams {
    tenantId: string
    API_KEY: string
}

/** FeedPost 구조에서 쓸 수 있는 모든 필드. **/
type FeedPostPatchBody = Partial<Pick<FeedPost, 'title' | 'contentHTML' | 'fromUserDisplayName' | 'tags' | 'weight' | 'meta' | 'media' | 'links'>>
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost 업데이트 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'internal'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]