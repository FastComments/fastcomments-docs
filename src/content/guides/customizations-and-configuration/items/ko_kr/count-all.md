[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

댓글 위젯 상단에 표시되는 댓글 수는 페이지나 게시물 자체에 직접 달린 답글을 의미하는 "top-level" 댓글만을 표시할 수도 있고, 또는 **모든** 중첩된 댓글의 수를 셀 수도 있습니다.

기본적으로 이것은 `true`입니다 - 후자의, 즉 모든 댓글의 수를 셉니다. 댓글 위젯의 이전 버전에서는 기본값이 `false`였습니다.

우리는 동작을 변경하여 **모든** 중첩 댓글의 수가 되도록 **countAll** 플래그를 true로 설정할 수 있습니다.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

카운트가 오직 최상위 댓글만 반영하도록 하려면, 플래그를 false로 설정합니다.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

이 동작은 현재 코드 변경 없이 사용자화할 수 없습니다.