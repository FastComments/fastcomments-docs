[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

기본적으로 사용자가 댓글을 입력한 후 제출하기 전에 페이지를 새로 고치거나 탭을 닫거나 다른 페이지로 이동하면 초안이 조용히 사라집니다.

**warnOnUnsavedComment** 를 true 로 설정하면, 댓글 입력란이나 진행 중인 편집에 텍스트가 남아 있는 동안 페이지를 떠나기 전에 브라우저가 사용자에게 확인을 요청합니다. 댓글이 제출되면 텍스트가 지워지므로 프롬프트가 표시되지 않습니다.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = '저장되지 않은 댓글에 대한 경고'; code-example-end]

프롬프트는 브라우저 자체 대화 상자를 사용합니다. 최신 브라우저는 자체 문구를 표시하고 사용자 지정 텍스트를 무시하므로 메시지를 맞춤 설정할 수 없습니다.

이 옵션은 필요에 따라 작은 확장을 로드하므로, 이를 활성화하지 않은 사이트의 위젯에 아무런 영향을 주지 않습니다.