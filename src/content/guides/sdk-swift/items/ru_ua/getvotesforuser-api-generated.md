## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |

## Ответ

Возвращает: [`GetVotesForUserResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetVotesForUserResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getVotesForUser'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все еще находятся в бета-версии. Если обнаружите проблему, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userId = "userId_example" // String |  (необязательно)
let anonUserId = "anonUserId_example" // String |  (необязательно)

DefaultAPI.getVotesForUser(tenantId: tenantId, urlId: urlId, userId: userId, anonUserId: anonUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]