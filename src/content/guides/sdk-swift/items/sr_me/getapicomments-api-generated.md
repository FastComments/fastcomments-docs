## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| page | number | query | Не |  |
| count | number | query | Не |  |
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sorts | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getApiComments Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем, пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (опционо)
let count = 987 // Double |  (опционо)
let textSearch = "textSearch_example" // String |  (опционо)
let byIPFromComment = "byIPFromComment_example" // String |  (опционо)
let filters = "filters_example" // String |  (опционо)
let searchFilters = "searchFilters_example" // String |  (опционо)
let sorts = "sorts_example" // String |  (опционо)
let demo = true // Bool |  (опционо)
let sso = "sso_example" // String |  (опционо)

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