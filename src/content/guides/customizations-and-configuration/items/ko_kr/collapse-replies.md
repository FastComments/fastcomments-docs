[related-parameter-start name = 'collapseReplies'; type = 'boolean'; related-parameter-end]

기본적으로, 최상위 댓글에 대한 답글이 표시됩니다.

이 설정을 변경하면 사용자가 최상위 댓글에서 "Show Replies"를 클릭해야 자식 댓글을 볼 수 있습니다.

[code-example-start config = {collapseReplies: true}; linesToHighlight = [6]; title = '최상위 댓글에 대한 답글 접기'; code-example-end]

코드를 사용하지 않고 위젯 맞춤 설정 페이지에서 이 옵션을 사용자 지정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.collapse-replies'; alt='위젯 맞춤 설정 UI에서 답글을 접는 옵션으로, 자식 댓글을 Show Replies 링크 뒤에 숨깁니다'; title='답글 접기' app-screenshot-end]

이 설정은 처음 로드되는 최상위 댓글 수에 영향을 주지 않습니다. 최상위 댓글이 하나이고, 자식 댓글이 29개인 경우, 이 설정을 켜면 다음과 같이 표시됩니다:

- 최상위 댓글을 확인합니다.
- 이 댓글 아래에 Show Replies (29)를 확인합니다.

이 옵션과 함께 모든 최상위 댓글을 표시하려면, [시작 페이지를 -1로 설정](#starting-page)하십시오.