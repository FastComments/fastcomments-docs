플러그인 설정 페이지는 **Site Administration > Plugins > Local plugins > FastComments**에 있습니다. 사용 가능한 옵션은 다음과 같습니다:

#### Tenant ID

귀하의 FastComments Tenant ID입니다. <a href="https://fastcomments.com/auth/my-account" target="_blank">FastComments 대시보드</a>의 계정 설정에서 확인할 수 있습니다.

#### API Secret

Secure SSO 모드에 필요한 API Secret 키입니다. <a href="https://fastcomments.com/auth/my-account/api-secret" target="_blank">My Account > API Secret</a>에서 확인할 수 있습니다.

#### SSO Mode

사용자 인증 방식을 선택합니다. 각 옵션에 대한 자세한 내용은 [SSO Modes](#moodle-sso-modes) 섹션을 참조하세요.

- **Secure** (권장) - 서버 측 HMAC-SHA256 서명 인증
- **Simple** - 서명 없는 클라이언트 측 사용자 데이터
- **None** - 익명 댓글 작성, Moodle 로그인 연동 없음

#### Page Contexts

댓글이 표시되는 위치를 제어합니다:

- **Course pages** - 강좌 메인 페이지의 댓글
- **Module/activity pages** - 개별 활동 및 리소스의 댓글
- **Both** - 모든 페이지 유형의 댓글

#### Commenting Style

댓글 사용 경험을 선택합니다. 각 모드의 스크린샷은 [Commenting Styles](#moodle-commenting-styles)를 참조하세요.

- **Comments** - 페이지 콘텐츠 아래의 표준 스레드형 댓글 위젯
- **Collab Chat** - 접속 상태 표시와 함께 인라인 텍스트 선택 토론
- **Both** - 댓글과 협업 채팅을 함께 활성화

#### CDN URL

FastComments CDN URL입니다. 기본값은 `https://cdn.fastcomments.com`입니다. 데이터가 EU 리전에 호스팅된 경우 EU CDN URL로 변경하세요.