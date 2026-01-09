[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 확인되지 않은 브라우저 세션을 가진 사용자에게 남겨진 댓글에 대해
"Unverified Comment" 라벨을 표시합니다. 확인되지 않은 댓글 작성에 대해 자세히 알아보려면 [여기](https://docs.fastcomments.com/guide-comment-vote-verification.html)를 참조하세요.

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

또한, 이 기능은 코드를 작성하지 않고 Customization UI에서 사용할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]