## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| text-search | string | query | Нет |  |
| byIPFromComment | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| sorts | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример postApiExport'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в стадии бета. Для сообщения о проблеме, пожалуйста, создайте отчет через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (необязательно)
let byIPFromComment = "byIPFromComment_example" // String |  (необязательно)
let filters = "filters_example" // String |  (необязательно)
let searchFilters = "searchFilters_example" // String |  (необязательно)
let sorts = "sorts_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

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