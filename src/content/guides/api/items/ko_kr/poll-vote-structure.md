`PollVote`는 설문에 대한 한 사람의 답변입니다. 설문 자체에 표시되는 카운트는 이와 동기화되어 유지되므로, 총합이 아니라 *누가* 무엇에 투표했는지 알고 싶을 때만 필요합니다.

투표자는 설문당 최대 하나의 투표만 가질 수 있습니다. 다시 투표하면 두 번째 투표를 추가하는 대신 기존 투표가 새로운 옵션으로 이동하며, `updatedAt`은 그 시점을 기록합니다.

`voterId`는 투표자가 로그인했을 때의 `userId`이며, 그렇지 않으면 `anonUserId`입니다.

[inline-code-attrs-start title = 'PollVote 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollVote {
    id: string
    tenantId: string
    commentId: string
    urlId: string
    /** 투표자가 로그인했을 때의 userId, 그렇지 않으면 anonUserId. **/
    voterId: string
    optionId: string
    createdAt: string
    /** 투표자가 마지막으로 투표를 다른 옵션으로 이동한 시점. **/
    updatedAt?: string
}
[inline-code-end]

### Privacy

설문의 `privacy` 설정은 댓글 위젯에 적용되는 방식과 동일하게 이 API에도 적용됩니다:

- **Anonymous** (the default): 아무도 누가 어떻게 투표했는지 볼 수 없으므로 투표를 읽을 수 없습니다. `GET /api/v1/poll-votes` 및 `GET /api/v1/poll-votes/:id`는 `poll-anonymous`로 응답합니다. 설문의 카운트는 `GET /api/v1/polls/:commentId`에서 여전히 확인할 수 있습니다.
- **Admins and moderators**: API 키가 사이트 관리자의 것이므로 투표를 읽을 수 있습니다.
- **Everyone**: 모든 사람이 투표를 읽을 수 있습니다.

설문에 투표가 있으면 프라이버시 설정을 좁게 할 수는 있지만 넓게 할 수는 없습니다.