Wyświetla listę webhooków skonfigurowanych dla tenanta, obejmując zarówno wiersze zarządzane z pulpitu, jak i subskrypcje API.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagany | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| event | string | query | Nie |  |
| domain | string | query | Nie |  |
| source | string | query | Nie |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (opcjonalnie)
let domain = "domain_example" // String |  (opcjonalnie)
let source = WebhookSource() // WebhookSource |  (opcjonalnie)
let skip = 987 // Double |  (opcjonalnie)

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