req
tenantId
afterId

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| afterId | string | query | いいえ |  |
| limit | integer | query | いいえ |  |
| tags | array | query | いいえ |  |
| sso | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |
| includeUserInfo | boolean | query | いいえ |  |

## レスポンス

戻り値: [`GetFeedPostsPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsPublic200Response.swift)

## 例

[inline-code-attrs-start title = 'getFeedPostsPublic の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (オプション)
let limit = 987 // Int |  (オプション)
let tags = ["inner_example"] // [String] |  (オプション)
let sso = "sso_example" // String |  (オプション)
let isCrawler = true // Bool |  (オプション)
let includeUserInfo = true // Bool |  (オプション)

PublicAPI.getFeedPostsPublic(tenantId: tenantId, afterId: afterId, limit: limit, tags: tags, sso: sso, isCrawler: isCrawler, includeUserInfo: includeUserInfo) { (response, error) in
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