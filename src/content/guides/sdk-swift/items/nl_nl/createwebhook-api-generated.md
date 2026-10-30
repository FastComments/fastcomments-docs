Abonneert een URL op een commentaar‑evenement (REST‑hook‑abonnement). Het opnieuw abonneren van dezelfde  
URL op hetzelfde evenement en domein retourneert de bestaande abonnement. Leveringen zijn HMAC‑ondertekend, zie de  
webhooks‑gids; de verouderde `token`‑header wordt nooit verzonden naar API‑abonnementen.

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |

## Reactie

Retourneert: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateWebhookResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'createWebhook Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Voor eventuele problemen, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createWebhookParams = CreateWebhookParams(url: "url_example", event: WebhookEventName(), domain: "domain_example", method: WebhookHTTPMethod()) // CreateWebhookParams | 

DefaultAPI.createWebhook(tenantId: tenantId, createWebhookParams: createWebhookParams) { (response, error) in
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