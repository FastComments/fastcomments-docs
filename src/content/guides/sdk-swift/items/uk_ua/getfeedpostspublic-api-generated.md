req
tenantId
afterId

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| afterId | string | query | Ні |  |
| limit | integer | query | Ні |  |
| tags | array | query | Ні |  |
| sso | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |
| includeUserInfo | boolean | query | Ні |  |

## Відповідь

Повертає: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getFeedPostsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені приклади коду ще в бета-версії. У разі проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (необов'язково)
let limit = 987 // Int |  (необов'язково)
let tags = ["inner_example"] // [String] |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)
let isCrawler = true // Bool |  (необов'язково)
let includeUserInfo = true // Bool |  (необов'язково)

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