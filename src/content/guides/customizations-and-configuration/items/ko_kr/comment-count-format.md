---
[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

댓글 위젯 상단에 표시되는 댓글 수는 사용자 지정할 수 있습니다.

이 문자열은 임의의 문자열로 바꿀 수 있으며, 값 **[count]** 은 사용자에 맞게 현지화된 카운트 값으로 대체됩니다.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Customizing The Comment Count Text'; code-example-end]

이 설정은 코드 없이 위젯 사용자 지정 페이지에서 변경할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; title='Customizing The Comment Count Text' app-screenshot-end]


---