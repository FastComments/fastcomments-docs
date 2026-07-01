## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| approved | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Réponse

Retourne : [`SetCommentApprovedResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetCommentApprovedResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple postSetCommentApprovalStatus'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// La suite des exemples de code est encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // Chaîne | 
let commentId = "commentId_example" // Chaîne | 
let approved = true // Booléen |  (optionnel)
let broadcastId = "broadcastId_example" // Chaîne |  (optionnel)
let sso = "sso_example" // Chaîne |  (optionnel)

ModerationAPI.postSetCommentApprovalStatus(tenantId: tenantId, commentId: commentId, options: ModerationAPI.PostSetCommentApprovalStatusOptions(approved: approved, broadcastId: broadcastId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]