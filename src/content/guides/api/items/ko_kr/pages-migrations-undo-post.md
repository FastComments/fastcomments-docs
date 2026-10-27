[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

이 라우트는 하나의 [`POST /api/v1/pages/migrations`](#pages-migrations-post) 작업에서 발생한 모든 마이그레이션을 되돌립니다. 완료된 마이그레이션 작업에서 `undoId`를 [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) 로 가져옵니다. `undoId`는 90일 동안 사용할 수 있으며 한 번만 사용할 수 있습니다.

undo는 마이그레이션과 동일하게 배치 작업으로 실행되며, 응답에 `batchJobId`가 포함됩니다. 작업이 완료되면 `pageMigrationUndoResult`가 다음을 보고합니다:

- `restored`: 마이그레이션 이전 값으로 복원된 레코드.
- `skipped`: 마이그레이션 후 변경되었거나 더 이상 존재하지 않아 그대로 남은 레코드. 이후 편집은 절대 덮어쓰지 않습니다.
- `notRestorable`: 두 페이지를 병합하거나 중복 페이지를 제거하는 과정에서 마이그레이션이 삭제된 레코드. 복구할 수 없습니다.

마이그레이션은 최신 순으로 역순으로 되돌리며, 따라서 `a`에서 `b`로, 그 다음 `b`에서 `c`로의 체인은 `a`로 되돌아갑니다. 중간에 실패한 마이그레이션 작업도 undo가 가능하며, 실패 전까지 적용된 변경을 되돌립니다.

undo를 큐에 넣는 데는 비용이 들지 않습니다. 작업이 완료되면 복원된 레코드 1,000개당 `1` 크레딧이 차감되며 최소 `1` 크레딧이 부과됩니다.

[inline-code-attrs-start title = '페이지 마이그레이션 되돌리기 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '페이지 마이그레이션 되돌리기 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '페이지 마이그레이션 되돌리기 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * 실패 시 포함됩니다.
     * undo-not-found (404): 해당 undoId를 가진 마이그레이션이 없거나 90일 이상 지난 경우.
     * already-undone (409) 및 undo-in-progress (409): 마이그레이션이 이미 되돌려졌거나, 해당 undo가 대기 중이거나 진행 중입니다.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** 실패 시 포함됩니다. **/
    reason?: string
    /** 진행 상황 및 결과를 확인하려면 GET /api/v1/batch-jobs/:id에 전달하십시오. **/
    batchJobId?: string
}
[inline-code-end]