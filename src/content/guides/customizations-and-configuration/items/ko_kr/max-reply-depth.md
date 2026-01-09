[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

기본적으로 FastComments는 답글의 중첩을 무제한으로 허용하여 사용자가 답글에 계속해서 답글을 달 수 있는 스레드 구조를 만듭니다.

maxReplyDepth 옵션은 답글 스레드가 얼마나 깊게 중첩될 수 있는지를 제한할 수 있게 해줍니다. 최대 깊이에 도달하면 해당 수준의 댓글에는 더 이상 답글 버튼이 표시되지 않습니다.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

With maxReplyDepth set to 2:
- 사용자는 최상위 수준(깊이 0)에 댓글을 달 수 있습니다
- 사용자는 최상위 댓글에 답글을 달 수 있습니다 (깊이 1)
- 사용자는 그 답글들에 다시 답글을 달 수 있습니다 (깊이 2)
- 깊이 2를 넘어서 추가 답글은 허용되지 않습니다

Setting to 1 would only allow replies to top-level comments, creating a flatter discussion structure.

maxReplyDepth를 0으로 설정하면 모든 답글이 비활성화되어 최상위 댓글만 허용됩니다. 지정하지 않으면 답글은 무제한으로 중첩될 수 있습니다.