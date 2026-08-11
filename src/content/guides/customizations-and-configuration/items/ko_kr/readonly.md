[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

댓글 작성을 잠글 수 있어, readonly 플래그를 true로 설정하면 새로운 댓글이나 투표를 남길 수 없습니다.

댓글은 편집하거나 삭제할 수도 없습니다.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

코드를 사용하지 않고 위젯 커스터마이징 페이지에서 전체 도메인이나 페이지에 대해 맞춤 설정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; alt='위젯 커스터마이징 페이지에서 새로운 답글을 방지하는 설정으로, 도메인이나 페이지에 대한 스레드를 잠급니다'; title='댓글 스레드 읽기 전용으로 만들기' app-screenshot-end]

## Update!

2022년 11월부터, 스레드는 답글 영역 위의 점 세 개 메뉴를 통해 관리자와 모더레이터가 **실시간**으로 잠그거나 잠금 해제할 수 있습니다.

이렇게 하면 새로운 댓글을 방지하면서도 투표는 허용하고, 사용자가 원한다면 자신의 댓글을 삭제할 수 있게 됩니다. 반면 `readonly`는 이러한 기능을 허용하지 않습니다. 

이는 `Page` API의 `isClosed` 필드에 해당합니다.