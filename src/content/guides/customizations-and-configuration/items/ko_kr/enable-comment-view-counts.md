[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 누가 각 댓글을 보았는지 추적하지 않으며 이에 대한 통계도 제공하지 않습니다.

하지만 이 기능을 활성화하면 시스템이 각 사용자가 댓글을 스크롤할 때 추적을 시작합니다.

이 경우 각 댓글에 표시되는 눈 아이콘 옆의 카운트가 증가합니다. 카운트는 실시간으로 업데이트되며 사용자의 로케일에 따라 축약됩니다.

이 기능은 **enableViewCounts** 플래그를 true로 설정하여 활성화할 수 있습니다:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

코드 없이 위젯 커스터마이징 페이지에서 이 설정을 맞춤화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='보기 카운트 체크박스가 선택된 위젯 커스터마이징 페이지로, 각 댓글에 눈 아이콘과 카운트가 표시됩니다.'; title='댓글 보기 카운트 활성화' app-screenshot-end]

우리는 댓글을 본 사용자 ID*를 추적하므로, 같은 댓글을 다시 보면 카운트가 증가하지 않습니다. 두 년 후에 다시 보면 카운트가 증가합니다.

- *Note: 익명 세션 ID 또는 사용자의 IP를 해시한 값일 수도 있습니다.