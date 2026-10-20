[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

此路由会撤销一次 [`POST /api/v1/pages/migrations`](#pages-migrations-post) 作业中的所有迁移。使用 [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) 获取已完成迁移作业的 `undoId`。`undoId` 可在 90 天内使用，且只能使用一次。

撤销操作以批处理作业的形式运行，方式与迁移相同，响应中包含其 `batchJobId`。作业完成后，其 `pageMigrationUndoResult` 会报告：

- `restored`：记录被恢复到迁移前的值。
- `skipped`：记录因迁移后已更改或已不存在而未被处理。后续的编辑永不被覆盖。
- `notRestorable`：记录在合并两个页面或删除重复页面时被迁移删除的记录。这些记录无法恢复。

迁移按最新的先撤销，因此像 `a` 到 `b` 再到 `b` 到 `c` 的链会被逆转回 `a`。即使迁移作业在中途失败，也仍可撤销，它会逆转失败前所做的更改。

排队撤销不收取费用。作业完成后，每恢复 `1000` 条记录收取 `1` 积分，最低收取 `1` 积分。

[inline-code-attrs-start title = '撤销页面迁移 cURL 示例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '撤销页面迁移 请求结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '撤销页面迁移 响应结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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