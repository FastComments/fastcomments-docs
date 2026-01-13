---
 [api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

이 경로는 단일 권한 있는 `Vote`를 추가할 수 있는 기능을 제공합니다. 투표는 `up` (+1) 또는 `down` (-1)일 수 있습니다.

[inline-code-attrs-start title = '투표 생성 cURL 예제'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '익명 투표 생성 cURL 예제'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '투표 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = '투표 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** 실패 시 포함됩니다. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### 익명 투표 생성

익명 투표는 쿼리 매개변수에서 `userId` 대신 `anonUserId`를 설정하여 생성할 수 있습니다.

이 id는 어느 곳의 사용자 객체와 일치할 필요가 없습니다(따라서 익명입니다). 이것은 단순히 식별자
세션을 위한 것이며, 같은 세션에서 다시 투표를 가져와 댓글에
투표가 되었는지 확인할 수 있습니다.

If you do not have such a thing as "anonymous sessions" like FastComments does - you can simply
set this to a random ID, like a UUID (although we appreciate smaller identifiers to save space).

### 기타 참고

- 이 API는 테넌트 수준 설정을 따릅니다. 예를 들어 특정 페이지에 대해 투표를 비활성화하고 API를 통해 투표를 생성하려고 하면, `voting-disabled` 오류 코드로 실패합니다.
- 이 API는 기본적으로 라이브 상태입니다.
- 이 API는 해당 `Comment`의 `votes`를 업데이트합니다.

---