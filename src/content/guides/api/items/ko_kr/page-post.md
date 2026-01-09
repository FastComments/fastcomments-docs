[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 페이지를 생성하는 기능을 제공합니다.

일반적인 사용 사례는 접근 제어입니다.

참고:

- 댓글 스레드에 댓글을 달았거나, `Comment`를 생성하는 API를 호출했다면, 이미 `Page` 객체를 생성한 것입니다! You can try fetching it via
  the `/by-url-id` `Page` route, passing in the same `urlId` passed to the comment widget.
- The `Page` structure contains some **calculated** values.
  Currently, these are `commentCount` and `rootCommentCount`.
  They are populated automatically and cannot be set by the API. Attempting to do so will cause the API to return an error.

[inline-code-attrs-start title = '페이지 POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"title": "Test Page",
	"url": "some0-url",
	"urlId": "page2",
	"accessibleByGroupIds": ["SOME_GROUP_ID"]
}'
[inline-code-end]

[inline-code-attrs-start title = '페이지 POST 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 POST 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 생성된 페이지. **/
    page?: Page
}
[inline-code-end]