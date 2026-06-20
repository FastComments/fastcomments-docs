## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nej |  |
| banEmailDomain | boolean | query | Nej |  |
| banIP | boolean | query | Nej |  |
| deleteAllUsersComments | boolean | query | Nej |  |
| bannedUntil | string | query | Nej |  |
| isShadowBan | boolean | query | Nej |  |
| updateId | string | query | Nej |  |
| banReason | string | query | Nej |  |
| sso | string | query | Nej |  |

## Response

Returnerer: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Eksempel

[inline-code-attrs-start title = 'postBanUserFromComment Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

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