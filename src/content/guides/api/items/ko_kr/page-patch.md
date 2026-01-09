[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 `Page`를 업데이트할 수 있는 기능을 제공합니다. 해당 댓글들이 업데이트됩니다.

[inline-code-attrs-start title = '페이지 업데이트 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = '페이지 업데이트 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 업데이트 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** 실패 시 포함됩니다. **/
    reason?: string
    user?: Page; // 성공 시 전체 업데이트된 페이지를 반환합니다.
}
[inline-code-end]

#### 참고

Page 객체의 일부 매개변수는 자동으로 업데이트됩니다. 이러한 항목에는 카운트와 title 속성이 포함됩니다. 카운트는 계산된 값이므로
API를 통해 업데이트할 수 없습니다. 페이지 `title`은 API를 통해 설정할 수 있지만, 동일한 `urlId`를 가진 페이지에서 댓글 위젯을 사용하고 페이지 제목이 다를 경우 덮어써질 수 있습니다.