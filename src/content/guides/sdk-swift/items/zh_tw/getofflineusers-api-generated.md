過去曾在該頁面發表過評論但目前未上線的使用者。依 displayName 排序。
在用盡 /users/online 後使用此端點來呈現「成員」區段。
在 commenterName 上的游標分頁：伺服器會利用部分索引 {tenantId, urlId, commenterName}，從 afterName 向前走透過 $gt，無 $skip 成本。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | 頁面網址識別碼（伺服器端會清理）。 |
| afterName | string | query | No | 分頁游標：傳入前一個回應中的 nextAfterName。 |
| afterUserId | string | query | No | 分頁平手決定器：傳入前一個回應中的 nextAfterUserId。當設定 afterName 時需提供，以免同名導致條目遺漏。 |

## 回應

回傳: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PageUsersOfflineResponse.swift)

## 範例

[inline-code-attrs-start title = 'getOfflineUsers 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式範例仍屬測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 頁面網址識別碼（伺服器端會清理）。
let afterName = "afterName_example" // String | 分頁游標：傳入前一個回應中的 nextAfterName。 (optional)
let afterUserId = "afterUserId_example" // String | 分頁平手決定器：傳入前一個回應中的 nextAfterUserId。當設定 afterName 時為必要，以避免同名導致條目遺漏。 (optional)

PublicAPI.getOfflineUsers(tenantId: tenantId, urlId: urlId, afterName: afterName, afterUserId: afterUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]