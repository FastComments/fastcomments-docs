---
Abonniert eine URL für ein Kommentarereignis (REST‑Hook‑Abonnement). Das erneute Abonnieren derselben URL für dasselbe Ereignis und dieselbe Domain gibt das bestehende Abonnement zurück. Lieferungen sind HMAC‑signiert, siehe den Webhooks‑Leitfaden; der veraltete `token`‑Header wird bei API‑Abonnements niemals gesendet.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateWebhookResponse.swift)

## Example

[inline-code-attrs-start title = 'createWebhook Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch im Beta-Stadium. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
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