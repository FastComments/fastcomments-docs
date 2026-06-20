## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| banEmail | boolean | query | Non |  |
| banEmailDomain | boolean | query | Non |  |
| banIP | boolean | query | Non |  |
| deleteAllUsersComments | boolean | query | Non |  |
| bannedUntil | string | query | Non |  |
| isShadowBan | boolean | query | Non |  |
| updateId | string | query | Non |  |
| banReason | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (optionnel)
let banEmailDomain = true // Bool |  (optionnel)
let banIP = true // Bool |  (optionnel)
let deleteAllUsersComments = true // Bool |  (optionnel)
let bannedUntil = "bannedUntil_example" // String |  (optionnel)
let isShadowBan = true // Bool |  (optionnel)
let updateId = "updateId_example" // String |  (optionnel)
let banReason = "banReason_example" // String |  (optionnel)
let sso = "sso_example" // String |  (optionnel)

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