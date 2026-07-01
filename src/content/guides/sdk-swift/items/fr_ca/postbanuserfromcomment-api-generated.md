## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
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

Renvoie : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (facultatif)
let banEmailDomain = true // Bool |  (facultatif)
let banIP = true // Bool |  (facultatif)
let deleteAllUsersComments = true // Bool |  (facultatif)
let bannedUntil = "bannedUntil_example" // String |  (facultatif)
let isShadowBan = true // Bool |  (facultatif)
let updateId = "updateId_example" // String |  (facultatif)
let banReason = "banReason_example" // String |  (facultatif)
let sso = "sso_example" // String |  (facultatif)

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