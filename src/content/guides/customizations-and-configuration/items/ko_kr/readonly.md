[related-parameter-start name = 'readonly'; type = 'boolean'; related-parameter-end]

readonly 플래그를 true로 설정하면 새 댓글이나 투표를 남길 수 없도록 댓글 달기를 잠글 수 있습니다.

댓글도 편집하거나 삭제할 수 없습니다.

[code-example-start config = {readonly: true}; linesToHighlight = [6]; title = 'Making The Comment Thread Readonly'; code-example-end]

코드를 사용하지 않고도 위젯 맞춤 설정 페이지에서 도메인 전체 또는 페이지별로 이 설정을 변경할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.prevent-new-replies'; title='Making The Comment Thread Readonly' app-screenshot-end]

## 업데이트!

2022년 11월부터는 관리자가 답글 입력란 위의 점 3개 메뉴를 통해 스레드를 **라이브로** 잠그거나 잠금을 해제할 수 있습니다.

이렇게 하면 새 댓글은 차단되지만 투표는 계속 허용되고 사용자가 원할 경우 자신의 댓글을 삭제할 수도 있습니다. 반면 `readonly`는 이러한 것들을 허용하지 않습니다. 

이것은 `Page` API의 `isClosed` 필드에 해당합니다.

---