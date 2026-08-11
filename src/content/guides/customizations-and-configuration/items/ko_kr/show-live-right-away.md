[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

기본적으로 실시간 댓글이 활성화됩니다. 이는 댓글이 추가, 삭제, 편집 또는 고정될 경우, 해당 변경 사항이 댓글 스레드를 보고 있는 모든 사용자에게 동시에 표시된다는 의미입니다.

하지만 기본적으로 새 댓글은 "Show 2 New Comments"와 유사한 텍스트가 표시된 동적으로 나타나는 버튼 아래에 표시됩니다.

새 댓글이 페이지에 직접 달린 답글인 경우, 버튼은 댓글 스레드 상단에 표시됩니다. 특정 댓글에 대한 답글인 경우, 버튼은 해당 댓글 아래에 표시됩니다.

이는 페이지 크기가 지속적으로 변하는 것을 방지하여 사용자가 스크롤 바를 잡으려 할 때 발생할 수 있는 불편함을 최소화하기 위함입니다.

실시간 입찰이나 온라인 이벤트와 같은 일부 사용 사례에서는 이러한 동작이 원하지 않을 수 있습니다. 새 댓글이 즉시 표시되는 "채팅" 상자와 같은 형태의 댓글 위젯을 원할 수 있습니다.

따라서 해당 기능을 활성화하는 플래그 이름은 **showLiveRightAway** 입니다.

다음과 같이 활성화할 수 있습니다:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = '실시간 댓글 바로 표시'; code-example-end]

코드 없이 위젯 맞춤 설정 페이지에서 이 설정을 사용자 지정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='실시간 댓글 접기 설정을 토글하여 새 댓글이 버튼 뒤가 아니라 즉시 표시되도록 함'; title='실시간 댓글 바로 표시' app-screenshot-end]