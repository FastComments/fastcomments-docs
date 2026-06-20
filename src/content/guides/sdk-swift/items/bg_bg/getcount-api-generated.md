## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filter | string | query | Не |  |
| searchFilters | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPICountCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета. Ако имате проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (по избор)
let byIPFromComment = "byIPFromComment_example" // String |  (по избор)
let filter = "filter_example" // String |  (по избор)
let searchFilters = "searchFilters_example" // String |  (по избор)
let demo = true // Bool |  (по избор)
let sso = "sso_example" // String |  (по избор)

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

---