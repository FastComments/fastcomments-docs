## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | requête | Oui |  |
| id | string | chemin | Oui |  |
| userId | string | requête | Non |  |
| anonUserId | string | requête | Non |  |

## Réponse

Retourne : [`BlockSuccess`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/BlockSuccess.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de blockUserFromComment'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let blockFromCommentParams = BlockFromCommentParams(commentIdsToCheck: ["commentIdsToCheck_example"]) // BlockFromCommentParams | 
let userId = "userId_example" // String |  (optionnel)
let anonUserId = "anonUserId_example" // String |  (optionnel)

DefaultAPI.blockUserFromComment(tenantId: tenantId, id: id, blockFromCommentParams: blockFromCommentParams, userId: userId, anonUserId: anonUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]