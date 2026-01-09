[related-parameter-start name = 'noNewRootComments'; type = 'boolean'; related-parameter-end]

Setting `noNewRootComments` to `true` will cause the widget to hide the root reply area, but still allow users to reply
하위 댓글에 답글을 달 수 있게 합니다. 예를 들어, 페이지 로드 시 조건부로 이 값을 설정하여 일부 사용자만 최상위 댓글을 남길 수 있도록 할 수 있습니다.

[code-example-start config = {noNewRootComments: true}; linesToHighlight = [6]; title = '최상위 댓글 작성 방지'; code-example-end]