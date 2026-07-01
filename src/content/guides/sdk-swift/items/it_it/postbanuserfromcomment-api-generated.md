## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | path | Sì |  |
| banEmail | boolean | query | No |  |
| banEmailDomain | boolean | query | No |  |
| banIP | boolean | query | No |  |
| deleteAllUsersComments | boolean | query | No |  |
| bannedUntil | string | query | No |  |
| isShadowBan | boolean | query | No |  |
| updateId | string | query | No |  |
| banReason | string | query | No |  |
| sso | string | query | No |  |

## Risposta

Restituisce: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Esempio

[inline-code-attrs-start title = 'postBanUserFromComment Esempio'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalalo via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (opzionale)
let banEmailDomain = true // Bool |  (opzionale)
let banIP = true // Bool |  (opzionale)
let deleteAllUsersComments = true // Bool |  (opzionale)
let bannedUntil = "bannedUntil_example" // String |  (opzionale)
let isShadowBan = true // Bool |  (opzionale)
let updateId = "updateId_example" // String |  (opzionale)
let banReason = "banReason_example" // String |  (opzionale)
let sso = "sso_example" // String |  (opzionale)

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