[related-parameter-start name = 'disableProfileDirectMessages'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자 프로필에 "Direct Messages" 탭을 표시하여 방문자가 사용자에게 직접 메시지를 보낼 수 있도록 합니다.

하지만 이 탭을 비활성화할 수 있습니다:

[code-example-start config = {disableProfileDirectMessages: true}; linesToHighlight = [6]; title = 'Disable Profile Direct Messages'; code-example-end]

코드를 사용하지 않고도 할 수 있습니다. 위젯 맞춤 설정 페이지에서 "Disable Direct Messages" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-profile-direct-messages']; selector = '.disable-profile-direct-messages'; alt='위젯 맞춤 설정 페이지에서 프로필 메시지 탭을 숨기기 위해 Disable Direct Messages 체크박스를 선택한 상태'; title='프로필 직접 메시지 비활성화' app-screenshot-end]