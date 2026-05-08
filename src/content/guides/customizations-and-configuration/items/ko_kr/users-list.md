[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 페이지에 사용자 목록을 표시하지 않습니다.

댓글 위젯과 함께 현재 페이지를 보고 있는 사람들의 목록을 렌더링할 수 있습니다. 이 목록은 사용자가 들어오고 나갈 때 실시간으로 업데이트되며, 이름, 아바타 및 온라인 표시기를 보여줍니다.

레이아웃 옵션은 세 가지가 있습니다:

- `1` - 상단: 댓글 위에 겹쳐진 아바타가 가로로 표시됩니다.
- `2` - 왼쪽: 위젯 왼쪽에 이름과 온라인 점이 있는 사이드바가 표시됩니다.
- `3` - 오른쪽: 위젯 오른쪽에 동일한 사이드바가 표시됩니다.

기능을 활성화하려면 **usersListLocation** 플래그를 설정하세요:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = 'Show Users List on the Right'; code-example-end]

기본적으로 목록은 현재 온라인 상태인 사용자만 표시합니다. 과거에 페이지에 댓글을 남겼지만 현재 보고 있지 않은 사람들도 포함하려면 **usersListIncludeOffline**을 true로 설정하세요:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = 'Include Past Commenters'; code-example-end]

과거 댓글 작성자는 초록색 온라인 점 없이 렌더링되어 누가 지금 접속해 있는지 명확하게 보여줍니다.

비공개 프로필을 가진 사용자는 일반 아바타와 "Private Profile" 레이블로 표시되어 신원을 밝히지 않으면서도 집계 수는 정확하게 유지됩니다.

코드 없이도 구성할 수 있습니다. 위젯 커스터마이즈 페이지에서 "Users List Location" 옵션을 참조하세요. 위치가 Off가 아닌 값으로 설정되면 그 아래에 "Include past commenters" 체크박스가 나타납니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; title='Users List Settings'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

실시간 사용자 500명 이후에는 목록이 최대 30초까지 최신이 아닐 수 있습니다.