[api-resource-header-start name = 'Poll'; route = 'PUT /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

기존 댓글에 설문을 연결하거나 이미 존재하는 설문의 전체 상태를 설정합니다.

본문은 전체 설문이며, 전송한 옵션은 해당 순서대로 설문의 옵션이 됩니다. 각 옵션은 `id` 로 매칭됩니다:

- 기존 옵션의 `id` 를 포함하여 전송된 옵션은 해당 옵션과 투표를 유지합니다. 라벨과 위치는 전송한 내용으로 업데이트됩니다.
- `id` 없이 전송된 옵션은 새로 추가되며, 투표는 없습니다.
- 기존 옵션을 제외하면 해당 옵션과 그에 대한 투표가 모두 제거됩니다. `totalVotes` 도 동일하게 감소합니다.

따라서 옵션을 추가하려면 현재 옵션들을 `id`와 함께 전송하고 새 옵션은 `id` 없이 전송합니다. 옵션을 제거하려면 해당 옵션을 제외한 목록을 전송합니다. 옵션 `id` 는 `GET /api/v1/polls/:commentId` 로 반환된 설문에 포함됩니다.

모든 `id` 를 전송하지 않으면 모든 옵션이 교체되고 설문에 이미 존재하던 모든 투표가 삭제됩니다. 설문에 투표가 있는 경우 `replaceVotes=true` 가 필요하며, 이를 지정하지 않으면 API는 `replace-votes-required` 로 응답합니다.

다른 필드도 동일하게 교체됩니다: `closesAt`, `privacy`, `requireVoteToSeeResults` 를 생략하면 기본값으로 재설정됩니다. 하나의 필드만 변경하고 나머지는 유지하려면 `PATCH /api/v1/polls/:commentId` 를 사용하세요.

[inline-code-attrs-start title = 'Poll Put cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PUT \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [
		{"id": "existing-option-id", "label": "The bugfix release"},
		{"label": "The feature release"}
	],
	"closesAt": "2026-12-31T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Put 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutQueryParams {
    tenantId: string
    API_KEY: string
    /** 설문에 투표가 있는 경우 기존 옵션 ID를 모두 유지하려면 필요합니다. 그렇지 않으면 모두 삭제됩니다. **/
    replaceVotes?: boolean
}

interface PollPutOption {
    /** 기존 옵션의 ID이며, 해당 옵션과 투표를 유지합니다. 새 옵션을 추가하려면 생략하세요. **/
    id?: string | null
    label: string
}

interface PollPutBody {
    question: string
    /** 전체 순서가 지정된 목록입니다. 제외된 기존 옵션은 해당 투표와 함께 제거됩니다. **/
    options: PollPutOption[]
    /** 아직 설문이 없는 댓글의 경우 미래 시점이어야 합니다. 계속 열려 있는 설문은 생략하세요. **/
    closesAt?: string | null
    /** 0: 익명(기본값), 1: 관리자 및 중재자, 2: 모두. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Put 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPutResponse {
    status: 'success' | 'failed'
    /** 실패 시 포함됩니다. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'polls-disabled' | 'poll-invalid' | 'replace-votes-required' | 'poll-privacy-locked' | 'locked'
    /** 실패 시 포함됩니다. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### 기타 참고 사항

- 설문에 존재하지 않는 `id` 이거나 동일한 `id` 를 두 번 제공하면 `poll-invalid` 오류가 발생합니다. 설문이 없는 댓글은 아직 옵션 `id` 가 없으므로 전송하는 모든 옵션은 `id` 를 생략해야 합니다.
- 설문에 투표가 있는 경우 프라이버시 설정은 좁게만 조정할 수 있고 넓게는 변경할 수 없습니다.
- 이 API는 사이트 설정을 따릅니다. 사이트나 페이지에서 설문이 활성화되지 않은 경우 `polls-disabled` 오류가 발생합니다.
- 잠긴 댓글은 설문을 변경할 수 없으며 `locked` 오류가 발생합니다.
- 연결된 위젯은 실시간으로 업데이트되므로 사용자는 새 설문을 페이지를 새로 고치지 않고도 볼 수 있습니다.