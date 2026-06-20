## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| banEmail | boolean | query | Нет |  |
| banEmailDomain | boolean | query | Нет |  |
| banIP | boolean | query | Нет |  |
| deleteAllUsersComments | boolean | query | Нет |  |
| bannedUntil | string | query | Нет |  |
| isShadowBan | boolean | query | Нет |  |
| updateId | string | query | Нет |  |
| banReason | string | query | Нет |  |
| sso | string | query | Нет |  |

## Ответ

Возвращает: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Пример

[inline-code-attrs-start title = 'Пример postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в стадии бета. В случае проблем, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (необязательно)
let banEmailDomain = true // Bool |  (необязательно)
let banIP = true // Bool |  (необязательно)
let deleteAllUsersComments = true // Bool |  (необязательно)
let bannedUntil = "bannedUntil_example" // String |  (необязательно)
let isShadowBan = true // Bool |  (необязательно)
let updateId = "updateId_example" // String |  (необязательно)
let banReason = "banReason_example" // String |  (необязательно)
let sso = "sso_example" // String |  (необязательно)

ModerationAPI.postBanUserFromComment(commentId: commentId, banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso) { (response, error) in
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