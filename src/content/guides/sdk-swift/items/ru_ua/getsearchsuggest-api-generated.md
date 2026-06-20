## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationSuggestResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getSearchSuggest'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. Если возникнут проблемы, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let textSearch = "textSearch_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getSearchSuggest(textSearch: textSearch, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]