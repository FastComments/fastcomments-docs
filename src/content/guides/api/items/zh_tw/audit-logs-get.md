[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

此 API 使用分頁，由 `skip`、`limit`、`before` 與 `after` 參數提供。AuditLogs 預設以 `1000` 筆為一頁返回，最大 `limit` 為 `10000`，依 `when` 與 `id` 排序。由於此端點通常用於一次性匯出歷史記錄，而非互動式分頁，頁面會相當大。

每返回 `100` 筆日誌會消耗 `1` 點信用。

預設情況下，您會收到 **最新項目優先** 的列表。如此，您可以從 `skip=0` 開始輪詢，持續分頁直到找到您已消耗的最後一筆記錄。

或者，您也可以將排序設為最舊優先，並持續分頁直到沒有更多記錄。

可透過將 `order` 設為 `ASC` 或 `DESC` 來排序。預設為 `DESC`。

可使用 `before` 與 `after`（以毫秒為單位的時間戳記）進行日期查詢。`before` 與 `after` 為**不含等於**的範圍，且任一參數皆可單獨使用。

## 找出某人的發生事件

每個事件都會記錄執行者（`username`、`userId`、`ip`）以及被執行的對象。`targetLabel` 為該對象的可讀標籤，例如 `jsmith (jsmith@example.com)`，`targetId` 為其 ID。當您知道某人的姓名或電子郵件但不知道其 ID 時，可使用 `target` 進行不區分大小寫的子字串匹配。

刪除操作會在事件發生時捕獲標籤，因此即使底層記錄已被移除，仍可辨識被刪除的使用者或審核員。

## 管理的租戶

如果您的租戶管理其他租戶，請將 `includeManagedTenants=true`，以在單一回應中返回您租戶及其管理的所有租戶的事件。每筆返回的日誌的 `tenantId` 會告訴您其來源租戶。

[inline-code-attrs-start title = 'AuditLog cURL 範例'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/audit-logs?tenantId=demo&API_KEY=DEMO_API_SECRET&skip=0&order=ASC&before=123&after=456'
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 請求結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsRequestQueryParams {
    tenantId: string
    API_KEY: string
    order?: 'ASC' | 'DESC'
    /** 最大 10000。預設為 1000。 **/
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** 僅限此使用者名稱執行的事件。 **/
    username?: string
    /** 僅限此 IP 位址的事件。 **/
    ip?: string
    /** 僅限此類型的事件。 **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** 僅限此資源的事件，例如 User 或 Moderator。 **/
    resourceName?: string
    /** 僅限受影響物件具有此 ID 的事件。 **/
    targetId?: string
    /** 對受影響物件的標籤進行不區分大小寫的子字串匹配。 **/
    target?: string
    /** 也返回此租戶所管理的租戶的事件。 **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** 失敗時包含。 **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** 失敗時包含。 **/
    reason?: string
    /** 日誌！ **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---