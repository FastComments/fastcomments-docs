## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | запрос | Да |  |
| id | string | путь | Да |  |
| userId | string | запрос | Нет |  |
| anonUserId | string | запрос | Нет |  |

## Ответ

Возвращает: [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Пример

[inline-code-attrs-start title = 'blockUserFromComment Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода находятся в бета-версии. При любых проблемах, пожалуйста, сообщайте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let blockFromCommentParams = BlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // BlockFromCommentParams | 
let userId = "userId_example" // String |  (необязательно)
let anonUserId = "anonUserId_example" // String |  (необязательно)

DefaultAPI.blockUserFromComment(tenantId: tenantId, id: id, blockFromCommentParams: blockFromCommentParams, options: DefaultAPI.BlockUserFromCommentOptions(userId: userId, anonUserId: anonUserId)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]