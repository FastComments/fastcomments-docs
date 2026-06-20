目前在線上檢視某頁面的使用者：其 websocket 會話目前已訂閱該頁面的人員。
回傳 anonCount + totalCount（房間範圍的訂閱者，包括我們不逐一列出的匿名檢視者）。

## 參數

| 名稱 | 類型 | 位置 | 是否必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別符（伺服器端已清理）。 |
| afterName | string | query | 否 | 分頁游標：從前一個回應傳遞 nextAfterName。 |
| afterUserId | string | query | 否 | 游標決勝：從前一個回應傳遞 nextAfterUserId。當設定 afterName 時需提供，以免姓名相同導致條目遺失。 |

## 回應

回傳：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOnlineResponse.swift)

## 範例

[inline-code-attrs-start title = 'getOnlineUsers 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 頁面 URL 識別符（伺服器端已清理）。
let afterName = "afterName_example" // String | 游標：從先前回應傳遞 nextAfterName。（可選）
let afterUserId = "afterUserId_example" // String | 游標決勝：從先前回應傳遞 nextAfterUserId。當設定 afterName 時需提供，以免姓名相同導致條目遺失。（可選）

PublicAPI.getOnlineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
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