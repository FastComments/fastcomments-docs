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

## Odziv

Vrne: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Primer

[inline-code-attrs-start title = 'Primer postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta različici. Za morebitne težave poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (izbirno)
let banEmailDomain = true // Bool |  (izbirno)
let banIP = true // Bool |  (izbirno)
let deleteAllUsersComments = true // Bool |  (izbirno)
let bannedUntil = "bannedUntil_example" // String |  (izbirno)
let isShadowBan = true // Bool |  (izbirno)
let updateId = "updateId_example" // String |  (izbirno)
let banReason = "banReason_example" // String |  (izbirno)
let sso = "sso_example" // String |  (izbirno)

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