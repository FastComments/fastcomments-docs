기본적으로 사용자는 자신의 댓글을 삭제할 수 있습니다. 또한, 댓글을 삭제하면 해당 스레드의 모든 하위 및 임시 댓글이 자동으로
삭제됩니다. 이 동작은 실시간으로 적용됩니다.

You can restrict this in the following ways:

- 대신 삭제된 댓글을 익명 처리하여 이름과 내용을 `[deleted]` 또는 사용자 지정 값으로 설정합니다.
- 답글이 있는 경우 댓글 삭제를 허용하지 않습니다. 사용자 지정 가능한 오류 메시지가 표시됩니다.
- 댓글에 답글이 있는 경우 삭제 권한을 관리자와 중재자에게만 제한합니다.

This can be configured via the `Comment Thread Deletion` section in the Widget Customization UI.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]