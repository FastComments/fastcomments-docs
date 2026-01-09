[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자가 기본 문자 수 제한 내에서 원하는 만큼 여러 줄로 댓글을 입력할 수 있도록 허용합니다.

그러나 사용자가 한 줄의 텍스트만 입력하도록 제한해야 할 경우가 있습니다. 예시 사용 사례로는 온라인 입찰이나 실시간 채팅이 있으며, FastComments
는 이러한 용도로 사용될 수 있습니다.

다음과 같이 **useSingleLineCommentInput** 플래그를 활성화합니다:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

이 작업은 코드 없이도 수행할 수 있습니다. 위젯 사용자화 페이지에서 "Enable Single-Line Comment Input" 섹션을 참조하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

참고로, 각 페이지에 대한 댓글은 각 정렬 방향별로 미리 계산되므로 모든 정렬 방향이 동일한 성능을 보입니다.

---