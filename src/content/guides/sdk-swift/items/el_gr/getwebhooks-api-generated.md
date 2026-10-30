Lists the webhooks configured for the tenant, both dashboard-managed rows and API subscriptions.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| event | string | query | Όχι |  |
| domain | string | query | Όχι |  |
| source | string | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetWebhooksResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhooksResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getWebhooks'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (προαιρετικό)
let domain = "domain_example" // String |  (προαιρετικό)
let source = WebhookSource() // WebhookSource |  (προαιρετικό)
let skip = 987 // Double |  (προαιρετικό)

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