## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|--------------|-------------|
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

## Svar

Returnerer: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Eksempel

[inline-code-attrs-start title = 'postBanUserFromComment Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig i beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (valgfri)
let banEmailDomain = true // Bool |  (valgfri)
let banIP = true // Bool |  (valgfri)
let deleteAllUsersComments = true // Bool |  (valgfri)
let bannedUntil = "bannedUntil_example" // String |  (valgfri)
let isShadowBan = true // Bool |  (valgfri)
let updateId = "updateId_example" // String |  (valgfri)
let banReason = "banReason_example" // String |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

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