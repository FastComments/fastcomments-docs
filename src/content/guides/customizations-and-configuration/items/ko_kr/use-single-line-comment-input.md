[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

기본적으로 FastComments는 사용자가 원하는 만큼 여러 줄의 댓글을 입력할 수 있도록 허용하며, 기본 문자 제한까지 가능합니다.

그러나 사용자가 한 줄의 텍스트만 입력하도록 제한하는 것이 바람직할 수 있습니다. 예시 사용 사례로는 온라인 입찰이나 실시간 채팅 등이 있으며, FastComments를 사용할 수 있습니다.

다음과 같이 **useSingleLineCommentInput** 플래그를 활성화합니다:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

코드를 사용하지 않고도 설정할 수 있습니다. 위젯 맞춤 설정 페이지에서 "Enable Single-Line Comment Input" 섹션을 확인하세요.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='위젯 맞춤 설정 페이지에서 단일 줄 댓글 입력 체크박스가 켜져 있어 입력이 한 줄로 제한됩니다'; title='단일 줄 댓글 입력 활성화' app-screenshot-end]

각 정렬 방향에 대한 페이지별 댓글이 사전 계산되므로, 모든 정렬 방향에서 동일한 성능을 제공합니다.