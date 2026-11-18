투표(Polls)는 웹 위젯과 동일하게 작동하며, FastComments 대시보드의 동일한 위젯 설정에서 켤 수 있습니다: `pollCreation` (누가 투표를 첨부할 수 있는지: 비활성화, 관리자 및 중재자, 또는 모든 사용자)와 `pollVoting` (로그인한 사용자 또는 누구나).

- 투표가 포함된 댓글은 텍스트 아래에 투표를 표시합니다. 독자는 옵션을 탭하여 투표할 수 있고, 투표 전에 결과를 표시하거나 숨길 수 있으며, 투표를 변경할 수도 있습니다. 다른 사람들이 투표함에 따라 집계가 실시간으로 업데이트됩니다.
- 투표는 특정 날짜에 마감되거나, 투표할 때까지 결과를 숨기거나, 투표자를 관리자 또는 모든 사람에게 표시할 수 있습니다("누가 투표했는지 보기").
- 뷰어가 투표를 생성할 수 있는 경우, `FastCommentsView`의 댓글 입력창에 투표 빌더를 여는 투표 버튼이 표시됩니다. 투표는 댓글과 함께 게시되므로 댓글 텍스트가 필요합니다. 실시간 채팅 입력창은 웹 실시간 채팅과 마찬가지로 이 버튼을 표시하지 않습니다.

투표하려면 로그인한 사용자가 필요하므로 [SSO](#authentication-sso-readme-generated)를 사용하세요. 서버는 익명 투표를 세션 쿠키에 연결하는데, API 클라이언트는 이 쿠키를 유지하지 않으므로 `pollVoting`이 누구나로 설정된 경우 SSO 사용자가 없는 뷰어에게는 "투표하려면 로그인하세요"가 표시됩니다. 투표할 때까지 결과를 숨기는 투표는 실시간 집계 업데이트를 받지 못하며, 해당 투표자는 다음 로드 시 새 숫자를 보게 됩니다.

자체 UI에서 투표를 게시하려면 `postComment`에 `CommentPollInput`을 전달하세요:

```swift
let draft = PollDraft(question: "Where should we eat?", options: ["Tacos", "Ramen"], closesDays: 1)
if draft.validate() == nil {
    try await sdk.postComment(text: "Lunch vote", poll: draft.toInput())
}
```

`sdk.votePoll(commentId:optionId:)`와 `sdk.getPollVoters(commentId:page:search:)`가 투표 및 투표자 목록을 처리합니다. 투표 색상은 [Poll Colors](#theming-readme-generated)에 나열되어 있습니다.