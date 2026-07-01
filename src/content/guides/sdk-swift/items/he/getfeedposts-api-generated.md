req
tenantId
afterId

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| afterId | string | query | לא |  |
| limit | integer | query | לא |  |
| tags | array | query | לא |  |

## תגובה

מחזיר: [`GetFeedPostsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetFeedPostsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getFeedPosts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// קטעי הקוד הבאים עדיין בטא. לכל בעיה, אנא דווחו דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let afterId = "afterId_example" // String |  (אופציונלי)
let limit = 987 // Int |  (אופציונלי)
let tags = ["inner_example"] // [String] |  (אופציונלי)

DefaultAPI.getFeedPosts(tenantId: tenantId, options: DefaultAPI.GetFeedPostsOptions(afterId: afterId, limit: limit, tags: tags)) { (response, error) in
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