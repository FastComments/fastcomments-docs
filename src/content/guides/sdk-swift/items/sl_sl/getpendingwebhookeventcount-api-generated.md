## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| commentId | string | query | Ne |  |
| externalId | string | query | Ne |  |
| eventType | string | query | Ne |  |
| type | string | query | Ne |  |
| domain | string | query | Ne |  |
| attemptCountGT | number | query | Ne |  |

## Odgovor

Vrne: [`GetPendingWebhookEventCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCount200Response.swift)

## Primer

[inline-code-attrs-start title = 'getPendingWebhookEventCount Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Če odkrijete težavo, prosimo poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (neobvezno)
let externalId = "externalId_example" // String |  (neobvezno)
let eventType = "eventType_example" // String |  (neobvezno)
let type = "type_example" // String |  (neobvezno)
let domain = "domain_example" // String |  (neobvezno)
let attemptCountGT = 987 // Double |  (neobvezno)

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

---