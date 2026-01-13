[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 댓글 영역의 오른쪽 상단에 알림 벨 아이콘을 표시합니다.

이 벨은 빨간색으로 바뀌고 사용자가 가진 알림 수를 표시합니다. 몇 가지 예시 알림은 다음과 같습니다:

- 사용자가 귀하에게 답글을 남겼습니다.
- 사용자가 당신이 댓글을 단 스레드에 답글을 남겼습니다.
- 사용자가 귀하의 댓글에 찬성(업보트)했습니다.
- 사용자가 당신이 구독한 페이지에 답글을 남겼습니다.

알림 벨은 페이지 전체를 구독할 수 있는 기능도 제공합니다.

하지만 알림 벨을 완전히 비활성화할 수 있습니다:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

이것은 코드 없이도 설정할 수 있습니다. 위젯 사용자화 페이지에서 '알림 벨 비활성화' 섹션을 참조하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]