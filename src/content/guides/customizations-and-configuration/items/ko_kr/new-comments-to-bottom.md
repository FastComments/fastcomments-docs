[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

기본적으로 새 라이브 댓글은 실시간으로 게시될 때 댓글 목록의 맨 위에 나타납니다.

이 옵션을 활성화하면 새 라이브 댓글이 대신 목록의 맨 아래에 추가됩니다. 이는 사용자가 댓글 스레드를 보는 동안 댓글이 실시간으로 게시될 때 댓글이 나타나는 방식에 영향을 줍니다.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

이 설정을 활성화하면:
- 다른 사용자가 게시한 새 라이브 댓글이 댓글 목록의 하단에 나타납니다
- 사용자는 새 댓글이 기존 댓글 아래에 실시간으로 나타나는 것을 보게 됩니다
- 이것은 라이브 댓글 업데이트에만 영향을 미치며 초기 페이지 로드에는 영향을 주지 않습니다
- 사용자가 토론을 따라갈 때 읽기 흐름을 유지하는 데 도움이 될 수 있습니다

이 설정은 새 라이브 댓글이 실시간으로 도착할 때 배치되는 위치에만 영향을 미친다는 점에 유의하세요. 페이지 로드 시의 초기 정렬 순서에는 영향을 주지 않습니다.