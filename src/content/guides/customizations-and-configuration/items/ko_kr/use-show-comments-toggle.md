[related-parameter-start name = 'useShowCommentsToggle'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 댓글 입력 상자와 댓글 스레드를 동시에 렌더링합니다. 세로 공간을 절약하기 위해 위젯이 상호작용될 때까지 다른 필수 필드들을 숨깁니다.

하지만 예를 들어 댓글 위젯을 버튼 뒤에 숨길 수 있습니다:

[app-screenshot-start width=700; url=`https://fastcomments.com/embed?config=%7B%22tenantId%22%3A%22L177BUDVvSe%22%2C%22useShowCommentsToggle%22%3A%22true%22%2C%22urlId%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22url%22%3A%22file%253A%252F%252F%252FC%253A%252FUsers%252Fwinrid%252FAppData%252FRoaming%252FJetBrains%252FWebStorm2021.2%252Fscratches%252Fscratch_295.html%22%2C%22pageTitle%22%3A%22%22%2C%22instanceId%22%3A%220.1281898364813452.1655790389169%22%7D&wId=comment-ui-v2`; selector = '.fast-comments'; delay=2000; alt='읽는 사람이 클릭할 때까지 댓글 수를 표시하는 버튼 뒤에 숨겨진 댓글 위젯'; title='댓글 표시 클릭' app-screenshot-end]

버튼은 현재 댓글이 표시되는지 여부에 따라 다른 번역 텍스트를 사용합니다. 댓글이 숨겨져 있을 경우 `translations.SHOW_COMMENTS_BUTTON_TEXT`를 사용하고, 댓글이 표시되어 있을 경우 `translations.HIDE_COMMENTS_BUTTON_TEXT`를 사용합니다. 번역 문자열에는 `[count]` 텍스트를 포함할 수 있으며, 이는 현지화된 댓글 수로 대체됩니다.

[code-example-start config = {useShowCommentsToggle: true}; linesToHighlight = [6]; title = '댓글 표시 또는 숨기기 클릭'; code-example-end]

이는 `hideCommentsUnderCountTextFormat` 설정을 대체하도록 설계되었습니다.

댓글 수는 댓글 스레드와 실시간으로 업데이트됩니다. 댓글이 없을 경우 버튼이 표시되지 않습니다.

코드를 작성하지 않고도 커스터마이징 규칙을 만들고 "댓글 표시 클릭"을 활성화하면 사용할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments']; selector = '.click-to-show-comments'; alt='위젯 커스터마이징 페이지의 커스터마이징 규칙에서 댓글 표시 체크박스가 선택된 상태'; title='댓글 표시 클릭 활성화' app-screenshot-end]