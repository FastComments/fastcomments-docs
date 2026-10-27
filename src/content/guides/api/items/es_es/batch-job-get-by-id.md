[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs/:id'; creditsCost = 1; api-resource-header-end]

Esta ruta devuelve el estado, el progreso y, una vez que ha finalizado, el resultado de un trabajo por lotes. Úsala para seguir un trabajo iniciado por [`POST /api/v1/pages/migrations`](#pages-migrations-post) o [`POST /api/v1/pages/migrations/:undoId/undo`](#pages-migrations-undo-post).

Un trabajo se considera terminado cuando su `status` es `success` o `failed`. Cada llamada cuesta `1` crédito, así que realiza sondeos cada pocos segundos en lugar de continuamente.

[inline-code-attrs-start title = 'Ejemplo cURL de BatchJob por ID'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs/my-batch-job-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de solicitud de BatchJob por ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-end]

[inline-code-attrs-start title = 'Estructura de respuesta de BatchJob por ID'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-start]
interface BatchJobResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'internal'
    /** Included on failure. **/
    reason?: string
    batchJob?: BatchJob
}
[inline-end]

[inline-code-attrs-start title = 'Ejemplo de trabajo de migración de página finalizado'; type = 'json'; isFunctional = false; inline-code-attrs-end]
[inline-start]
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
        "undoId": "my-undo-id-undo"
    }
}
[inline-end]