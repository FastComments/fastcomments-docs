---
Kiracı için yapılandırılmış webhook'ları listeler, hem kontrol paneli yönetimli satırları hem de API aboneliklerini.

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| event | string | query | Hayır |  |
| domain | string | query | Hayır |  |
| source | string | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getWebhooks Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (isteğe bağlı)
let domain = "domain_example" // String |  (isteğe bağlı)
let source = WebhookSource() // WebhookSource |  (isteğe bağlı)
let skip = 987 // Double |  (isteğe bağlı)

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