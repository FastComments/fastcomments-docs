## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | requête | Oui |  |
| urlIdWS | string | requête | Oui |  |
| userIds | string | requête | Oui |  |

## Réponse

Renvoie: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserPresenceStatuses200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getUserPresenceStatuses'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlIdWS = "urlIdWS_example" // String | 
let userIds = "userIds_example" // String | 

PublicAPI.getUserPresenceStatuses(tenantId: tenantId, urlIdWS: urlIdWS, userIds: userIds) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]