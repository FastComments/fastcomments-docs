이름은 도메인 마이그레이션 도구이지만, 개별 페이지 간에 댓글을 마이그레이션하는 데에도 사용할 수 있습니다.

`from` 및 `to` 입력 필드에 이전 및 새 [urlId](/guide-customizations-and-configuration.html#url-id) 값을 입력하기만 하면 됩니다.

### API를 통한 페이지 마이그레이션

여러 페이지를 한 번에 마이그레이션하려면 [`POST /api/v1/pages/migrations`](/guide-api.html#pages-migrations-post)를 사용하세요. 요청당 최대 100개의 `from` 및 `to` 쌍을 받아 배치 작업으로 실행합니다.

`type`이 `urlId`로 설정되면 `urlId`가 정확히 `from`과 일치하는 페이지만 이동되므로, `post-1`은 `post-10`과 일치하지 않습니다. 대시보드 도구에는 이 옵션이 없습니다.

각 API 마이그레이션은 90일 동안 [`POST /api/v1/pages/migrations/:undoId/undo`](/guide-api.html#pages-migrations-undo-post)를 통해 취소할 수 있습니다.