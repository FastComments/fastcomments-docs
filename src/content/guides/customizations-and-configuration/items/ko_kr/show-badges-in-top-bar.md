[related-parameter-start name = 'showBadgesInTopBar'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 댓글 스레드 내에서 사용자의 댓글에만 배지를 표시합니다.

하지만 위젯 커스터마이징 페이지에서 이 기능을 활성화하면 댓글 양식 위에 이름 옆에 사용자 배지를 표시할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.show-badges-in-top-bar'; alt='위젯 커스터마이징 페이지에서 상단 바에 배지를 표시하는 체크박스이며, 댓글 양식 위에 이름 옆에 배지를 배치합니다'; title='상단 바에 배지 표시 옵션' app-screenshot-end]

이렇게 하면 상단 바 영역에 사용자의 이름 옆에 배지가 표시되어, 댓글을 작성할 때 사용자의 업적과 상태가 더 눈에 띄게 됩니다.

이 기능이 작동하려면 위젯 커스터마이징 UI에서 반드시 활성화되어야 합니다. 서버 수준에서 켜져 있더라도 코드 구성에서 **showBadgesInTopBar** 플래그를 false로 설정하여 선택적으로 비활성화할 수 있습니다:

[code-example-start config = {showBadgesInTopBar: false}; linesToHighlight = [6]; title = 'Disable Show Badges in Top Bar'; code-example-end]