기본적으로 FastComments는 사용자가 자신의 댓글을 수정할 수 있도록 허용합니다.

그러나 이를 방지할 수 있습니다.

위젯 맞춤 설정 페이지에서 "편집 비활성화" 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- 이 설정은 일반 댓글 작성자에게만 영향을 미치며, 중재자(moderators)나 관리자(admins)는 여전히 댓글을 수정할 수 있습니다.
- 이 설정은 `contextUserId`가 전달되는 경우의 API 통합에도 영향을 미칩니다.