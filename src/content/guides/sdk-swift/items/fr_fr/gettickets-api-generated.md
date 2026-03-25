## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| state | number | query | Non |  |
| skip | number | query | Non |  |
| limit | number | query | Non |  |

## Réponse

Renvoie : [`GetTickets200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetTickets200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getTickets'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (facultatif)
let state = 987 // Double |  (facultatif)
let skip = 987 // Double |  (facultatif)
let limit = 987 // Double |  (facultatif)

DefaultAPI.getTickets(tenantId: tenantId, userId: userId, state: state, skip: skip, limit: limit) { (response, error) in
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