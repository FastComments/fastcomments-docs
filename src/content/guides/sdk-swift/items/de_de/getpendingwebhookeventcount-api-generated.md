## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nein |  |
| externalId | string | query | Nein |  |
| eventType | string | query | Nein |  |
| type | string | query | Nein |  |
| domain | string | query | Nein |  |
| attemptCountGT | number | query | Nein |  |

## Antwort

Gibt zurück: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCountResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getPendingWebhookEventCount Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (optional)
let externalId = "externalId_example" // String |  (optional)
let eventType = "eventType_example" // String |  (optional)
let type = "type_example" // String |  (optional)
let domain = "domain_example" // String |  (optional)
let attemptCountGT = 987 // Double |  (optional)

DefaultAPI.getPendingWebhookEventCount(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]