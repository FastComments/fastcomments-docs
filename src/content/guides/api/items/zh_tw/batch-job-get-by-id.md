[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs/:id'; creditsCost = 1; api-resource-header-end]

此路由會回傳單一批次作業的狀態、進度，且在完成後回傳結果。可用來追蹤由 [`POST /api/v1/pages/migrations`](#pages-migrations-post) 或 [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) 所啟動的作業。

當作業的 `status` 為 `success` 或 `failed` 時，即表示已完成。每次呼叫會消耗 `1` 點額度，建議每隔幾秒輪詢一次，而非持續不斷呼叫。

[inline-code-attrs-start title = 'BatchJob By Id cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs/my-batch-job-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob By Id 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob By Id 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title =已完成的頁面遷移作業範例; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
{
    "status": "success",
    "batchJob": {
        "id": "my-batch-job-id",
        "type": "migrate-pages",
        "status": "success",
        "createdAt": "2026-10-19T16:00:00.000Z",
        "completedAt": "2026-10-19T16:00:12.000Z",
        "processedCount": 2,
        "totalCount": 2,
        "pageMigrationResults": [
            {"from": "old-post-id", "to": "new-post-id", "type": "urlId", "status": "success", "commentsMigrated": 42},
            {"from": "old.example.com", "to": "new.example.com", "type": "domain", "status": "success", "commentsMigrated": 1250}
        ],
        "undoId": "my-undo-id"
    }
}
[inline-code-end]