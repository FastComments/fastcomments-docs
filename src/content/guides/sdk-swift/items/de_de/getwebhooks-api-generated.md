Lists the webhooks configured for the tenant, both dashboard-managed rows and API subscriptions.

## Parameters

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|---------------|--------------|
| tenantId | string | query | Ja |  |
| event | string | query | Nein |  |
| domain | string | query | Nein |  |
| source | string | query | Nein |  |
| skip | number | query | Nein |  |

## Response

Returns: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Example

[inline-code-attrs-start title = 'getWebhooks Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
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