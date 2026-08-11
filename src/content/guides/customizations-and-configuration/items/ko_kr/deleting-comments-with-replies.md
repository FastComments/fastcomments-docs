---
기본적으로 사용자는 자신의 댓글을 삭제할 수 있습니다. 또한, 댓글을 삭제하면 해당 스레드의 모든 하위 및 일시적인 댓글이 자동으로 삭제됩니다. 이 동작은 실시간으로 적용됩니다.

다음과 같은 방법으로 이를 제한할 수 있습니다:

- 대신, 삭제된 댓글을 익명 처리합니다 (이름과 텍스트를 `[deleted]` 또는 사용자 정의 값으로 설정).
- 답글이 있는 경우 댓글 삭제를 허용하지 않습니다. 사용자 정의 가능한 오류 메시지가 표시됩니다.
- 댓글에 답글이 있을 때 삭제를 관리자와 중재자에게만 제한합니다.

이는 위젯 맞춤 UI의 `Comment Thread Deletion` 섹션을 통해 구성할 수 있습니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='답글이 있는 경우 삭제를 익명 처리하거나 제한하기 위한 위젯 맞춤 UI의 댓글 스레드 삭제 옵션'; title='답글에 대한 삭제 동작 맞춤 설정' app-screenshot-end]

---