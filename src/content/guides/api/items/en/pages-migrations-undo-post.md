[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

This route reverses every migration from one [`POST /api/v1/pages/migrations`](#pages-migrations-post) job. Get the `undoId` from the finished migration job with [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). An `undoId` can be used for 90 days, and only once.

The undo runs as a batch job, the same as the migration, and the response contains its `batchJobId`. Once the job is done, its `pageMigrationUndoResult` reports:

- `restored`: records put back to their values from before the migration.
- `skipped`: records left alone because they changed after the migration, or no longer exist. A later edit is never overwritten.
- `notRestorable`: records the migration deleted while merging two pages, or removing duplicate page reacts. These cannot be brought back.

Migrations are reversed newest first, so a chain like `a` to `b` followed by `b` to `c` is unwound back to `a`. A migration job that failed partway can still be undone, which reverses the changes it made before failing.

Queueing an undo costs nothing. When the job finishes it costs `1` credit per `1000` records restored, with a minimum of `1`.

[inline-code-attrs-start title = 'Undo Page Migrations cURL Example'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Undo Page Migrations Request Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Undo Page Migrations Response Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Included on failure.
     * undo-not-found (404): no migration has this undoId, or it is older than 90 days.
     * already-undone (409) and undo-in-progress (409): the migration has been undone, or an undo of it is queued or running.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Included on failure. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]
