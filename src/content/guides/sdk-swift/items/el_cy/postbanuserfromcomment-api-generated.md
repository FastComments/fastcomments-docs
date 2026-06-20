## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| banEmail | boolean | query | Όχι |  |
| banEmailDomain | boolean | query | Όχι |  |
| banIP | boolean | query | Όχι |  |
| deleteAllUsersComments | boolean | query | Όχι |  |
| bannedUntil | string | query | Όχι |  |
| isShadowBan | boolean | query | Όχι |  |
| updateId | string | query | Όχι |  |
| banReason | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BanUserFromCommentResult.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα postBanUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let banEmail = true // Bool |  (προαιρετικό)
let banEmailDomain = true // Bool |  (προαιρετικό)
let banIP = true // Bool |  (προαιρετικό)
let deleteAllUsersComments = true // Bool |  (προαιρετικό)
let bannedUntil = "bannedUntil_example" // String |  (προαιρετικό)
let isShadowBan = true // Bool |  (προαιρετικό)
let updateId = "updateId_example" // String |  (προαιρετικό)
let banReason = "banReason_example" // String |  (προαιρετικό)
let sso = "sso_example" // String |  (προαιρετικό)

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