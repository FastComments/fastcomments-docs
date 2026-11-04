[api-resource-header-start name = 'Page'; route = 'GET /api/v1/pages'; creditsCost = 10; api-resource-header-end]

현재는 귀하의 계정에 연결된 모든 페이지(또는 `/by-url-id`를 통해 단일 페이지)만 가져올 수 있습니다. 더 세밀한 검색이 필요하시면 [저희에게 문의해 주세요](https://fastcomments.com/auth/my-account/help).

[inline-code-attrs-start title = 'Pages cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Pages 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesRequestQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Pages 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagesResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 실패 시 포함됩니다. **/
    reason?: string
    pages: Page[]
}
[inline-code-end]

#### 유용한 팁

`Comment` API에는 `urlId`가 필요합니다. 먼저 `Pages` API를 호출하여 사용 가능한 `urlId` 값이 어떤 형태인지 확인할 수 있습니다.