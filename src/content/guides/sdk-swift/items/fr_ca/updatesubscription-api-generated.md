---
## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| id | string | path | Oui |  |
| userId | string | query | Non |  |

## Réponse

Retourne: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/UpdateSubscriptionAPIResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple pour updateSubscription'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code ci-dessous sont encore en version bêta. Pour tout problème, veuillez signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 
let updateAPIUserSubscriptionData = UpdateAPIUserSubscriptionData(notificationFrequency: 123) // UpdateAPIUserSubscriptionData | 
let userId = "userId_example" // String |  (optionnel)

DefaultAPI.updateSubscription(tenantId: tenantId, id: id, updateAPIUserSubscriptionData: updateAPIUserSubscriptionData, userId: userId) { (response, error) in
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