## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| approved | boolean | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie: [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de postSetCommentApprovalStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let approved = true // Bool |  (facultatif)
let sso = "sso_example" // String |  (facultatif)

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