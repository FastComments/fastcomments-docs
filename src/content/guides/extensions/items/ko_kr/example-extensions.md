FastComments에서는 동일한 API를 사용하여 자체 확장 기능을 작성합니다. 다음 엔드포인트에서 이러한 확장 기능의 압축되지 않은 코드를
볼 수 있습니다:

#### 다크 모드

다크 모드 확장은 페이지가 "dark"로 감지될 때 조건부로 로드됩니다. 이 확장은 단순히
댓글 위젯에 일부 CSS를 추가합니다. 이렇게 하면 필요하지 않을 때 다크 모드 CSS를 로드할 필요가 없습니다.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### 모더레이션

현재 사용자가 모더레이터인 경우, 모더레이션 확장을 사용합니다.

이는 클릭 기반 이벤트 리스너 추가, API 요청 수행, 댓글 메뉴 및 댓글 영역에 항목을 추가하는 좋은 예입니다.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### 라이브 채팅

라이브 채팅 확장(다른 구성 및 스타일과 결합하여)은 댓글 위젯을 라이브 채팅
구성 요소로 전환합니다.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js