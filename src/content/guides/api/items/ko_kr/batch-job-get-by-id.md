[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs/:id'; creditsCost = 1; api-resource-header-end]

이 엔드포인트는 단일 배치 작업의 상태, 진행 상황 및 작업이 완료된 경우 결과를 반환합니다. [`POST /api/v1/pages/migrations`](#pages-migrations-post) 또는 [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) 로 시작된 작업을 추적하는 데 사용하십시오.

작업은 `status`가 `success` 또는 `failed`일 때 완료됩니다. 각 호출은 `1` 크레딧이 소모되므로, 지속적으로 호출하기보다 몇 초 간격으로 폴링하십시오.

[inline-code-attrs-start title = 'BatchJob ID별 cURL 예시'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs/my-batch-job-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob ID별 요청 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob ID별 응답 구조'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'internal'
    /** Included on failure. **/
    reason?: string
    batchJob?: BatchJob
}
[inline-code-end]

[inline-code-attrs-start title = '완료된 페이지 마이그레이션 작업 예시'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "batchJob": {
        "id": "my-batch-job-id",
        "type": "migrate-pages",
        "status": "success",
        "createdAt": "2026-10-19T16:00:00.000Z",
        "completedAt": "2026-10-19T16:00:12.000Z",
        "processedCount":  ": 2,
        "totalCount": 2,
        "pageMigrationResults": [
            {"from": "old-post-id", "to": "new-post-id", "type": "urlId", "status": "success", "commentsMigrated": 42},
            {"from": "old.example.com", "to": "new.example.com", "type": "domain", "status": "success", "commentsMigrated": 1250}
        ],
        "undoId": "my-undo-id"
    }
}
[inline-code-end]