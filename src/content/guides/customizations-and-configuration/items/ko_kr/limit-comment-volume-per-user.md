---
기본적으로 각 사용자는 동일한 분에 최대 `5 comments`까지 제출할 수 있습니다.

이는 사용자 ID, 익명 사용자 ID 및 IP 주소(해시됨)로 추적됩니다.

코드를 작성하지 않고도 위젯 사용자 정의 페이지에서 맞춤 설정할 수 있습니다:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='위젯 사용자 정의 페이지의 분당 최대 댓글 수 필드, 기본값은 5로 설정되어 있습니다'; title='사용자당 댓글 양 제한' app-screenshot-end]

댓글 생성 API를 사용하는 경우, 사용자 원본 `ip` 주소를 백엔드에 요청에 포함시켜야 할 수 있습니다. 그 결과 속도 제한이 적용됩니다
사용자별로 적용되고 귀하의 계정 전체에 전역적으로 적용되지 않습니다.