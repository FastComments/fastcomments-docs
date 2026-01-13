[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

인증을 위해 FastComments는 브라우저에서 타사 쿠키가 활성화되어 있어야 합니다. 쿠키가 없으면(이메일 입력 필드가 숨겨져 있지 않은 한) 사용자는 항상 댓글을 남길 때 이메일을 입력해야 하며, 댓글은 기본적으로 항상 검증되지 않은 상태로 표시됩니다.

이를 우회하려면 타사 쿠키 우회를 활성화할 수 있습니다. 

이 설정이 활성화되면 사용자가 로그인되고 있음을 알리는 작은 팝업이 표시됩니다. 이 팝업은 사용자가 댓글 위젯과 상호작용할 때마다 표시됩니다; 예를 들어 댓글을 남길 때 표시됩니다.

코드에서는 **enableThirdPartyCookieBypass** 플래그를 true로 설정하여 이 기능을 활성화할 수 있습니다:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

또한 위젯 사용자화 UI에서 `Enable Third-Party Cookie Popup` 항목을 통해 이 설정을 구성할 수도 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]