## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Nee |  |
| externalId | string | query | Nee |  |
| eventType | string | query | Nee |  |
| type | string | query | Nee |  |
| domain | string | query | Nee |  |
| attemptCountGT | number | query | Nee |  |

## Respons

Retourneert: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventCountResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getPendingWebhookEventCount Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in beta. Voor eventuele problemen, gelieve te melden via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (optioneel)
let externalId = "externalId_example" // String |  (optioneel)
let eventType = "eventType_example" // String |  (optioneel)
let type = "type_example" // String |  (optioneel)
let domain = "domain_example" // String |  (optioneel)
let attemptCountGT = 987 // Double |  (optioneel)

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