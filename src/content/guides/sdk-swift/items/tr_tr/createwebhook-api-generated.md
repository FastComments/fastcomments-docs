Subscribes a URL to a comment event (REST hook subscribe). Subscribing the same URL to the same
event and domain again returns the existing subscription. Deliveries are HMAC signed, see the
webhooks guide; the legacy `token` header is never sent to API subscriptions.

## Parameters

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |

## Yanıt

Döndürür: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateWebhookResponse.swift)

## Örnek

[inline-code-attrs-start title = 'createWebhook Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta sürümündedir. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
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