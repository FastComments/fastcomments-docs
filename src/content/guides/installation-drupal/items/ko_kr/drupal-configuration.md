All settings live under `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).

## 필수

- **Tenant ID** - FastComments 테넌트 ID입니다. [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api))에서 찾을 수 있습니다.
- **API Secret** - Secure SSO, 웹훅 검증 및 페이지 동기화에 필요합니다. [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api))에서 찾을 수 있습니다.

## API Secret 저장

모듈은 값이 있는 다음 소스 중 첫 번째에서 API Secret을 읽습니다:

1. **Key module** - [Key](https://www.drupal.org/project/key)를 설치하고, *Authentication* 키를 생성한 뒤 설정 양식의 **API Secret key**에서 선택합니다. 키는 환경 변수나 웹 루트 외부 파일로 백업될 수 있습니다.
2. **settings.php** - `settings.php` 또는 `settings.local.php`에 `$config['fastcomments.settings']['api_secret'] = 'your-secret';`를 추가합니다. 비밀은 내보낸 구성에 절대 포함되지 않습니다.
3. **설정 양식** - **API Secret** 필드에 입력합니다. 이 값은 `fastcomments.settings`에 저장되어 구성 내보내기에 포함됩니다. Drupal 상태 보고서는 이것이 활성 소스일 때 경고합니다.

양식은 어떤 소스가 활성화되어 있는지 표시합니다. 필드를 비워두면 저장된 비밀이 유지됩니다. 비밀을 Key 모듈이나 `settings.php`로 이동한 후 **Remove the API Secret stored in configuration**을 선택하면 구성에 저장된 비밀을 삭제합니다.

## 댓글 스타일

사이트에서 사용자가 대화하는 방식을 가장 잘 반영하는 위젯을 선택하세요.

- **Live Comments** - 실시간 스레드형 댓글.
- **Streaming Chat** - 실시간 채팅 인터페이스로, 이벤트 및 라이브 스트림에 적합합니다.
- **Collab Chat** - 메인 콘텐츠 영역에서 텍스트 선택 주석을 달 수 있습니다. 방문자는 텍스트를 강조하고 해당 맥락에서 토론을 시작합니다.
- **Collab Chat + Comments** - 동일 페이지에 협업 채팅과 일반 댓글을 모두 제공합니다.

## SSO 모드

- **None** - SSO 없음. 사용자는 게스트로 댓글을 달거나 FastComments 계정을 생성합니다.
- **Simple** - 서버 측 검증 없이 Drupal 사용자 정보(이름, 이메일, 아바타)를 FastComments에 전달합니다.
- **Secure** - HMAC-SHA256을 사용해 Drupal 사용자를 FastComments와 검증합니다. API Secret이 구성된 경우 권장됩니다.

`Single Sign-On (SSO)` 섹션을 참조하세요.

## 기타 설정

- **CDN URL** - 기본값은 `https://cdn.fastcomments.com`입니다.
- **Site URL** - 기본값은 `https://fastcomments.com`입니다.
- **Email notifications** - 새로운 댓글이 게시될 때 콘텐츠 작성자에게 이메일을 보냅니다.

EU 데이터 거주에 대해서는 `EU Data Residency` 섹션을 참조하세요.