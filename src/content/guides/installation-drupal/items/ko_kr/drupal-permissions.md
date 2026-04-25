이 모듈은 `People > Permissions`에서 역할별로 할당할 수 있는 세 가지 Drupal 권한을 추가합니다.

- **Administer FastComments** - `/admin/config/content/fastcomments`에 있는 FastComments 설정 양식에 대한 액세스.
- **View FastComments** - 댓글 위젯을 보려면 필요합니다. 이 권한이 없으면 위젯이 렌더링되지 않습니다.
- **Toggle FastComments** - 필드 위젯을 사용하여 사용자가 엔터티별로 댓글을 활성화하거나 비활성화할 수 있도록 허용합니다.

기본적으로 `administer site configuration` 권한을 가진 사용자만 FastComments 설정을 변경할 수 있습니다. 방문자가 위젯을 보게 하려면 익명 사용자 및 인증된 사용자에게 `View FastComments` 권한을 부여하세요.