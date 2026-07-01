## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Отговор

Returns: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Пример

[inline-code-attrs-start title = 'postBanUserFromComment Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове са все още бета. За всеки проблем, моля, докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (опционално)
let banEmailDomain = true // Bool |  (опционално)
let banIP = true // Bool |  (опционално)
let deleteAllUsersComments = true // Bool |  (опционално)
let bannedUntil = "bannedUntil_example" // String |  (опционално)
let isShadowBan = true // Bool |  (опционално)
let updateId = "updateId_example" // String |  (опционално)
let banReason = "banReason_example" // String |  (опционално)
let sso = "sso_example" // String |  (опционално)

ModerationAPI.postBanUserFromComment(tenantId: tenantId, commentId: commentId, options: ModerationAPI.PostBanUserFromCommentOptions(banEmail: banEmail, banEmailDomain: banEmailDomain, banIP: banIP, deleteAllUsersComments: deleteAllUsersComments, bannedUntil: bannedUntil, isShadowBan: isShadowBan, updateId: updateId, banReason: banReason, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]