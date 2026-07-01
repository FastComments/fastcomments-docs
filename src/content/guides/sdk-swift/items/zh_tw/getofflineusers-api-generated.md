Past commenters on the page who are NOT currently online. Sorted by displayName.  
頁面上過去的評論者（目前未在線）。依 displayName 排序。

Use this after exhausting /users/online to render a "Members" section.  
在耗盡 /users/online 後使用，以呈現「成員」區段。

Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName} index from afterName forward via $gt, no $skip cost.  
在 commenterName 上使用游標分頁：伺服器從 afterName 開始，透過 $gt 向前走訪部分 {tenantId, urlId, commenterName} 索引，無 $skip 成本。

## Parameters

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面 URL 識別碼（在伺服器端清理）。 |
| afterName | string | query | No | 游標：從先前的回應中傳入 nextAfterName。（可選） |
| afterUserId | string | query | No | 游標平手斷點：從先前的回應中傳入 nextAfterUserId。當 afterName 被設定時此為必填，以避免因名稱相同而遺漏條目。（可選） |

## Response

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍屬於測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 頁面 URL 識別碼（在伺服器端清理）。
let afterName = "afterName_example" // String | 游標：從先前的回應中傳入 nextAfterName。（可選）
let afterUserId = "afterUserId_example" // String | 游標平手斷點：從先前的回應中傳入 nextAfterUserId。當 afterName 被設定時此為必填，以避免因名稱相同而遺漏條目。（可選）

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetOfflineUsersOptions(afterName: afterName, afterUserId: afterUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]