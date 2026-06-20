## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| text-search | string | query | Нет |  |
| byIPFromComment | string | query | Нет |  |
| filters | string | query | Нет |  |
| searchFilters | string | query | Нет |  |
| afterId | string | query | Нет |  |
| demo | boolean | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationAPIGetCommentIdsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationAPIGetCommentIdsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getApiIds Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Приведенные примеры кода всё ещё находятся в бета-версии. В случае проблем, пожалуйста, сообщите по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (необязательно)
let byIPFromComment = "byIPFromComment_example" // String |  (необязательно)
let filters = "filters_example" // String |  (необязательно)
let searchFilters = "searchFilters_example" // String |  (необязательно)
let afterId = "afterId_example" // String |  (необязательно)
let demo = true // Bool |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

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