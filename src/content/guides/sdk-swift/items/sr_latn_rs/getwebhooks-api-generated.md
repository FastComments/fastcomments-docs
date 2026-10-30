Lists the webhooks configured for the tenant, both dashboard-managed rows and API subscriptions.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| domain | string | query | No |  |
| source | string | query | No |  |
| skip | number | query | No |  |

## Response

Returns: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Example

[inline-code-attrs-start title = 'Primer getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
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