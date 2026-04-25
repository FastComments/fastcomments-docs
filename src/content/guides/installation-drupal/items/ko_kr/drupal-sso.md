FastComments는 SSO를 통해 Drupal의 사용자 시스템과 통합됩니다. 사용자는 Drupal 사이트에 로그인하면 모듈이 자동으로 그들의 신원을 FastComments로 전달합니다. 추가 계정을 만들 필요도, 초기 동기화를 실행할 필요도 없습니다.

The module supports three SSO modes, set under `Administration > Configuration > Content > FastComments`.

### 없음

SSO 없음. 사용자는 게스트로 댓글을 달거나 FastComments 계정을 생성합니다. 사이트가 공개적이고 댓글을 Drupal 사용자와 연동할 필요가 없을 때 사용하세요.

### 간단

서버 측 검증 없이 Drupal 사용자의 이름, 이메일, 아바타를 FastComments로 전달합니다. API Secret이 필요하지 않습니다. 내부용이나 위험이 낮은 사이트에 적합합니다.

### 보안 (권장)

[HMAC-SHA256](https://en.wikipedia.org/wiki/HMAC)을 사용하여 각 사용자의 신원을 FastComments와 함께 확인합니다. API Secret이 구성되어 있을 때 이 모드를 사용하면 되며, 방문자가 다른 사용자를 사칭하는 것을 방지하는 유일한 모드입니다.

사용자가 댓글 스레드를 볼 때마다 사용자 신원이 FastComments로 전달됩니다. 실행해야 하는 초기 동기화나 지속적 동기화가 없습니다.

<sup>(Optional)</sup> 관리자 계정을 [Users & Administrators](https://fastcomments.com/auth/my-account/users)에 추가하고 중재자들을 [Comment Moderators](https://fastcomments.com/auth/my-account/moderate-comments/moderators)에 추가하면 사용성 향상과 중재자 통계 추적 활성화에 도움이 됩니다.

SSO 작동 방식에 대한 자세한 내용은 커스터마이제이션 문서의 [SSO 섹션](/guide-customizations-and-configuration.html#sso)을 참조하세요.