A `BatchJob` is a long running task, like an import, an export, or a [page migration](#pages-migrations-post). Jobs are queued, then picked up and run in the background.

`processedCount` and `totalCount` show progress. Their unit depends on the `type`:

- `migrate-pages`: migrations finished, out of the migrations requested.
- `undo-page-migration`: records restored. There is no `totalCount`.
- `import`, `export` and other jobs: rows processed.

The structure for the `BatchJob` object is as follows:

[inline-code-attrs-start title = 'BatchJob Structure'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** For example migrate-pages, undo-page-migration, migrate-domains, import, export, users-export or copy-comments. **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** Total work, when it is known up front. **/
    totalCount?: number
    /** migrate-pages: one entry per finished migration, in request order. **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages: set once the job has finished (success or failed). Pass to POST /api/v1/pages/migrations/:undoId/undo within 90 days.
     * undo-page-migration: the migration being undone.
     **/
    undoId?: string
    /** undo-page-migration: set once the job has finished. **/
    pageMigrationUndoResult?: PageMigrationUndoResult
}

interface PageMigrationSuccess {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'success'
    commentsMigrated: number
}

interface PageMigrationFailure {
    from: string
    to: string
    type: 'urlId' | 'domain'
    status: 'failed'
    code: string
    reason: string
}

interface PageMigrationUndoResult {
    /** Records put back to their values from before the migration. **/
    restored: number
    /** Records left alone because they changed after the migration, or no longer exist. **/
    skipped: number
    /** Records the migration deleted while merging pages. These cannot be brought back. **/
    notRestorable: number
}
[inline-code-end]

#### Retries

If the server running a page migration job restarts, the job is put back in the queue and picks up after the last finished migration. A job is tried up to 3 times, after which its `status` is `failed`.
