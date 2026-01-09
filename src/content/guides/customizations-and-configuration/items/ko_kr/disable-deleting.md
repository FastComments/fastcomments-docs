기본적으로 FastComments는 사용자가 자신의 댓글을 삭제할 수 있도록 허용합니다.

하지만 이를 방지할 수 있습니다.

위젯 맞춤 설정 페이지에서 '삭제 비활성화' 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- 이것은 일반 댓글 작성자에게만 영향을 미치며, 중재자나 관리자들은 여전히 삭제할 수 있습니다.
- 이것은 `contextUserId`가 전달되는 경우 API 통합에도 영향을 미칩니다. 

---