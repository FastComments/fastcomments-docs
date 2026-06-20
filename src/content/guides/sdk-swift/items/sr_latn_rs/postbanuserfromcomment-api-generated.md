## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| banEmail | boolean | query | Ne |  |
| banEmailDomain | boolean | query | Ne |  |
| banIP | boolean | query | Ne |  |
| deleteAllUsersComments | boolean | query | Ne |  |
| bannedUntil | string | query | Ne |  |
| isShadowBan | boolean | query | Ne |  |
| updateId | string | query | Ne |  |
| banReason | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Primer

[inline-code-attrs-start title = 'postBanUserFromComment Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (neobavezno)
let banEmailDomain = true // Bool |  (neobavezno)
let banIP = true // Bool |  (neobavezno)
let deleteAllUsersComments = true // Bool |  (neobavezno)
let bannedUntil = "bannedUntil_example" // String |  (neobavezno)
let isShadowBan = true // Bool |  (neobavezno)
let updateId = "updateId_example" // String |  (neobavezno)
let banReason = "banReason_example" // String |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

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