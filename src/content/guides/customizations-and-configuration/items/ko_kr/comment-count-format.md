---
[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

댓글 위젯 상단에 표시되는 댓글 수는 사용자 정의할 수 있습니다.

이는 원하는 문자열로 교체할 수 있으며, **[count]** 값은 사용자에게 현지화된 카운트 값으로 대체됩니다.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = '댓글 수 텍스트 사용자 정의'; code-example-end]

코드를 사용하지 않고도 위젯 사용자 정의 페이지에서 맞춤 설정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='위젯 사용자 정의 페이지의 댓글 수 텍스트 필드이며, [count]는 실시간 총합으로 대체됩니다'; title='댓글 수 텍스트 사용자 정의' app-screenshot-end]

---