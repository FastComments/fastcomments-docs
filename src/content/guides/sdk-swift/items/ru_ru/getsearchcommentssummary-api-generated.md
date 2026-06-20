## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationCommentSearchResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример использования getSearchCommentsSummary'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все еще находятся в бета-версии. О любых проблемах сообщайте по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (необязательно)
let filters = "filters_example" // String |  (необязательно)
let searchFilters = "searchFilters_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getSearchCommentsSummary(value: value, filters: filters, searchFilters: searchFilters, sso: sso) { (response, error) in
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