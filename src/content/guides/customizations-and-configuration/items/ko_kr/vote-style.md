[related-parameter-start name = 'voteStyle'; type = 'number'; related-parameter-end]

기본적으로 FastComments는 투표 옵션을 위쪽 및 아래쪽 화살표로 표시하여 사용자가 댓글에 찬성 또는 반대표를 할 수 있게 합니다.

그러나 투표 도구 모음의 스타일을 변경할 수 있습니다. 현재 옵션은 기본 위/아래 버튼이거나 하트 스타일 투표 메커니즘을 사용하는 것입니다.

다음과 같이 **voteStyle** 플래그를 사용합니다:

[code-example-start config = {voteStyle: 1}; linesToHighlight = [6]; title = '하트 버튼 활성화'; code-example-end]

코드 없이 이 작업을 수행하는 것을 강력히 권장합니다. 이렇게 하면 서버 측 검증도 활성화됩니다. 위젯 커스터마이징 페이지에서 "Vote Style" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.vote-style'; selector = '.vote-style'; alt='위젯 커스터마이징 페이지의 투표 스타일 설정, 위아래 화살표 또는 하트 투표 제공'; title='투표 스타일 변경' app-screenshot-end]

투표는 또한 비활성화할 수 있으며, 스타일 옵션 위에 있는 `Disable Voting`을 참조하십시오.