## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| afterId | string | query | Не |  |
| demo | boolean | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getApiIds'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни кодове все още са в бета. За всякакъв проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (незадължително)
let byIPFromComment = "byIPFromComment_example" // String |  (незадължително)
let filters = "filters_example" // String |  (незадължително)
let searchFilters = "searchFilters_example" // String |  (незадължително)
let afterId = "afterId_example" // String |  (незадължително)
let demo = true // Bool |  (незадължително)
let sso = "sso_example" // String |  (незадължително)

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