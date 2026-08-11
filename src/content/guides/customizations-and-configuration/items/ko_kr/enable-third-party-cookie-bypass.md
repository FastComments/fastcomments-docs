[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

인증을 위해 FastComments는 브라우저에서 제3자 쿠키가 활성화되어 있어야 합니다. 쿠키가 없으면 사용자는 항상 이메일을 입력해야 댓글을 달 수 있습니다(이메일 입력 필드가 숨겨져 있지 않은 경우), 그리고 기본적으로 댓글이 확인되지 않은 상태로 표시됩니다.

이를 해결하려면 제3자 쿠키 우회를 활성화할 수 있습니다. 

이 설정을 활성화하면 사용자가 로그인 중임을 알리는 메시지가 표시되는 작은 팝업이 나타납니다. 이 팝업은 사용자가 댓글 위젯과 상호작용할 때마다 표시됩니다; 예를 들어, 댓글을 남길 때.

코드에서 **enableThirdPartyCookieBypass** 플래그를 true로 설정하여 이 작업을 수행할 수 있습니다:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = '제3자 쿠키 우회 활성화'; code-example-end]

위젯 커스터마이징 UI에서 `Enable Third-Party Cookie Popup` 아래에 이 설정을 할 수도 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='제3자 쿠키 팝업 활성화 체크박스가 선택된 위젯 커스터마이징 페이지'; title='제3자 쿠키 우회 활성화' app-screenshot-end]