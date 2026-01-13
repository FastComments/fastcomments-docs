## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| commentId | string | query | Hayır |  |
| externalId | string | query | Hayır |  |
| eventType | string | query | Hayır |  |
| type | string | query | Hayır |  |
| domain | string | query | Hayır |  |
| attemptCountGT | number | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## Örnek

[inline-code-attrs-start title = 'getPendingWebhookEvents Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (isteğe bağlı)
let externalId = "externalId_example" // String |  (isteğe bağlı)
let eventType = "eventType_example" // String |  (isteğe bağlı)
let type = "type_example" // String |  (isteğe bağlı)
let domain = "domain_example" // String |  (isteğe bağlı)
let attemptCountGT = 987 // Double |  (isteğe bağlı)
let skip = 987 // Double |  (isteğe bağlı)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]