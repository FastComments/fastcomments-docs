---
## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
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

## Отговор

Връща: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примери за код все още са в бета. За всеки проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (незадължително)
let count = 987 // Double |  (незадължително)
let textSearch = "textSearch_example" // String |  (незадължително)
let byIPFromComment = "byIPFromComment_example" // String |  (незадължително)
let filters = "filters_example" // String |  (незадължително)
let searchFilters = "searchFilters_example" // String |  (незадължително)
let sorts = "sorts_example" // String |  (незадължително)
let demo = true // Bool |  (незадължително)
let sso = "sso_example" // String |  (незадължително)

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

---