[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

기본적으로 FastComments는 투표 옵션을 위/아래 화살표로 렌더링하여 사용자가 댓글에 찬성 또는 반대 투표를 할 수 있게 합니다.

하지만 투표 도구 모음의 스타일을 변경할 수 있습니다. 현재 옵션은 기본 위/아래 버튼 또는 하트 스타일 투표 방식입니다.

다음과 같이 **voteStyle** 플래그를 사용합니다:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = 'Enable Heart Button'; code-example-end]

이 작업은 서버 측 검증도 활성화되므로 코드 없이 수행할 것을 강력히 권장합니다. 위젯 사용자화 페이지의 "투표 스타일" 섹션을 참조하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; title='Change Voting Style' app-screenshot-end]

투표는 비활성화할 수도 있습니다. 스타일 옵션 위의 `Disable Voting`을 참조하세요.

---