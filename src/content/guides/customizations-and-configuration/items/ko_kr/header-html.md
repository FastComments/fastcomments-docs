[related-parameter-start name = 'headerHTML'; type = 'string'; related-parameter-end]

헤더나 메시지와 같은 일부 텍스트는 댓글 수 아래이지만 로그인 상태 텍스트 위에 표시될 수 있습니다.

이를 헤더라고 하며 기본적으로 숨겨져 있습니다.

[code-example-start config = {headerHTML: "<h1>Leave a Comment!</h1>"}; linesToHighlight = [6]; title = 'Specifying Header HTML'; code-example-end]

이 항목은 코드 없이 위젯 사용자화 페이지의 고급 옵션에서 사용자화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.show-advanced-option'; selector = '.absolute-dates'; title='Specifying Header HTML' app-screenshot-end]