## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| afterId | string | query | לא |  |
| demo | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getApiIds'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד שלהלן עדיין בבטא. עבור כל בעיה, דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (אופציונלי)
let byIPFromComment = "byIPFromComment_example" // String |  (אופציונלי)
let filters = "filters_example" // String |  (אופציונלי)
let searchFilters = "searchFilters_example" // String |  (אופציונלי)
let afterId = "afterId_example" // String |  (אופציונלי)
let demo = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.getApiIds(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, afterId: afterId, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]