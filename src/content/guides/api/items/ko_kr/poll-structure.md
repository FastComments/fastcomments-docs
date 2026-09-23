A `Poll`은 자체 객체가 아니라 댓글에 연결됩니다. 댓글과 함께 생성됩니다(`POST /api/v1/comments` 참조) 또는 나중에 `PUT /api/v1/polls/:commentId` 로 기존 댓글에 추가됩니다.

투표 수는 설문 자체에 저장되므로 설문을 읽는 것만으로 결과를 확인할 수 있으며 별도로 합산할 필요가 없습니다. 해당 수치 뒤에 있는 개별 투표는 `PollVote` 객체입니다.

각 옵션은 설문이 생성될 때 생성되는 `id`를 가집니다. 이 id를 사용해 투표를 하거나 옵션의 라벨을 변경하고, 옵션을 추가하거나 제거하면서 `PUT`으로 설문을 업데이트할 때 옵션(및 해당 투표)을 유지합니다. 옵션을 참조하는 유일한 안전한 방법은 이 id이며, 리스트 내 위치를 사용해서는 안 됩니다.

[inline-code-attrs-start title = '설문 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentPollOption {
    id: string
    label: string
    votes: number
}

interface CommentPoll {
    question: string
    options: CommentPollOption[]
    totalVotes: number
    /** 설정되어 과거인 경우, 설문이 종료되어 더 이상 투표를 받지 않습니다. **/
    closesAt?: string | null
    /** 0 익명(기본값), 1 관리자 및 중재자, 2 모두. 없으면 익명으로 간주됩니다. **/
    privacy?: 0 | 1 | 2 | null
    /** true인 경우, 아직 투표하지 않은 사람에게는 투표 수가 숨겨집니다. 없으면 false로 간주됩니다. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### Limits

- 질문은 필수이며 최대 200자까지 가능합니다.
- 설문은 2개에서 10개 사이의 옵션을 가져야 합니다.
- 옵션 라벨은 필수이며 최대 100자이고, 설문 내에서(대소문자 구분 없이) 고유해야 합니다.
- `closesAt`은 설문 생성 시 미래 시점이어야 합니다. 설문을 즉시 종료하려면 과거 날짜와 함께 `PATCH` 요청을 보냅니다.

### Site Settings

설문은 사이트 설정을 따르며, 이는 Customize Widget 아래에서 변경할 수 있습니다:

- 설문을 생성하기 전에 설문 기능을 활성화해야 하며, 그렇지 않으면 API가 `polls-disabled` 오류를 반환합니다.
- 투표를 로그인한 사용자로 제한할 수 있으며, 이 경우 `anonUserId`만 포함된 투표는 `poll-login-required` 오류로 거부됩니다.