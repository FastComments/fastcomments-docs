---
[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자 배지를 댓글 스레드 내의 사용자 댓글에만 표시합니다.

그러나 위젯 사용자화 페이지에서 이 기능을 활성화하면 댓글 작성 양식 위에 있는 이름 옆에 사용자 배지를 표시할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; title='Show Badges in Top Bar Option' app-screenshot-end]

이렇게 하면 사용자가 댓글을 작성할 때 상단 바 영역에서 이름 옆에 배지가 표시되어 업적과 상태가 더 눈에 띄게 됩니다.

이 기능은 작동하려면 위젯 사용자화 UI에서 활성화되어 있어야 합니다. 서버 수준에서 켜져 있어도 코드 구성에서 **showBadgesInTopBar** 플래그를 false로 설정하여 선택적으로 비활성화할 수 있습니다:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]
---