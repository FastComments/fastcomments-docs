[api-resource-header-start name = 'PollVote'; route = 'GET /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

하나의 설문 조사 카운트 뒤에 있는 개별 투표를 오래된 순서대로 나열합니다. 반환된 100표당 1크레딧이 소모됩니다.

설문 조사는 댓글에 속하므로 투표는 한 번에 하나의 설문 조사씩 읽으며 `commentId`가 필요합니다. `voterId`를 사용하여 특정 사용자의 투표를 확인하거나 `optionId`를 사용하여 특정 옵션을 선택한 모든 사용자를 나열할 수 있습니다.

한 호출당 최대 1000개의 투표가 반환됩니다. 더 많은 투표를 페이지네이션하려면 `skip`을 사용하세요.

설문 조사의 `privacy` 설정이 적용됩니다: 익명 설문 조사에 대한 투표는 읽을 수 없으며, 요청은 `poll-anonymous` 오류와 함께 실패합니다. 자세한 내용은 `PollVote` 구조를 참조하세요.

[inline-code-attrs-start title = 'PollVotes Get cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET&commentId=comment-id'
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetQueryParams {
    tenantId: string
    API_KEY: string
    commentId: string
    voterId?: string
    optionId?: string
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVotes Get 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVotesGetResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'poll-not-found' | 'poll-anonymous'
    /** 실패 시 포함됩니다. **/
    reason?: string
    pollVotes: PollVote[]
}
[inline-code-end]

### 옵션별 투표 수 계산

결과를 얻기 위해 직접 합산할 필요가 없습니다 - 설문 조사는 자체 카운트를 가지고 있습니다. 대신 `GET /api/v1/polls/:commentId` 로 설문 조사를 읽고, 누가 투표했는지 알아야 할 때 이 API를 사용하세요.

### 페이지의 모든 설문 조사

페이지 전체에 대한 투표 목록은 제공되지 않습니다. 전체 페이지를 보고하려면 `GET /api/v1/comments` 로 해당 페이지의 댓글을 가져오세요. 이 호출은 각 댓글의 설문 조사와 그 카운트를 반환하며, 관심 있는 설문 조사에 대한 투표를 읽을 수 있습니다.