## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | query | Non |  |
| externalId | string | query | Non |  |
| eventType | string | query | Non |  |
| type | string | query | Non |  |
| domain | string | query | Non |  |
| attemptCountGT | number | query | Non |  |

## Réponse

Renvoie : [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCountResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getPendingWebhookEventCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les extraits de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (optionnel)
let externalId = "externalId_example" // String |  (optionnel)
let eventType = "eventType_example" // String |  (optionnel)
let type = "type_example" // String |  (optionnel)
let domain = "domain_example" // String |  (optionnel)
let attemptCountGT = 987 // Double |  (optionnel)

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, options: DefaultAPI.GetPendingWebhookEventCountOptions(commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]