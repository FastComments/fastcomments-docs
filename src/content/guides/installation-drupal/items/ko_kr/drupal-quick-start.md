---
이것은 Drupal 지침의 "요약(TL;DR)" 버전입니다.

1. `composer require drupal/fcom`로 모듈을 설치하거나 `modules/custom/fastcomments/`에 넣으세요.
2. `drush en fastcomments`로 활성화하거나 관리 UI의 `/admin/modules`에서 활성화하세요.
3. `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`)로 이동하세요.
4. [설정 > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api))에서 Tenant ID와 API Secret을 입력하세요.
5. 원하는 콘텐츠 유형에 `FastComments` 필드를 `Structure > Content types > [type] > Manage fields`에서 추가하세요.

모듈은 [drupal.org/project/fcom](https://www.drupal.org/project/fcom)에 게시되어 있습니다.

---