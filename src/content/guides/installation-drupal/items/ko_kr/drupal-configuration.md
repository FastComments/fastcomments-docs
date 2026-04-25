All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## 필수

- **Tenant ID** - 귀하의 FastComments Tenant ID. 다음 위치에서 찾을 수 있습니다: [설정 > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
- **API Secret** - Secure SSO, 웹훅 검증, 페이지 동기화에 필요합니다. [설정 > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api))에서 찾을 수 있습니다.

## 댓글 스타일

사이트에서 사람들이 대화하는 방식에 맞는 위젯을 선택하세요.

- **Live Comments** - 실시간 스레드형 댓글입니다.
- **Streaming Chat** - 라이브 채팅 인터페이스로 이벤트 및 라이브스트림에 적합합니다.
- **Collab Chat** - 메인 콘텐츠 영역에서 텍스트 선택 주석 기능입니다. 방문자가 텍스트를 강조 표시하고 맥락에서 토론을 시작합니다.
- **Collab Chat + Comments** - 동일한 페이지에서 collab chat과 표준 댓글을 모두 표시합니다.

## SSO Mode

- **None** - SSO 없음. 사용자는 게스트로 댓글을 달거나 FastComments 계정을 생성합니다.
- **Simple** - 서버 측 검증 없이 Drupal 사용자 정보(이름, 이메일, 아바타)를 FastComments로 전달합니다.
- **Secure** - HMAC-SHA256을 사용하여 Drupal 사용자를 FastComments와 검증합니다. API Secret이 구성된 경우 권장됩니다.

자세한 내용은 `Single Sign-On (SSO)` 섹션을 참조하세요.

## 기타 설정

- **CDN URL** - 기본값은 `https://cdn.fastcomments.com`입니다.
- **Site URL** - 기본값은 `https://fastcomments.com`입니다.
- **Email notifications** - 새 댓글이 해당 콘텐츠에 게시되면 콘텐츠 작성자에게 이메일을 보냅니다.

EU 데이터 레지던시의 경우 `EU Data Residency` 섹션을 참조하세요.

---