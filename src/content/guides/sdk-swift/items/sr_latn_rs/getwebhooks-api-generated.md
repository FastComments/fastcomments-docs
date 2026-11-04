Navodi konfigurisane webhook-ove za tenant, i redove kojima upravlja dashboard i API pretplate.

## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| event | string | query | Ne |  |
| domain | string | query | Ne |  |
| source | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Primer

[inline-code-attrs-start title = 'getWebhooks primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beti. Za bilo koji problem, prijavite preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (opciono)
let domain = "domain_example" // String |  (opciono)
let source = WebhookSource() // WebhookSource |  (opciono)
let skip = 987 // Double |  (opciono)

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