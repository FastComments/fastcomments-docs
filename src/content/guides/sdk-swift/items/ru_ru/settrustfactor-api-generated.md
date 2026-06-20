## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| userId | string | query | Нет |  |
| trustFactor | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример использования setTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все ещё находятся в бета-версии. В случае проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (необязательно)
let trustFactor = "trustFactor_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.setTrustFactor(userId: userId, trustFactor: trustFactor, sso: sso) { (response, error) in
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