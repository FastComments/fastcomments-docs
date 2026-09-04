[api-resource-header-start name = 'AuditLog'; route = 'GET /api/v1/audit-logs'; creditsCost = 10; api-resource-header-end]

此 API 使用分頁功能，由 `skip`、`limit`、`before` 與 `after` 參數提供。AuditLogs 預設以 `100` 筆為一頁返回，最大 `limit` 為 `200`，依 `when` 與 `id` 排序。

每返回 `100` 筆日誌，會消耗 `1` 點信用。

預設情況下，您會收到 **最新項目優先** 的列表。如此，您可以從 `skip=0` 開始輪詢，持續分頁直到找到您已消耗的最後一筆記錄。

或者，您也可以將排序設為最舊優先，並持續分頁直到沒有更多記錄。

排序可透過將 `order` 設為 `ASC` 或 `DESC` 來完成。預設為 `DESC`。

可使用 `before` 與 `after`（以毫秒為單位的時間戳記）進行日期查詢。`before` 與 `after` 為**不含**的範圍，且任一參數皆可單獨使用。

## 找出某人的發生事件

每個事件都會記錄執行者（`username`、`userId`、`ip`）以及被執行的對象。`targetLabel` 是該對象的可讀標籤，例如 `jsmith (jsmith@example.com)`，而 `targetId` 為其 ID。當您知道某人的姓名或電子郵件但不知道其 ID 時，可使用 `target` 進行不區分大小寫的子字串匹配。

刪除操作會在事件發生時捕獲標籤，因此即使底層記錄已被移除，仍可辨識被刪除的使用者或審核員。

## 管理的租戶

如果您的租戶管理其他租戶，請將 `includeManagedTenants=true` 設定，以在單一回應中返回您租戶及其所管理的所有租戶的事件。每筆返回的日誌的 `tenantId` 會告訴您其來源租戶。

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
    limit?: number
    skip?: number
    before?: number
    after?: number
    /** Only events performed by this username. **/
    username?: string
    /** Only events from this IP address. **/
    ip?: string
    /** Only events of this type. **/
    crudType?: 'c' | 'r' | 'u' | 'd' | 'login'
    /** Only events for this resource, e.g. User or Moderator. **/
    resourceName?: string
    /** Only events whose affected object has this id. **/
    targetId?: string
    /** Case-insensitive substring match on the affected object's label. **/
    target?: string
    /** Also return events from tenants this tenant manages. **/
    includeManagedTenants?: boolean
}
[inline-code-end]

[inline-code-attrs-start title = 'AuditLog 回應結構'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface AuditLogsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-limit' | 'invalid-skip'
    /** Included on failure. **/
    reason?: string
    /** The logs! **/
    auditLogs: AuditLog[]
}
[inline-code-end]

---