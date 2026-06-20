## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBannedUsersFromCommentResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getBanUsersFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё в бета-версии. В случае проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.getBanUsersFromComment(commentId: commentId, sso: sso) { (response, error) in
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