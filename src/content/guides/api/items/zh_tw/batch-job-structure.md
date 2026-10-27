`BatchJob` 是一個長時間執行的任務，例如匯入、匯出，或是[頁面遷移](#pages-migrations-post)。工作會排入佇列，然後被取出並在背景執行。

`processedCount` 與 `totalCount` 顯示進度。它們的單位取決於 `type`：

- `migrate-pages`：已完成的遷移數量，對應於請求的遷移總數。
- `undo-page-migration`：已還原的記錄。此類型沒有 `totalCount`。
- `import`、`export` 以及其他工作：已處理的列數。

`BatchJob` 物件的結構如下：

[inline-code-attrs-start title = 'BatchJob 結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJob {
    id: string
    /** 例如 migrate-pages、undo-page-migration、migrate-domains、import、export、users-export 或 copy-comments。 **/
    type: string
    status: 'requested' | 'running' | 'success' | 'failed'
    createdAt: string
    completedAt?: string
    processedCount: number
    /** 總工作量，若事先已知的話。 **/
    totalCount?: number
    /** migrate-pages：每個已完成的遷移一筆記錄，依請求順序排列。 **/
    pageMigrationResults?: Array<PageMigrationSuccess | PageMigrationFailure>
    /**
     * migrate-pages：在工作完成（成功或失敗）後設定。於 90 天內傳遞至 POST /api/v1/pages/migrations/:undoId/undo。
     * undo-page-migration：正在還原的遷移。
     **/
    undoId?: string
    /** undo-page-migration：在工作完成後設定。 **/
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
    /** 記錄已恢復至遷移前的值。 **/
    restored: number
    /** 記錄因遷移後已變更或不再存在而未被處理。 **/
    skipped: number
    /** 記錄在合併頁面時被刪除的遷移。這些無法復原。 **/
    notRestorable: number
}
[inline-code-end]

#### 重試

如果執行頁面遷移工作的伺服器重新啟動，該工作會重新放回佇列，並在最後一次完成的遷移之後繼續執行。工作最多會嘗試 3 次，之後其 `status` 會變為 `failed`。

---