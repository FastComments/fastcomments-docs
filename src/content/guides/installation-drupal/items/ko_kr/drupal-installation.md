FastComments Drupal 모듈은 Drupal의 기본 내장 댓글 기능을 빠르고 실시간으로 작동하는 댓글 시스템으로 대체합니다. 이 모듈은 [drupal.org에 게시되어 있으며](https://www.drupal.org/project/fcom) Drupal 10 및 11과 호환됩니다.

설치 방법은 두 가지입니다.

## Composer로 설치

```
composer require drupal/fcom
drush en fastcomments
```

## 수동 설치

[drupal.org/project/fcom](https://www.drupal.org/project/fcom)에서 모듈을 다운로드한 후, 사이트의 `modules/custom/fastcomments/` 디렉터리에 넣습니다. 그런 다음 `drush en fastcomments` 명령으로 활성화하거나, 관리자 UI의 `Extend`(`/admin/modules`)에서 활성화할 수 있습니다.

<sup>참고!</sup> 이 모듈은 Drupal 코어(`user` 및 `field`)에만 의존합니다. 다른 Drupal 모듈이나 라이브러리는 필요하지 않습니다.

모듈을 활성화한 후, `Configuration` 섹션으로 이동하여 Tenant ID와 API Secret을 설정하세요.