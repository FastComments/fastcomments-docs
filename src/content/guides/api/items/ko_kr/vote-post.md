 [api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

이 라우트는 단일 인증된 `Vote`를 추가하는 기능을 제공합니다. 투표는 `up`(+1) 또는 `down`(-1)일 수 있습니다.

[inline-code-attrs-start title = 'Vote 생성 cURL 예제'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = '익명 Vote 생성 cURL 예제'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Vote 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

### 익명 투표 생성하기

쿼리 파라미터에서 `userId` 대신 `anonUserId`를 설정하여 익명 투표를 생성할 수 있습니다.

이 ID는 어디에도 사용자 객체와 연결될 필요가 없습니다(익명이므로). 단순히 세션을 위한 식별자로,
같은 세션에서 투표를 다시 조회하여 댓글에 투표했는지 확인할 수 있습니다.

FastComments처럼 "익명 세션"과 같은 개념이 없다면, UUID와 같은 무작위 ID를
설정하면 됩니다(공간 절약을 위해 더 짧은 식별자를 권장합니다).

### 기타 참고 사항

- 이 API는 테넌트 수준 설정을 따릅니다. 예를 들어, 특정 페이지에 대해 투표를 비활성화한 상태에서 API를 통해 투표를 생성하려고 하면 `voting-disabled` 오류 코드와 함께 실패합니다.
- 이 API는 기본적으로 실시간입니다.
- 이 API는 해당 `Comment`의 `votes`를 업데이트합니다.