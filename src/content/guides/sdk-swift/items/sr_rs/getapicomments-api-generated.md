---
## Параметри

| Име | Тип | Локација | Обавезно | Опис |
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

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бета фази. За било који проблем пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (необавезно)
let count = 987 // Double |  (необавезно)
let textSearch = "textSearch_example" // String |  (необавезно)
let byIPFromComment = "byIPFromComment_example" // String |  (необавезно)
let filters = "filters_example" // String |  (необавезно)
let searchFilters = "searchFilters_example" // String |  (необавезно)
let sorts = "sorts_example" // String |  (необавезно)
let demo = true // Bool |  (необавезно)
let sso = "sso_example" // String |  (необавезно)

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