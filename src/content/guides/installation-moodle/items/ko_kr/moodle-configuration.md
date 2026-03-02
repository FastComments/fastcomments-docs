플러그인 설정 페이지는 **사이트 관리 > 플러그인 > 로컬 플러그인 > FastComments**에 있습니다. 사용 가능한 옵션은 다음과 같습니다:

#### 테넌트 ID

FastComments 테넌트 ID입니다. 계정 설정의 <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 대시보드</a>에서 확인하세요.

#### API Secret

Secure SSO 모드에 필요한 API Secret 키입니다. <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">내 계정 > API Secret</a>에서 찾으세요.

#### SSO 모드

사용자 인증 방식을 선택하세요. 각 옵션에 대한 자세한 내용은 [SSO 모드](#items-moodle-sso-modes) 섹션을 참조하세요.

- **Secure** (권장) - 서버 측 HMAC-SHA256 서명 인증
- **Simple** - 서명 없는 클라이언트 측 사용자 데이터
- **None** - 익명 댓글 작성, Moodle 로그인 통합 없음

#### 페이지 컨텍스트

댓글이 표시되는 위치를 제어합니다:

- **Course pages** - 코스 메인 페이지에 댓글 표시
- **Module/activity pages** - 개별 활동 및 리소스에 대한 댓글
- **Both** - 모든 페이지 유형에 댓글 표시

#### 댓글 스타일

댓글 경험을 선택하세요. 각 모드의 스크린샷은 [댓글 스타일](#items-moodle-commenting-styles)을 참조하세요.

- **Comments** - 페이지 콘텐츠 아래의 표준 스레드형 댓글 위젯
- **Collab Chat** - 선택한 텍스트에 대한 인라인 토론(참여자 표시 포함)
- **Both** - 댓글과 Collab Chat 모두 활성화

#### CDN URL

FastComments CDN URL입니다. 기본값은 `https://cdn.fastcomments.com`입니다. 데이터가 EU 리전에 호스팅되어 있는 경우 EU CDN URL로 변경하세요.