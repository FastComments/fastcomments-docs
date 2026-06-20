## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| text-search | string | query | Не |  |
| byIPFromComment | string | query | Не |  |
| filters | string | query | Не |  |
| searchFilters | string | query | Не |  |
| sorts | string | query | Не |  |
| sso | string | query | Не |  |

## Отговор

Връща: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са в бета. При проблеми, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (незадължително)
let byIPFromComment = "byIPFromComment_example" // String |  (незадължително)
let filters = "filters_example" // String |  (незадължително)
let searchFilters = "searchFilters_example" // String |  (незадължително)
let sorts = "sorts_example" // String |  (незадължително)
let sso = "sso_example" // String |  (незадължително)

ModerationAPI.postApiExport(textSearch: textSearch, byIPFromComment: byIPFromComment, filters: filters, searchFilters: searchFilters, sorts: sorts, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]