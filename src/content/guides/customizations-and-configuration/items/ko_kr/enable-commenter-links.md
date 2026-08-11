[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자에게 댓글, 사용자 이름 및 이메일만 요청합니다.

하지만 경우에 따라 사용자가 자신의 블로그나 웹사이트 링크를 남기도록 할 수 있습니다.

**enableCommenterLinks** 플래그를 true로 설정하면 사용자의 웹사이트 URL을 입력할 수 있는 추가 입력 필드를 표시하도록 활성화할 수 있습니다:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

해당 URL이 제공되면 사용자의 계정이 업데이트되고 과거 및 미래의 모든 댓글에서 사용자 이름이 이 URL에 연결됩니다.

이는 코드 없이 위젯 맞춤 설정 페이지에서 사용자 지정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; alt='댓글자 링크 체크박스가 선택된 위젯 맞춤 설정 페이지로, 댓글 양식에 웹사이트 URL 필드를 추가합니다.'; title='댓글자 링크 활성화' app-screenshot-end]