[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 `Vote`를 삭제할 수 있는 기능을 제공합니다.

[inline-code-attrs-start title = 'Vote 삭제 cURL 예제'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote 삭제 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote 삭제 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** 실패 시 포함됩니다. **/
    reason?: string
}
[inline-code-end]

Notes:

- 이 API는 테넌트 수준 설정을 따릅니다. 예를 들어 특정 페이지에서 투표를 비활성화한 상태에서 API를 통해 투표를 생성하려고 하면 오류 코드 `voting-disabled`로 실패합니다.
- 이 API는 기본적으로 라이브 상태입니다.
- 이 API는 해당 `Comment`의 `votes`를 업데이트합니다.