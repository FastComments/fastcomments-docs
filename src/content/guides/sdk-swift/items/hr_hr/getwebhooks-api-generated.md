---
Popis webhookova konfiguriranih za najamnika, kako redaka upravljanih putem nadzorne ploče, tako i pretplata na API.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| domain | string | query | No |  |
| source | string | query | No |  |
| skip | number | query | No |  |

## Odgovor

Vraća: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Primjer

[inline-code-attrs-start title = 'Primjer getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda još su beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (opcionalno)
let domain = "domain_example" // String |  (opcionalno)
let source = WebhookSource() // WebhookSource |  (opcionalno)
let skip = 987 // Double |  (opcionalno)

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