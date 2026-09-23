[related-parameter-start name = 'enableViewCounts'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 누가 각 댓글을 보았는지 추적하거나 이에 대한 통계를 제공하지 않습니다.

하지만 이 기능을 활성화하면, 사용자가 댓글까지 스크롤할 때마다 시스템이 추적을 시작합니다.

이 경우 각 댓글에 표시되는 눈 아이콘 옆의 카운트가 증가합니다. 카운트는 실시간으로 업데이트되며 사용자의 로케일에 맞게 축약됩니다.

**enableViewCounts** 플래그를 true로 설정하면 이 기능을 활성화할 수 있습니다:

[code-example-start config = {enableViewCounts: true}; linesToHighlight = [6]; title = 'Enabling Comment View Counts'; code-example-end]

코드를 사용하지 않고 위젯 사용자 정의 페이지에서 이 설정을 맞춤화할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-view-counts']; selector = '.enable-view-counts'; alt='위젯 사용자 정의 페이지에서 보기 횟수 체크박스가 선택된 상태이며 각 댓글에 눈 아이콘과 카운트가 표시됩니다.'; title='댓글 보기 횟수 활성화' app-screenshot-end]

우리는 댓글을 본 사용자 ID*를 일주일 동안 추적합니다. 따라서 일주일 이내에 다시 댓글을 보면 카운트가 증가하지 않습니다. 일주일이 지난 후 다시 댓글을 보면 카운트가 다시 증가합니다.

- *참고: 익명 세션 ID 또는 사용자의 IP를 해시한 값.