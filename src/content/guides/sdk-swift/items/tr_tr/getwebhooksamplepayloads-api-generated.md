---
Webhook teslimlerinin tam olarak kullandığı biçimdeki son yorumlar, entegrasyonlar (örneğin Zapier örnek verileri) oluşturmak için. Her olay aynı yorum nesnesini gönderir, bu yüzden `event` yalnızca geçerli olmalıdır.

## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| event | string | query | Hayır |  |
| limit | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## Örnek

[inline-code-attrs-start title = 'getWebhookSamplePayloads Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresine bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (isteğe bağlı)
let limit = 987 // Double |  (isteğe bağlı)

DefaultAPI.getWebhookSamplePayloads(tenantId: tenantId, options: DefaultAPI.GetWebhookSamplePayloadsOptions(event: event, limit: limit)) { (response, error) in
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