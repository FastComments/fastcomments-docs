---
[related-parameter-start name = 'disableBlocking'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자가 다른 사용자를 차단할 수 있도록 허용합니다. 사용자를 차단하면 해당 사용자의 댓글이
마스킹되고, 사용자들 간의 알림이 차단되는 등 여러 영향이 있습니다.

이 기능을 비활성화하는 것이 바람직할 수 있습니다. 다음과 같이 할 수 있습니다:

[code-example-start config = {disableBlocking: true}; linesToHighlight = [6]; title = 'Disable Blocking'; code-example-end]

이 작업은 코드 없이도 수행할 수 있으며, 이렇게 하면 서버 측 유효성 검사도 적절히 활성화됩니다. 위젯 사용자 지정 UI를 통해 다음과 같이 할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-blocking']; selector = '.disable-blocking'; title='Disable Blocking' app-screenshot-end]

---