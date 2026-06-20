## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| approved | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrača: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer postSetCommentApprovalStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Če naletite na težave, jih prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let approved = true // Bool |  (neobvezno)
let sso = "sso_example" // String |  (neobvezno)

ModerationAPI.postSetCommentApprovalStatus(commentId: commentId, approved: approved, sso: sso) { (response, error) in
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