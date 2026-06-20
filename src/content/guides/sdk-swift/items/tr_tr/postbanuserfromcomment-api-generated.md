## Parametreler

| Ad | Tip | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| commentId | string | path | Evet |  |
| banEmail | boolean | query | Hayır |  |
| banEmailDomain | boolean | query | Hayır |  |
| banIP | boolean | query | Hayır |  |
| deleteAllUsersComments | boolean | query | Hayır |  |
| bannedUntil | string | query | Hayır |  |
| isShadowBan | boolean | query | Hayır |  |
| updateId | string | query | Hayır |  |
| banReason | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Örnek

[inline-code-attrs-start title = 'postBanUserFromComment Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresi üzerinden bildirin
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (isteğe bağlı)
let banEmailDomain = true // Bool |  (isteğe bağlı)
let banIP = true // Bool |  (isteğe bağlı)
let deleteAllUsersComments = true // Bool |  (isteğe bağlı)
let bannedUntil = "bannedUntil_example" // String |  (isteğe bağlı)
let isShadowBan = true // Bool |  (isteğe bağlı)
let updateId = "updateId_example" // String |  (isteğe bağlı)
let banReason = "banReason_example" // String |  (isteğe bağlı)
let sso = "sso_example" // String |  (isteğe bağlı)

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