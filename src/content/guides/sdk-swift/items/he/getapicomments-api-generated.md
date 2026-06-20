## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| page | number | query | לא |  |
| count | number | query | לא |  |
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sorts | string | query | לא |  |
| demo | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// דוגמאות הקוד הבאות עדיין בבטא. לכל בעיה, אנא דווח דרך http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (אופציונלי)
let count = 987 // Double |  (אופציונלי)
let textSearch = "textSearch_example" // String |  (אופציונלי)
let byIPFromComment = "byIPFromComment_example" // String |  (אופציונלי)
let filters = "filters_example" // String |  (אופציונלי)
let searchFilters = "searchFilters_example" // String |  (אופציונלי)
let sorts = "sorts_example" // String |  (אופציונלי)
let demo = true // Bool |  (אופציונלי)
let sso = "sso_example" // String |  (אופציונלי)

ModerationAPI.getApiComments(page: page, count: count, textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, demo: demo, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]