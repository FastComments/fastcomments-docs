## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| urlId | string | query | Non |  |
| fromCommentId | string | query | Non |  |
| viewed | boolean | query | Non |  |
| type | string | query | Non |  |

## Réponse

Renvoie : [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCount200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getNotificationCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les extraits de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (optionnel)
let urlId = "urlId_example" // String |  (optionnel)
let fromCommentId = "fromCommentId_example" // String |  (optionnel)
let viewed = true // Bool |  (optionnel)
let type = "type_example" // String |  (optionnel)

DefaultAPI.getNotificationCount(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type) { (response, error) in
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