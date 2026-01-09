[related-parameter-start name = 'defaultSortDirection'; type = 'string'; related-parameter-end]

기본적으로 FastComments는 댓글을 "가장 관련성 높은" 정렬 방향으로 정렬합니다.

가장 관련성 높은 정렬은 댓글이 남겨진 시간과 추천 수를 정렬에 반영합니다.

사용자는 댓글 위젯 UI에서 정렬 방향을 '가장 오래된' 또는 '최신 순'으로 변경할 수 있습니다.

하지만 기본값은 세 가지 중 어떤 것으로든 변경할 수 있습니다. 예를 들어 가장 오래된 댓글을 먼저 표시하려면:

[code-example-start config = {defaultSortDirection: "OF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Oldest First'; code-example-end]

**defaultSortDirection**의 값을 "OF"로 설정하여 정렬 방향을 "OF"로 설정합니다.

최신 순 정렬 방향의 경우에는 다음과 같이 합니다:

[code-example-start config = {defaultSortDirection: "NF"}; linesToHighlight = [6]; title = 'Changing The Default Sort To Newest First'; code-example-end]

**defaultSortDirection**의 유효한 값은 다음과 같습니다:

- MR: "가장 최근"
- NF: "최신 순"
- OF: "가장 오래된 순"

이 작업은 코드 없이도 할 수 있습니다. 위젯 맞춤 설정 페이지에서 '기본 정렬 방향' 섹션을 참조하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.default-sort-direction'; title='Changing The Default Sort Direction' app-screenshot-end]

참고로, 각 페이지의 댓글은 각 정렬 방향에 대해 미리 계산되어 있으므로 모든 정렬 방향에서 성능은 동일합니다.