[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

`noNewRootComments`를 `true`로 설정하면 위젯이 루트 댓글 작성 영역을 숨기지만, 사용자가 여전히 자식 댓글에는 답글을 달 수 있습니다. 예를 들어, 페이지 로드 시 조건부로 이 값을 설정하여 일부 사용자만 최상위 댓글을 작성할 수 있도록 할 수 있습니다.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = '새 루트 댓글 방지'; code-example-end]