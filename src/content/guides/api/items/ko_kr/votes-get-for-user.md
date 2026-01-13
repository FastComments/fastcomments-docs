[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes/for-user'; creditsCost = 1; api-resource-header-end]

지정된 `urlId`에 대해 사용자가 남긴 투표를 가져올 수 있습니다. `userId`는 FastComments.com의 사용자 또는 `SSO User`가 될 수 있습니다.

이는 사용자가 댓글에 투표했는지 보여주려는 경우에 유용합니다. 댓글을 가져올 때 동일한 `urlId`로 사용자에 대해 이 API를 함께 호출하면 됩니다.

익명 투표를 사용하는 경우 대신 `anonUserId`를 전달하세요.

[inline-code-attrs-start title = '사용자별 투표 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&userId=some-user-id'
[inline-code-end]

[inline-code-attrs-start title = '익명 사용자 투표 cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes/for-user?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test&anonUserId=some-user-id'
[inline-code-end]

익명 투표는 `appliedAuthorizedVotes` 목록에 표시됩니다. 이들은 API 키로 API를 통해 생성되었기 때문에 권한이 있는 것으로 간주됩니다.

[inline-code-attrs-start title = '사용자별 투표 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = '사용자별 투표 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesForUserResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'invalid-user-id'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 권한이 확인된(검증된) 투표로, 해당 댓글에 적용됩니다. **/
    appliedAuthorizedVotes: Vote[]
    /** 검증 대기 중인 투표로, 아직 해당 댓글에 적용되지 않았습니다. **/
    pendingVotes: Vote[]
}
[inline-code-end]