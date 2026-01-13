[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

기본적으로 최상위 댓글의 답글이 표시됩니다.

사용자가 자식 댓글을 보려면 최상위 댓글에서 '답글 보기'를 클릭해야 하도록 구성할 수 있습니다.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = 'Collapse Replies to Top Level Comments'; code-example-end]

이 설정은 코드 없이 위젯 맞춤 설정 페이지에서 변경할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; title='Collapse Replies' app-screenshot-end]

이 설정은 처음 로드되는 최상위 댓글의 수에는 영향을 주지 않습니다. 예를 들어 최상위 댓글이 1개이고 자식 댓글이 29개인 경우, 이 설정을 켜면:

- 최상위 댓글을 볼 수 있습니다.
- 이 댓글 아래에 '답글 보기 (29)'가 표시됩니다.

이 옵션과 함께 모든 최상위 댓글을 표시하려면 [시작 페이지를 -1로 설정](#starting-page)하십시오.