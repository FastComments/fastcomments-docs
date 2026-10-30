Izpiše spletne kuke (webhooks), konfigurirane za najemnika, tako vrstice, ki jih upravlja nadzorna plošča, kot tudi naročnine na API.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| event | string | query | Ne |  |
| domain | string | query | Ne |  |
| source | string | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
```swift
// Naslednji vzorci kode so še v beta različici. Za morebitne težave prosimo poročajte na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName | (neobvezno)
let domain = "domain_example" // String | (neobvezno)
let source = WebhookSource() // WebhookSource | (neobvezno)
let skip = 987 // Double | (neobvezno)

DefaultAPI.getWebhooks(tenantId: tenantId, options: DefaultAPI.GetWebhooksOptions(event: event, domain: domain, source: source, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```
[inline-code-end]