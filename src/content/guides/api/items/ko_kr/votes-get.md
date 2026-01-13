[api-resource-header-start name = 'Vote'; route = 'GET /api/v1/votes'; creditsCost = 10; api-resource-header-end]

투표는 `urlId`로 가져와야 합니다.

### 투표 유형

투표에는 세 가지 유형이 있습니다:

- 인증된 투표(Authenticated Votes)는 해당 댓글에 적용됩니다. 이 API를 통해 생성할 수 있습니다.
- 인증된 투표(Authenticated Votes) 중 **검증 대기(pending)** 상태인 투표는 아직 댓글에 적용되지 않았습니다. 사용자가 FastComments.com의 *login to vote* 메커니즘을 사용할 때 생성됩니다.
- 익명 투표(Anonymous Votes)는 해당 댓글에 적용됩니다. 익명 댓글 작성과 함께 생성됩니다.

혼동을 줄이기 위해 API는 이를 별도의 목록으로 반환합니다.

[inline-code-attrs-start title = 'Votes cURL 예제'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=test'
[inline-code-end]

[inline-code-attrs-start title = 'Votes 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Votes 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotesResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 권한이 부여되고 검증된 투표로, 해당 댓글에 적용됩니다. **/
    appliedAuthorizedVotes: Vote[]
    /** 익명 투표로, 해당 댓글에 적용됩니다. **/
    appliedAnonymousVotes: Vote[]
    /** 검증 대기 중인 투표로, 아직 해당 댓글에 적용되지 않았습니다. **/
    pendingVotes: Vote[]
}
[inline-code-end]

#### 익명 투표 참고

이 API를 통해 생성된 익명 투표는 `appliedAuthorizedVotes` 목록에 나타납니다. API 키로 API를 통해 생성되었기 때문에 권한이 부여된 것으로 간주됩니다.

`appliedAnonymousVotes` 구조는 이메일, API 키 등이 없이 생성된 투표에 대한 것입니다.

---