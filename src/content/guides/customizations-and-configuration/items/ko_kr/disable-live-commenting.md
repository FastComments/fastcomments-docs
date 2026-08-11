[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 실시간 댓글 기능이 활성화됩니다.

이는 댓글 스레드를 보는 모든 사용자가 동일한 내용을 보게 된다는 의미입니다.

예를 들어, 댓글이 추가되면 해당 댓글이 표시됩니다. 댓글이 편집되거나 삭제되면,
해당 댓글은 스레드를 보는 모든 사용자에게 편집되거나 삭제된 것으로 반영됩니다. 투표 및 모든 관리 작업도 마찬가지입니다.

하지만 이를 비활성화할 수 있습니다:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = '실시간 댓글 비활성화'; code-example-end]

코드 없이도 할 수 있습니다. 위젯 커스터마이징 페이지에서 "실시간 댓글 비활성화" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='위젯 커스터마이징 페이지의 실시간 댓글 비활성화 섹션, 실시간 스레드 업데이트를 끕니다'; title='실시간 댓글 비활성화' app-screenshot-end]