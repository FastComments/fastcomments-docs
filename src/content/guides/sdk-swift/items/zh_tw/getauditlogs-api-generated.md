## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 | 要讀取稽核日誌的租戶。 |
| limit | integer | query | 否 | 要返回的日誌數量。最大 10000，預設為 1000。 |
| skip | integer | query | 否 | 要跳過的日誌數量，用於分頁。 |
| order | string | query | 否 | 事件時間的排序方向。預設為 DESC（最新的在前）。 |
| after | integer | query | 否 |  |
| before | integer | query | 否 |  |
| username | string | query | 否 | 僅返回此使用者名稱執行的事件。 |
| ip | string | query | 否 | 僅返回來自此 IP 位址的事件。 |
| crudType | string | query | 否 | 僅返回此類型的事件：c、r、u、d，或 login。 |
| resourceName | string | query | 否 | 僅返回此資源的事件，例如 User 或 Moderator。 |
| targetId | string | query | 否 | 僅返回受影響物件具有此 ID 的事件。 |
| target | string | query | 否 | 對受影響物件的標籤進行不區分大小寫的子字串匹配，例如使用者名稱或電子郵件。 |
| includeManagedTenants | boolean | query | 否 | 同時返回此租戶所管理的租戶的事件。 |

## 回應

返回：[`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetAuditLogsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getAuditLogs 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 要讀取稽核日誌的租戶。
let limit = 987 // Int | 要返回的日誌數量。最大 10000，預設為 1000。（可選）
let skip = 987 // Int | 要跳過的日誌數量，用於分頁。（可選）
let order = SORT_DIR() // SORTDIR | 事件時間的排序方向。預設為 DESC（最新的在前）。（可選）
let after = 987 // Int64 |  （可選）
let before = 987 // Int64 |  （可選）
let username = "username_example" // String | 僅返回此使用者名稱執行的事件。（可選）
let ip = "ip_example" // String | 僅返回來自此 IP 位址的事件。（可選）
let crudType = "crudType_example" // String | 僅返回此類型的事件：c、r、u、d，或 login。（可選）
let resourceName = "resourceName_example" // String | 僅返回此資源的事件，例如 User 或 Moderator。（可選）
let targetId = "targetId_example" // String | 僅返回受影響物件具有此 ID 的事件。（可選）
let target = "target_example" // String | 對受影響物件的標籤進行不區分大小寫的子字串匹配，例如使用者名稱或電子郵件。（可選）
let includeManagedTenants = true // Bool | 同時返回此租戶所管理的租戶的事件。（可選）

DefaultAPI.getAuditLogs(tenantId: tenantId, options: DefaultAPI.GetAuditLogsOptions(limit: limit, skip: skip, order: order, after: after, before: before, username: username, ip: ip, crudType: crudType, resourceName: resourceName, targetId: targetId, target: target, includeManagedTenants: includeManagedTenants)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]