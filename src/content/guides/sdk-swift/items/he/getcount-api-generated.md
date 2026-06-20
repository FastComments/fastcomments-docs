## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filter | string | query | לא |  |
| searchFilters | string | query | לא |  |
| demo | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דגימות הקוד הבאות עדיין בבטא. עבור כל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (אופציונלי)
let byIPFromComment = "byIPFromComment_example" // String |  (אופציונלי)
let filter = "filter_example" // String |  (אופציונלי)
let searchFilters = "searchFilters_example" // String |  (אופציונלי)
let demo = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.getCount(textSearch: textSearch, byIPFromComment: byIPFromComment, filter: filter, searchFilters: searchFilters, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]