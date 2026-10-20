[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

此路由會撤銷單一 [`POST /api/v1/pages/migrations`](#pages-migrations-post) 作業中的所有遷移。可使用 [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id) 取得已完成遷移作業的 `undoId`。`undoId` 可在 90 天內使用，且只能使用一次。

撤銷會以批次作業的方式執行，與遷移相同，回應中會包含其 `batchJobId`。作業完成後，其 `pageMigrationUndoResult` 會回報：

- `restored`：將記錄還原至遷移前的值。
- `skipped`：因記錄在遷移後已變更或已不存在而未被處理。之後的編輯不會被覆寫。
- `notRestorable`：在合併兩個頁面或移除重複頁面回應時被遷移刪除的記錄。這些記錄無法復原。

遷移會以最新的優先順序撤銷，因此像 `a` 到 `b` 再到 `c` 的鏈結會被逆向回到 `a`。即使遷移作業在途中失敗，也仍可撤銷，會還原失敗前所做的變更。

排隊執行撤銷不會產生費用。作業完成後，依每恢復 `1000` 筆記錄收取 `1` 點信用，最低收取 `1` 點。

[inline-code-attrs-start title = '撤銷頁面遷移 cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = '撤銷頁面遷移請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = '撤銷頁面遷移回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * 失敗時包含此欄位。
     * undo-not-found (404)：沒有遷移對應此 undoId，或該 undoId 已超過 90 天。
     * already-undone (409) 與 undo-in-progress (409)：遷移已被撤銷，或撤銷作業已排隊或正在執行。
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** 失敗時包含此欄位。 **/
    reason?: string
    /** 傳遞給 GET /api/v1/batch-jobs/:id 以取得進度與結果。 **/
    batchJobId?: string
}
[inline-code-end]