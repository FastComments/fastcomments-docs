## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |

## Ответ

Возвращает: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UnblockSuccess.swift)

## Пример

[inline-code-attrs-start title = 'Пример unBlockUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода все еще находятся в бета-версии. При возникновении проблемы, пожалуйста, сообщите по адресу http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let unBlockFromCommentParams = UnBlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // UnBlockFromCommentParams | 
let userId = "userId_example" // String |  (необязательно)
let anonUserId = "anonUserId_example" // String |  (необязательно)

DefaultAPI.unBlockUserFromComment(tenantId: tenantId, id: id, unBlockFromCommentParams: unBlockFromCommentParams, userId: userId, anonUserId: anonUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]