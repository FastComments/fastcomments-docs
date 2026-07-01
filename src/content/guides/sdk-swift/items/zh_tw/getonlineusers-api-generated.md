---
目前在線的頁面檢視者：指目前已訂閱該頁面的 websocket 會話使用者。  
返回 anonCount + totalCount（全房間訂閱者，包括我們未列舉的匿名檢視者）。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（在伺服器端清理）。 |
| afterName | string | query | No | 游標：傳入先前回應中的 nextAfterName。 |
| afterUserId | string | query | No | 游標平手時的分割條件：傳入先前回應中的 nextAfterUserId。當設定 afterName 時必填，以避免因名稱相同而遺漏項目。 |

## 回應

返回: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 範例

[inline-code-attrs-start title = '取得線上使用者 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍處於測試階段。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 頁面 URL 識別碼（在伺服器端清理）。
let afterName = "afterName_example" // String | 游標：傳入先前回應中的 nextAfterName。（可選）
let afterUserId = "afterUserId_example" // String | 游標平手時的分割條件：傳入先前回應中的 nextAfterUserId。當設定 afterName 時必填，以避免因名稱相同而遺漏項目。（可選）

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---