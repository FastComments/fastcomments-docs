---
기본적으로 FastComments는 사용자가 자신의 댓글을 편집하도록 허용합니다.

하지만 이를 방지할 수도 있습니다.

위젯 사용자 정의 페이지에서 "편집 비활성화" 옵션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='위젯 사용자 정의 페이지에서 편집 비활성화 옵션, 댓글 작성자가 댓글을 편집하는 것을 방지합니다'; title='댓글 편집 비활성화' app-screenshot-end]

- 이는 일반 댓글 작성자에게만 영향을 미치며, 여전히 편집할 수 있는 중재자나 관리자에게는 영향을 주지 않습니다.
- `contextUserId`가 전달될 때 API 통합에도 영향을 미칩니다. 

---