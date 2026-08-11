[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자가 다른 사용자를 차단할 수 있도록 허용합니다. 사용자를 차단하면 해당 사용자의 댓글이 가려지고, 사용자 간 알림이 차단되는 등 여러 효과가 있습니다.

이 기능을 비활성화하고 싶을 수 있습니다. 다음과 같이 할 수 있습니다:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = '차단 비활성화'; code-example-end]

코드 없이도 이 작업을 수행할 수 있으며, 이는 적절한 서버 측 검증을 가능하게 합니다. 위젯 맞춤 설정 UI를 통해 수행합니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; alt='위젯 맞춤 설정 UI에서 차단 옵션을 비활성화하면 사용자가 서로 차단하는 것을 방지합니다.'; title='차단 비활성화' app-screenshot-end]