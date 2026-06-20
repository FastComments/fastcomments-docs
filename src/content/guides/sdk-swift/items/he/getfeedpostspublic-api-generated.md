---
req
tenantId
afterId

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| afterId | string | query | לא |  |
| limit | integer | query | לא |  |
| tags | array | query | לא |  |
| sso | string | query | לא |  |
| isCrawler | boolean | query | לא |  |
| includeUserInfo | boolean | query | לא |  |

## תגובה

מחזיר: [`PublicFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/PublicFeedPostsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getFeedPostsPublic'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, אנא דווח באמצעות http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (אופציונלי)
let limit = 987 // Int |  (אופציונלי)
let tags = ["inner_example"] // [String] |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)
let isCrawler = true // Bool |  (אופציונלי)
let includeUserInfo = true // Bool |  (אופציונלי)

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