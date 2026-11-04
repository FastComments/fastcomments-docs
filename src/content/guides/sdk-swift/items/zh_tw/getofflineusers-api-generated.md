過去曾在頁面上留言但目前不在線上的留言者。按 displayName 排序。
在用完 /users/online 之後,使用此端點來渲染「成員」區塊。
以 commenterName 進行游標分頁:伺服器透過 $gt 從 afterName 開始,沿著部分索引 {tenantId, urlId, commenterName} 向後走,沒有 $skip 的成本。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 | 頁面 URL 識別碼(由伺服器端清理)。 |
| afterName | string | query | 否 | 游標:傳入上一個回應中的 nextAfterName。 |
| afterUserId | string | query | 否 | 游標決勝依據:傳入上一個回應中的 nextAfterUserId。當設定 afterName 時為必填,以避免同名時遺漏項目。 |

## 回應

回傳:[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 頁面 URL 識別碼(由伺服器端清理)。
let afterName = "afterName_example" // String | 游標:傳入上一個回應中的 nextAfterName。(optional)
let afterUserId = "afterUserId_example" // String | 游標決勝依據:傳入上一個回應中的 nextAfterUserId。當設定 afterName 時為必填,以避免同名時遺漏項目。(optional)

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