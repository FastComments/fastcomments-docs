## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Да |  |
| banEmail | boolean | query | Не |  |
| banEmailDomain | boolean | query | Не |  |
| banIP | boolean | query | Не |  |
| deleteAllUsersComments | boolean | query | Не |  |
| bannedUntil | string | query | Не |  |
| isShadowBan | boolean | query | Не |  |
| updateId | string | query | Не |  |
| banReason | string | query | Не |  |
| sso | string | query | Не |  |

## Одговор

Враћа: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још увек у бета фази. За било који проблем, пријавите на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (опционо)
let banEmailDomain = true // Bool |  (опционо)
let banIP = true // Bool |  (опционо)
let deleteAllUsersComments = true // Bool |  (опционо)
let bannedUntil = "bannedUntil_example" // String |  (опционо)
let isShadowBan = true // Bool |  (опционо)
let updateId = "updateId_example" // String |  (опционо)
let banReason = "banReason_example" // String |  (опционо)
let sso = "sso_example" // String |  (опционо)

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