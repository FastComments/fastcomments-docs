## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Нет |  |
| sso | string | query | Нет |  |

## Response

Возвращает: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationPageSearchResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getSearchPages'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в статусе бета. При возникновении проблем, пожалуйста, сообщите об этом по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getSearchPages(value: value, sso: sso) { (response, error) in
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