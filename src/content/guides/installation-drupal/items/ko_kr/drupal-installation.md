FastComments Drupal 모듈은 Drupal의 기본 댓글 기능을 빠르고 실시간 댓글 시스템으로 대체합니다. 이 모듈은 [drupal.org에 게시됨](https://www.drupal.org/project/fcom)이며 Drupal 10 및 11에서 작동합니다.

설치 방법은 두 가지가 있습니다.

## Composer로 설치

```
composer require drupal/fcom
drush en fastcomments
```

## 수동 설치

Download the module from [drupal.org/project/fcom](https://www.drupal.org/project/fcom) and place it in your site's `modules/custom/fastcomments/` directory. Then enable it with `drush en fastcomments`, or from the admin UI at `Extend` (`/admin/modules`).

<sup>참고!</sup> 이 모듈은 Drupal 코어 (`user` and `field`). 다른 Drupal 모듈이나 라이브러리는 필요하지 않습니다.

모듈이 활성화되면 `Configuration` 섹션으로 이동하여 Tenant ID와 API Secret을 설정하세요.