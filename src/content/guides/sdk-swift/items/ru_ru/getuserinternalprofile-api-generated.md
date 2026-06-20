## Параметры

| Name | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| commentId | string | query | Нет |  |
| sso | string | query | Нет |  |

## Возвращает

Возвращает: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getUserInternalProfile'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. В случае проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getUserInternalProfile(commentId: commentId, sso: sso) { (response, error) in
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