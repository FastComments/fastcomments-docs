## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| commentId | string | query | No |  |
| externalId | string | query | No |  |
| eventType | string | query | No |  |
| type | string | query | No |  |
| domain | string | query | No |  |
| attemptCountGT | number | query | No |  |

## Risposta

Restituisce: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCount200Response.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPendingWebhookEventCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (opzionale)
let externalId = "externalId_example" // String |  (opzionale)
let eventType = "eventType_example" // String |  (opzionale)
let type = "type_example" // String |  (opzionale)
let domain = "domain_example" // String |  (opzionale)
let attemptCountGT = 987 // Double |  (opzionale)

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