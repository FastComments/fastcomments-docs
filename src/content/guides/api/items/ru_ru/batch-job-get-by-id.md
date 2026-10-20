[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs/:id'; creditsCost = 1; api-resource-header-end]

Этот маршрут возвращает статус, прогресс и, после завершения, результат одного пакетного задания. Используйте его, чтобы отслеживать задание, запущенное через [`POST /api/v1/pages/migrations`](#pages-migrations-post) или [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post).

Задание считается завершённым, когда его `status` равен `success` или `failed`. Каждый вызов стоит `1` кредит, поэтому опрашивайте каждые несколько секунд, а не постоянно.

[inline-code-attrs-start title = 'Пример cURL для BatchJob по ID'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs/my-batch-job-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Структура запроса BatchJob по ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Структура ответа BatchJob по ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Пример завершённого задания миграции страниц'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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