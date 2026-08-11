---
[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 댓글 영역 오른쪽 상단에 알림 벨을 표시합니다.

이 벨은 빨간색으로 변하고 사용자가 받은 알림 수를 표시합니다. 예시 알림은 다음과 같습니다:

- 사용자가 당신에게 답글을 달았습니다.
- 사용자가 당신이 댓글을 단 스레드에 답글을 달았습니다.
- 사용자가 당신의 댓글에 찬성을 표시했습니다.
- 사용자가 당신이 구독한 페이지에 답글을 달았습니다.

알림 벨은 전체 페이지를 구독하는 메커니즘도 제공합니다.

그러나 알림 벨을 완전히 비활성화할 수 있습니다:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

코드를 사용하지 않고도 할 수 있습니다. 위젯 사용자 정의 페이지에서 "알림 벨 끄기" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='알림 벨 끄기 체크박스가 선택된 위젯 사용자 정의 페이지'; title='알림 벨 끄기' app-screenshot-end]

---