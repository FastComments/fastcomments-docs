## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| banEmail | boolean | query | Nie |  |
| banEmailDomain | boolean | query | Nie |  |
| banIP | boolean | query | Nie |  |
| deleteAllUsersComments | boolean | query | Nie |  |
| bannedUntil | string | query | Nie |  |
| isShadowBan | boolean | query | Nie |  |
| updateId | string | query | Nie |  |
| banReason | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są wciąż w fazie beta. W razie problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (opcjonalne)
let banEmailDomain = true // Bool |  (opcjonalne)
let banIP = true // Bool |  (opcjonalne)
let deleteAllUsersComments = true // Bool |  (opcjonalne)
let bannedUntil = "bannedUntil_example" // String |  (opcjonalne)
let isShadowBan = true // Bool |  (opcjonalne)
let updateId = "updateId_example" // String |  (opcjonalne)
let banReason = "banReason_example" // String |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

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