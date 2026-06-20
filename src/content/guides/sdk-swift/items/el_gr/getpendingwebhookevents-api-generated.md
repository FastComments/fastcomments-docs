## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| commentId | string | query | Όχι |  |
| externalId | string | query | Όχι |  |
| eventType | string | query | Όχι |  |
| type | string | query | Όχι |  |
| domain | string | query | Όχι |  |
| attemptCountGT | number | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetPendingWebhookEventsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEventsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getPendingWebhookEvents'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα βρίσκονται ακόμα σε beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (optional)
let externalId = "externalId_example" // String |  (optional)
let eventType = "eventType_example" // String |  (optional)
let type = "type_example" // String |  (optional)
let domain = "domain_example" // String |  (optional)
let attemptCountGT = 987 // Double |  (optional)
let skip = 987 // Double |  (optional)

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

---