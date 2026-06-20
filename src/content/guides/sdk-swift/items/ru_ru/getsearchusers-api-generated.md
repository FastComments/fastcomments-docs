## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| value | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`ModerationUserSearchResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationUserSearchResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getSearchUsers'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие образцы кода всё ещё находятся в бета-версии. По любым проблемам, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let value = "value_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getSearchUsers(value: value, sso: sso) { (response, error) in
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