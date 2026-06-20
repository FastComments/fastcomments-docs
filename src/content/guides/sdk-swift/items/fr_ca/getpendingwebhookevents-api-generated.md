## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | requête | Oui |  |
| commentId | string | requête | Non |  |
| externalId | string | requête | Non |  |
| eventType | string | requête | Non |  |
| type | string | requête | Non |  |
| domain | string | requête | Non |  |
| attemptCountGT | number | requête | Non |  |
| skip | number | requête | Non |  |

## Réponse

Renvoie : [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventsResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple de getPendingWebhookEvents'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (optionnel)
let externalId = "externalId_example" // String |  (optionnel)
let eventType = "eventType_example" // String |  (optionnel)
let type = "type_example" // String |  (optionnel)
let domain = "domain_example" // String |  (optionnel)
let attemptCountGT = 987 // Double |  (optionnel)
let skip = 987 // Double |  (optionnel)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]