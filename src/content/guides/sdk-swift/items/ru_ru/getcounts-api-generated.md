## Параметры

| Имя | Тип | Местоположение | Обязательный | Описание |
|------|------|----------|----------|-------------|
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetBannedUsersCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersCountResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример использования getCounts'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. Если возникнут проблемы, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getCounts(sso: sso) { (response, error) in
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