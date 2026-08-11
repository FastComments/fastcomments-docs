[related-parameter-start name = 'usersListLocation'; type = 'number'; related-parameter-end]
[related-parameter-start name = 'usersListIncludeOffline'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 페이지에 사용자 목록을 표시하지 않습니다.

현재 페이지를 보고 있는 사람들의 목록을 댓글 위젯과 함께 렌더링할 수 있습니다. 사용자가 입장하거나 퇴장할 때 목록이 실시간으로 업데이트되며, 이름, 아바타 및 온라인 표시기가 표시됩니다.

다음 세 가지 레이아웃 옵션이 있습니다:

- `1` - Top: 댓글 위에 렌더링되는 겹치는 아바타들의 가로 행.
- `2` - Left: 위젯 왼쪽에 렌더링되는 이름과 온라인 점이 있는 사이드바.
- `3` - Right: 위젯 오른쪽에 렌더링되는 동일한 사이드바.

**usersListLocation** 플래그를 설정하여 기능을 활성화합니다:

[code-example-start config = {usersListLocation: 3}; linesToHighlight = [6]; title = '오른쪽에 사용자 목록 표시'; code-example-end]

기본적으로 목록은 현재 온라인인 사용자만 표시합니다. 과거에 페이지에 댓글을 달았지만 현재 보고 있지 않은 사람들을 포함하려면 **usersListIncludeOffline**을 true로 설정합니다:

[code-example-start config = {usersListLocation: 3, usersListIncludeOffline: true}; linesToHighlight = [6, 7]; title = '과거 댓글 작성자 포함'; code-example-end]

과거 댓글 작성자는 녹색 온라인 점 없이 렌더링되어 현재 누가 있는지 명확히 표시됩니다.

비공개 프로필을 가진 사용자는 일반 아바타와 "Private Profile" 라벨로 표시되어, 신원을 밝히지 않으면서도 카운트가 정확하게 유지됩니다.

코드 없이도 설정할 수 있습니다. 위젯 커스터마이징 페이지에서 "Users List Location" 옵션을 확인하세요. 위치가 Off가 아닌 값으로 설정되면, 아래에 "Include past commenters" 체크박스가 나타납니다.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.users-list-settings'; alt='사용자 목록 위치가 오른쪽으로 설정되고, 아래에 과거 댓글 작성자 포함 체크박스가 표시됨'; title='사용자 목록 설정'; actions=[{type: 'set-value', selector: '#users-list-location-input', value: '3'}] app-screenshot-end]

지난 500명의 실시간 사용자에 대해, 목록은 최대 30초 정도 지연될 수 있습니다.