## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nee |  |
| banEmailDomain | boolean | query | Nee |  |
| banIP | boolean | query | Nee |  |
| deleteAllUsersComments | boolean | query | Nee |  |
| bannedUntil | string | query | Nee |  |
| isShadowBan | boolean | query | Nee |  |
| updateId | string | query | Nee |  |
| banReason | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Retourneert: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Voorbeeld

[inline-code-attrs-start title = 'postBanUserFromComment Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, meld deze alstublieft via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (optioneel)
let banEmailDomain = true // Bool |  (optioneel)
let banIP = true // Bool |  (optioneel)
let deleteAllUsersComments = true // Bool |  (optioneel)
let bannedUntil = "bannedUntil_example" // String |  (optioneel)
let isShadowBan = true // Bool |  (optioneel)
let updateId = "updateId_example" // String |  (optioneel)
let banReason = "banReason_example" // String |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

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