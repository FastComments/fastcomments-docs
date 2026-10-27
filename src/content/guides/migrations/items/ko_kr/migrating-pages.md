---
이 도구의 이름은 Domain Migration tool이지만, 개별 페이지의 댓글을 마이그레이션하는 데에도 사용할 수 있습니다.

단순히 `from` 및 `to` 입력 필드에 기존 및 새로운 [urlId](/guide-customizations-and-configuration.html#url-id) 값을 입력하십시오.

### Migrating Pages via API

여러 페이지를 한 번에 마이그레이션하려면 [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post)를 사용하십시오. 이 요청은 한 번에 최대 100개의 `from` 및 `to` 쌍을 받아 배치 작업으로 실행합니다.

`type`을 `urlId`로 설정하면, `urlId`가 정확히 `from`인 페이지만 이동되므로 `post-1`은 `post-10`과 일치하지 않습니다. 대시보드 도구에는 이 옵션이 없습니다.

각 API 마이그레이션은 [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post)를 사용하여 90일 동안 되돌릴 수 있습니다.

---