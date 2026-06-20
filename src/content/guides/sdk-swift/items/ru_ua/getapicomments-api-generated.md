## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| page | number | query | Нет |  |
| count | number | query | Нет |  |
| text-search | string | query | Нет |  |
| byIPFromComment | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| sorts | string | query | Нет |  |
| demo | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getApiComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. В случае проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let page = 987 // Double |  (необязательно)
let count = 987 // Double |  (необязательно)
let textSearch = "textSearch_example" // String |  (необязательно)
let byIPFromComment = "byIPFromComment_example" // String |  (необязательно)
let filters = "filters_example" // String |  (необязательно)
let searchFilters = "searchFilters_example" // String |  (необязательно)
let sorts = "sorts_example" // String |  (необязательно)
let demo = true // Bool |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

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