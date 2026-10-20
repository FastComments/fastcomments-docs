[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs/:id'; creditsCost = 1; api-resource-header-end]

Diese Route gibt den Status, den Fortschritt und, sobald sie abgeschlossen ist, das Ergebnis eines Batch-Jobs zurück. Verwenden Sie sie, um einen Job zu verfolgen, der durch [`POST /api/v1/pages/migrations`](#pages-migrations-post) oder [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post) gestartet wurde.

Ein Job ist abgeschlossen, wenn sein `status` `success` oder `failed` ist. Jeder Aufruf kostet `1` Credit, daher sollten Sie alle paar Sekunden abfragen statt kontinuierlich.

[inline-code-attrs-start title = 'BatchJob nach ID cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs/my-batch-job-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob nach ID Anforderungsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob nach ID Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobResponse {
    status: 'success' | 'failed'
    /** Bei einem Fehler enthalten. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'internal'
    /** Bei einem Fehler enthalten. **/
    reason?: string
    batchJob?: BatchJob
}
[inline-code-end]

[inline-code-attrs-start title = 'Beispiel für abgeschlossenen Seiten-Migrations-Job'; type = 'json'; isFunctional = false; inline-code-attrs-end]
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