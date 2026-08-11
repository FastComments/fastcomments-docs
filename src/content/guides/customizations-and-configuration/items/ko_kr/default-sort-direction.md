[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

기본적으로 FastComments는 댓글을 "Most Relevant" 정렬 방향으로 정렬합니다.

Most Relevant 정렬은 댓글이 작성된 시간과 투표 수를 고려하여 정렬합니다.

사용자는 댓글 위젯 UI에서 정렬 방향을 Oldest 또는 Newest First 중 하나로 변경할 수 있습니다.

하지만 기본값을 세 가지 중 어느 것이든 변경할 수 있습니다. 예를 들어 가장 오래된 댓글을 먼저 표시하고 싶다면:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

**defaultSortDirection** 값을 "OF" 로 설정하여 방향을 "OF" 로 지정합니다.

Newest-first 정렬 방향을 위해서는 다음과 같이 합니다:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

**defaultSortDirection**에 대한 유효한 값은 다음과 같습니다:

- MR: "Most Recent"
- NF: "Newest First"
- OF: "Oldest First"

코드 없이도 설정할 수 있습니다. 위젯 커스터마이징 페이지에서 "Default Sort Direction" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; alt='Most Relevant, Newest First, 및 Oldest First를 제공하는 기본 정렬 방향 선택기'; title='기본 정렬 방향 변경' app-screenshot-end]

각 정렬 방향에 대한 페이지별 댓글은 미리 계산되므로 모든 정렬 방향이 동일한 성능을 가집니다.