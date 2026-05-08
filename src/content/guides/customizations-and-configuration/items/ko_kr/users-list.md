[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 페이지에 사용자 목록을 표시하지 않습니다.

댓글 위젯 옆에 현재 페이지를 보고 있는 사람들의 목록을 렌더링할 수 있습니다. 목록은 사용자가 들어오고 나갈 때 실시간으로 업데이트되며, 이름, 아바타 및 온라인 표시기를 보여줍니다.

레이아웃 옵션은 세 가지입니다:

- `1` - Top: 댓글 위에 렌더링되는 겹친 아바타들의 수평 행입니다.
- `2` - Left: 위젯 왼쪽에 이름과 온라인 점이 표시된 사이드바입니다.
- `3` - Right: 동일한 사이드바가 위젯 오른쪽에 렌더링됩니다.

이 기능을 활성화하려면 **usersListLocation** 플래그를 설정하세요:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

기본적으로 목록에는 현재 온라인인 사용자만 표시됩니다. 과거에 이 페이지에 댓글을 남겼지만 현재는 보고 있지 않은 사람들도 포함하려면 **usersListIncludeOffline**을 true로 설정하세요:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

과거 댓글 작성자는 녹색 온라인 점 없이 렌더링되어 누가 지금 접속 중인지 명확히 알 수 있습니다.

비공개 프로필 사용자는 일반 아바타와 "비공개 프로필" 레이블로 표시되어 신원을 드러내지 않으면서도 개수는 정확하게 유지됩니다.

이 기능은 코드 없이도 구성할 수 있습니다. 위젯 커스터마이즈 페이지에서 "사용자 목록 위치" 옵션을 확인하세요:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-location'; title='Users List Location' app-screenshot-end]

위치가 Off가 아닌 다른 값으로 설정되면, 그 아래에 "과거 댓글 작성자 포함" 체크박스가 표시됩니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-include-offline'; title='Include Past Commenters' app-screenshot-end]