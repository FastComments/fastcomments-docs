Lists the webhooks configured for the tenant, both dashboard-managed rows and API subscriptions.

## Parameters

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|-------------|----------|------|
| tenantId | string | query | Tak |  |
| event | string | query | Nie |  |
| domain | string | query | Nie |  |
| source | string | query | Nie |  |
| skip | number | query | Nie |  |

## Response

Zwraca: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Example

[inline-code-attrs-start title = 'Przykład getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemów proszę zgłaszać pod http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (optional)
let domain = "domain_example" // String |  (optional)
let source = WebhookSource() // WebhookSource |  (optional)
let skip = 987 // Double |  (optional)

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
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