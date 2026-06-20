## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| limit | number | query | Non |  |
| skip | number | query | Non |  |

## Réponse

Renvoie: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetUserBadgeProgressListResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUserBadgeProgressList'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont toujours en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (facultatif)
let limit = 987 // Double |  (facultatif)
let skip = 987 // Double |  (facultatif)

DefaultAPI.getUserBadgeProgressList(tenantId: tenantId, userId: userId, limit: limit, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]