[api-resource-header-start name = 'Page'; route = 'POST /api/v1/pages'; creditsCost = 1; api-resource-header-end]

이 API 엔드포인트는 페이지를 생성하는 기능을 제공합니다.

일반적인 사용 사례는 접근 제어입니다.

참고 사항:

- 댓글 스레드에 댓글을 작성했거나 `Comment`를 생성하는 API를 호출했다면, 이미 `Page` 객체를 생성한 것입니다! 댓글 위젯에 전달한 것과 동일한 `urlId`를 사용하여 `/by-url-id` `Page` 라우트를 통해 해당 객체를 가져올 수 있습니다.
- `Page` 구조에는 일부 **계산된** 값이 포함되어 있습니다.
  현재 이 값들은 `commentCount`와 `rootCommentCount`입니다.
  이 값들은 자동으로 채워지며 API를 통해 설정할 수 없습니다. 설정을 시도하면 API가 오류를 반환합니다.

[inline-code-attrs-start title = 'Page POST cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Page POST 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page POST 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface PagePostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large';  
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 생성된 페이지입니다. **/
    page?: Page
}
[inline-code-end]