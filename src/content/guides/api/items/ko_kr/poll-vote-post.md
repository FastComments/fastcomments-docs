[api-resource-header-start name = 'PollVote'; route = 'POST /api/v1/poll-votes'; creditsCost = 1; api-resource-header-end]

설문에 대한 투표를 기록합니다.

투표자는 설문당 최대 하나의 투표만 할 수 있습니다. 동일한 투표자에 대해 이 API를 다시 호출하면 두 번째 투표를 추가하는 것이 아니라 기존 투표를 새로운
옵션으로 이동시킵니다, 그리고 이미 선택한 옵션에 다시 투표하면 아무 변화도 없습니다.

응답에 설문이 포함되어 있어 두 번째 요청 없이 업데이트된 투표 수를 확인할 수 있습니다.

[inline-code-attrs-start title = 'PollVote 생성 cURL 예시'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"userId": "user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = '익명 PollVote 생성 cURL 예시'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/poll-votes?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"commentId": "comment-id",
	"optionId": "the-option-id",
	"anonUserId": "some-randomly-generated-identifier",
	"ip": "203.0.113.4"
}'
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 생성 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollVoteCreateBody {
    commentId: string
    optionId: string
    /** One of userId or anonUserId is required. **/
    userId?: string
    anonUserId?: string
    /** The end user's IP, used for the anonymous rate limit. Defaults to the caller's IP. **/
    ip?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'PollVote 생성 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVoteCreateResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-comment-id' | 'missing-user-id' | 'invalid-user' | 'unauthorized' | 'poll-not-found' | 'poll-invalid-option' | 'poll-closed' | 'poll-login-required' | 'rate-limited'
    /** Included on failure. **/
    reason?: string
    pollVote: PollVote
    /** The poll with its updated counts. **/
    poll: CommentPoll
}
[inline-code-end]

### 익명 투표

`anonUserId`를 `userId` 대신 설정하여 로그인하지 않은 사용자의 투표를 기록합니다. 해당 ID는 어느 곳의 사용자와도 일치할 필요가
없으며 - 세션을 식별할 뿐이므로 같은 사람이 두 번 계산되지 않습니다.

사이트에서 익명 투표를 활성화해야 합니다. 투표가 로그인한 사용자에게만 제한된 경우, `anonUserId`만 포함된 투표는 `poll-login-required`
오류로 실패합니다.

익명 투표는 설문당 IP별로 속도 제한이 적용되어, 한 사람이 세션을 초기화하여 설문을 채우는 것을 방지합니다. 최종 사용자의 `ip`를 전송하면 제한이 서버가 아니라 해당 사용자에게 적용됩니다.

### 기타 참고 사항

- `userId`는 사이트에 존재하는 사용자여야 합니다. 다른 사이트에 속한 사용자를 위한 투표는 거부됩니다.
- 닫힌 설문에 대한 투표는 `poll-closed` 오류로 실패합니다.
- 이 API는 설문의 투표 수를 업데이트하고 연결된 위젯에 실시간으로 푸시합니다.

---