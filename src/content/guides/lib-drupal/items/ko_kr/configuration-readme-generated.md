다음 위치로 이동하세요: **관리 > 구성 > 콘텐츠 > FastComments** (`/admin/config/content/fastcomments`).

### 설정

- **Tenant ID** (필수) - 귀하의 FastComments Tenant ID입니다. [설정 > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api))에서 확인하세요.
- **API Secret** - Secure SSO, 웹훅 검증 및 페이지 동기화에 필요합니다. [설정 > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api))에서 찾을 수 있습니다.
- **SSO Mode** - 싱글 사인온 통합:
  - **None** - SSO 없음, 사용자는 게스트로 댓글을 달거나 FastComments 계정을 생성합니다.
  - **Simple** - 서버 측 검증 없이 Drupal 사용자 정보(이름, 이메일, 아바타)를 FastComments로 전달합니다.
  - **Secure** - HMAC-SHA256 검증을 사용하여 Drupal 사용자를 FastComments에 안전하게 인증합니다(권장).
- **Commenting Style** - 표시할 위젯 유형:
  - **Live Comments** - 실시간 스레드형 댓글.
  - **Streaming Chat** - 라이브 채팅 인터페이스.
  - **Collab Chat** - 본문 영역에서의 협업 텍스트 선택 주석 기능.
  - **Collab Chat + Comments** - 협업 채팅과 일반 댓글 모두.
- **CDN URL** - FastComments CDN URL (기본값: `https://cdn.fastcomments.com`).
- **Site URL** - FastComments 사이트 URL (기본값: `https://fastcomments.com`).
- **Email notifications** - 새 댓글이 게시되면 콘텐츠 작성자에게 이메일을 전송합니다.

### 콘텐츠 유형에 댓글 추가

**구조 > 콘텐츠 유형 > [type] > 필드 관리**를 통해 콘텐츠 유형에 **FastComments** 필드를 추가하세요. 이 필드에는 상태 토글과 엔터티별 선택적 커스텀 식별자가 있습니다.

### EU 데이터 레지던시

EU 데이터 레지던시를 사용하려면 다음을 업데이트하세요:
- **CDN URL** 을 `https://cdn-eu.fastcomments.com` 로 변경
- **Site URL** 을 `https://eu.fastcomments.com` 로 변경