[related-parameter-start name = 'enableCommenterLinks'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자에게 댓글 내용, 사용자 이름, 그리고 이메일만 요청합니다.

하지만 경우에 따라 사용자가 자신의 블로그나 웹사이트 링크를 남기도록 하고 싶을 수 있습니다.

사용자의 웹사이트 URL을 입력할 수 있는 추가 입력 필드를 표시하려면 **enableCommenterLinks** 플래그를 true로 설정하면 됩니다:

[code-example-start config = {enableCommenterLinks: true}; linesToHighlight = [6]; title = 'Enabling Commenter Links'; code-example-end]

해당 URL이 제공되면 사용자의 계정이 업데이트되고 과거 및 향후 모든 댓글에서 해당 사용자의 이름은 이 URL로 링크됩니다.

이 설정은 위젯 커스터마이즈 페이지에서 코드 없이도 변경할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.click-to-show-comments', '.commenter-links']; selector = '.commenter-links'; title='Enabling Commenter Links' app-screenshot-end]