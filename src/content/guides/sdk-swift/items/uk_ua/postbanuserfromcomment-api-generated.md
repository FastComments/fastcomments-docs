## Параметри

| Назва | Тип | Location | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| commentId | string | path | Так |  |
| banEmail | boolean | query | Ні |  |
| banEmailDomain | boolean | query | Ні |  |
| banIP | boolean | query | Ні |  |
| deleteAllUsersComments | boolean | query | Ні |  |
| bannedUntil | string | query | Ні |  |
| isShadowBan | boolean | query | Ні |  |
| updateId | string | query | Ні |  |
| banReason | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще перебувають у бета-версії. У разі проблем, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (необов'язково)
let banEmailDomain = true // Bool |  (необов'язково)
let banIP = true // Bool |  (необов'язково)
let deleteAllUsersComments = true // Bool |  (необов'язково)
let bannedUntil = "bannedUntil_example" // String |  (необов'язково)
let isShadowBan = true // Bool |  (необов'язково)
let updateId = "updateId_example" // String |  (необов'язково)
let banReason = "banReason_example" // String |  (необов'язково)
let sso = "sso_example" // String |  (необов'язково)

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