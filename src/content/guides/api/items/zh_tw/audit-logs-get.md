[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

此 API 使用分頁，由 `skip`、`before` 與 `after` 參數提供。AuditLogs 會以每頁 `1000` 筆回傳，依 `when` 與 `id` 排序。

每取得 `1000` 筆日誌會花費 `10` 點額度。

預設會以 **最新項目在前** 的方式回傳清單。如此一來，您可以從 `skip=0` 開始輪詢，分頁直到找到您已消費的最後一筆紀錄。

或者，您可以將排序改為最舊優先，然後分頁直到沒有更多紀錄。

可透過將 `order` 設為 `ASC` 或 `DESC` 來設定排序。預設為 `ASC`。

可以使用帶毫秒的時間戳記透過 `before` 與 `after` 查詢日期。`before` 與 `after` 不包含邊界值。

[inline-code-attrs-start title = 'AuditLog cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    skip?: number
    before?: number
    after?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** 失敗時包含。 **/
    reason?: string
    /** 日誌！ **/
    auditLogs: AuditLog[]
}
[inline-code-end]