## 參數

| 名稱 | 類型 | 位置 | 必需 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| postIds | array | query | Yes |  |
| sso | string | query | No |  |

## 回應

回傳: [`FeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FeedPostsStatsResponse.swift)

## 範例

[inline-code-attrs-start title = 'getFeedPostsStats 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下範例程式碼仍屬測試版。如遇任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let postIds = ["inner_example"] // [String] | 
let sso = "sso_example" // String |  (可選)

PublicAPI.getFeedPostsStats(tenantId: tenantId, postIds: postIds, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]