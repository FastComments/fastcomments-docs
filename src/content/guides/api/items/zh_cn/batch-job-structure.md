A `BatchJob` 是一种长时间运行的任务，例如导入、导出或[页面迁移](#pages-migrations-post)。作业会被排队，然后被取出并在后台运行。

`processedCount` 和 `totalCount` 显示进度。它们的单位取决于 `type`：

- `migrate-pages`：已完成的迁移数，针对请求的迁移总数。
- `undo-page-migration`：已恢复的记录。没有 `totalCount`。
- `import`、`export` 以及其他作业：已处理的行数。

`BatchJob` 对象的结构如下：

[inline-code-attrs-start title = 'BatchJob 结构'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** 例如 migrate-pages、undo-page-migration、migrate-domains、import、export、users-export 或 copy-comments。 **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** 总工作量（如果事先已知）。 **/
    totalCount?: number
    /** migrate-pages：每个已完成的迁移对应一个条目，按请求顺序。 **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages：在作业完成（成功或失败）后设置。请在 90 天内将其传递给 POST /api/v1/pages/migrations/:undoId/undo。
     * undo-page-migration：正在撤销的迁移。
     **/
    undoId?: string
    /** undo-page-migration：在作业完成后设置。 **/
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
    /** 记录已恢复到迁移前的值。 **/
    restored: number
    /** 记录保持不变，因为它们在迁移后已更改，或已不存在。 **/
    skipped: number
    /** 记录在合并页面时被删除的迁移。这些记录无法恢复。 **/
    notRestorable: number
}
[inline-code-end]

#### 重试

如果运行页面迁移作业的服务器重启，作业会重新放回队列，并在上一次完成的迁移之后继续。作业最多会尝试 3 次，之后其 `status` 为 `failed`。

---