`Poll`는 독립적인 객체가 아니라 댓글에 첨부됩니다. 댓글과 함께 생성되거나( `POST /api/v1/comments` 참조), 이후 `PUT /api/v1/polls/:commentId`를 사용하여 기존 댓글에 추가됩니다.

투표 수는 poll 자체에 저장되므로, poll을 읽으면 아무것도 더할 필요 없이 결과를 바로 얻을 수 있습니다. 이 수치 뒤에 있는 개별 투표는 `PollVote` 객체입니다.

각 옵션은 poll이 생성될 때 생성되는 `id`를 가집니다. 이 id는 투표를 하거나, 옵션의 라벨을 변경하거나, 옵션을 추가하거나 제거하여 poll을 `PUT`할 때 옵션(및 해당 투표)을 유지하는 데 사용됩니다. 옵션을 참조하는 유일한 안전한 방법은 이 id이며, 목록에서의 위치는 절대 사용하지 마십시오.

[inline-code-attrs-start title = 'Poll 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** 설정되어 있고 과거의 시점인 경우, poll은 종료되어 더 이상 투표를 받지 않습니다. **/
    closesAt?: string | null
    /** 0은 익명(기본값), 1은 관리자 및 중재자, 2는 모든 사용자입니다. 값이 없으면 익명을 의미합니다. **/
    privacy?: 0 | 1 | 2 | null
    /** true인 경우, 아직 투표하지 않은 사용자에게는 수치가 숨겨집니다. 값이 없으면 false를 의미합니다. **/
    requireVoteToSeeResults?: boolean | null
}
[inline-code-end]

### 제한 사항

- 질문은 필수이며 최대 200자입니다.
- poll은 2개에서 10개 사이의 옵션을 가집니다.
- 옵션 라벨은 필수이며, 최대 100자이고, poll 내에서 고유해야 합니다(대소문자 무시).
- `closesAt`은 poll이 생성될 때 미래의 시점이어야 합니다. poll을 즉시 종료하려면 과거의 날짜로 `PATCH`하십시오.

### 사이트 설정

Poll은 사이트 구성을 따르며, 이는 Customize Widget에서 변경할 수 있습니다:

- poll을 생성하려면 먼저 Poll이 활성화되어 있어야 하며, 그렇지 않으면 API가 `polls-disabled`로 응답합니다.
- 투표는 로그인한 사용자로 제한될 수 있으며, 이 경우 `anonUserId`만 포함된 투표는 `poll-login-required`로 거부됩니다.