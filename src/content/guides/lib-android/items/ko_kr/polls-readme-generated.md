투표(Polls)는 웹 위젯과 동일하게 작동하며, FastComments 대시보드의 동일한 위젯 설정(또는 위의 `pollCreation` / `pollVoting`)에서 켤 수 있습니다.

- 투표가 포함된 댓글은 텍스트 아래에 투표를 표시합니다. 독자는 옵션을 탭하여 투표할 수 있고, 투표 전에 결과를 보이거나 숨길 수 있으며, 투표를 변경할 수도 있습니다. 다른 사람들이 투표하면 집계가 실시간으로 업데이트됩니다.
- 투표는 특정 날짜에 마감되거나, 투표할 때까지 결과를 숨기거나, 투표자를 관리자에게만 또는 모든 사람에게 보이게 할 수 있습니다("투표자 보기").
- 조회자가 투표를 만들 수 있는 경우, `FastCommentsView`의 댓글 입력창에 투표 빌더를 여는 투표 버튼이 표시됩니다. 툴바를 켠 적이 없더라도 이를 위한 툴바 행이 나타납니다. `sdk.setCommentToolbarEnabled(false)`를 호출하면 투표 버튼을 포함해 툴바가 숨겨지고, 입력창의 `setPollCreationEnabled(false)`는 투표 버튼만 제거합니다. 입력창이 SDK를 받을 때 이 설정을 읽어들이므로 `FastCommentsView`를 생성하기 전에 `setCommentToolbarEnabled`를 호출하세요. 이미 화면에 있는 뷰의 경우 대신 `getBottomCommentInput().setToolbarVisible(false)`를 호출하세요. 라이브 채팅 입력창은 웹 라이브 채팅과 마찬가지로 이 버튼을 표시하지 않습니다.

자체 UI에서 투표를 게시하려면 `CommentPollInput`을 `postComment`에 전달하세요:

```java
CommentPollInput poll = new CommentPollInput()
        .question("Where should we eat?")
        .options(Arrays.asList("Tacos", "Ramen"))
        // 선택 사항: ISO 8601 마감 날짜, PollPrivacy 값, 투표할 때까지 결과 숨기기.
        .closesAt(Instant.now().plus(1, ChronoUnit.DAYS).toString())
        .privacy(PollPrivacy.NUMBER_2.getValue())
        .requireVoteToSeeResults(true);
sdk.postComment("Lunch vote", null, null, poll, callback);
```

`sdk.votePoll(commentId, optionId, callback)`과 `sdk.getPollVoters(commentId, page, search, callback)`가 투표 및 투표자 목록 기능을 담당합니다. 투표 색상은 THEMING.md에 나열되어 있습니다.