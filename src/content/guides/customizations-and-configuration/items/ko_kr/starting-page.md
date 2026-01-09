[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

댓글을 가져오고 렌더링할 때, 댓글 위젯은 어느 페이지에서 시작할지 알아야 합니다. 기본적으로는 시작 페이지로
첫 번째 페이지만 렌더링합니다.

원한다면, 렌더링할 정확한 페이지를 설정 *startingPage*로 댓글 위젯에 전달할 수 있습니다.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

페이지 번호는 0부터 시작한다는 점에 유의하세요. 따라서 위의 예제는 두 번째 페이지를 렌더링합니다.