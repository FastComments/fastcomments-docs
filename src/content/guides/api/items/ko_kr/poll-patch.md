[api-resource-header-start name = 'Poll'; route = 'PATCH /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

투표의 투표수를 방해하지 않고 설문을 편집합니다. 질문이나 옵션의 오타를 수정하거나, 설문을 닫거나 다시 열거나, 누가 투표했는지를 볼 수 있는 권한을 변경할 때 사용합니다.

옵션은 `id` 로 지정되며, `PATCH` 는 지정한 옵션의 라벨을 변경합니다. 옵션을 추가, 제거 또는 순서를 바꾸려면 전체 옵션 목록을 `PUT /api/v1/polls/:commentId` 로 전송하십시오: `id`와 함께 전송한 옵션은 투표도 그대로 유지됩니다.

모든 필드는 선택 사항이지만, 최소 하나는 제공해야 합니다.

[inline-code-attrs-start title = 'Poll Patch cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"question": "Which release should we cut first?",
	"options": [{"id": "the-option-id", "label": "The bugfix release"}]
}'
[inline-code-end]

[inline-code-attrs-start title = '지금 설문 닫기 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"closesAt": "2020-01-01T00:00:00.000Z"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchQueryParams {
    tenantId: string
    API_KEY: string
}

interface PollPatchBody {
    question?: string | null
    /** Relabels existing options. Every id given must already be on the poll. **/
    options?: { id: string, label: string }[] | null
    /** A date in the past closes the poll now. null reopens a closed poll. **/
    closesAt?: string | null
    /** 0 anonymous, 1 admins and moderators, 2 everyone. **/
    privacy?: 0 | 1 | 2 | null
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

[inline-code-attrs-start title = 'Poll Patch 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollPatchResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found' | 'poll-invalid' | 'poll-privacy-locked' | 'locked'
    /** Included on failure. **/
    reason?: string
    poll: CommentPoll
}
[inline-code-end]

### 기타 참고 사항

- 설문에 존재하지 않는 옵션 id를 지정하면 `poll-invalid` 오류가 발생하며, 조용히 무시되지 않습니다.
- 라벨은 설문 내에서 고유해야 하며, 변경하지 않는 옵션도 포함하여 중복되지 않아야 합니다.
- 설문을 생성할 때와 달리, 여기서는 `closesAt` 를 과거 시점으로 지정할 수 있습니다—즉시 설문을 닫는 방법입니다.
- 설문에 투표가 있으면 프라이버시 설정을 좁게 할 수는 있지만, 넓게 할 수는 없습니다.
- 잠긴 댓글은 설문을 변경할 수 없으며, `locked` 오류가 발생합니다.

---