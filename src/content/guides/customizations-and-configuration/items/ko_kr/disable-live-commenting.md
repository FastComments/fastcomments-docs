---
[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 실시간 댓글 기능이 활성화되어 있습니다.

즉, 댓글 스레드를 보는 모든 사용자는 동일한 내용을 보게 됩니다.

예를 들어, 댓글이 추가되면 해당 댓글이 표시됩니다. 댓글이 수정되거나 삭제되면,
그 댓글들은 스레드를 보는 모든 사용자에게도 수정되거나 삭제됩니다. 투표 및 모든 모더레이션 작업도 마찬가지입니다.

하지만, 이를 비활성화할 수 있습니다:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

코드를 사용하지 않고도 이 작업을 할 수 있습니다. 위젯 맞춤설정 페이지에서 "실시간 댓글 비활성화" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---