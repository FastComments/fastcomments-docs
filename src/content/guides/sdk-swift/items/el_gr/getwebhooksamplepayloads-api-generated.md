Πρόσφατα σχόλια ακριβώς στη μορφή που χρησιμοποιούν οι παραδόσεις webhook, για τη δημιουργία ενσωματώσεων (για παράδειγμα δείγματα δεδομένων Zapier). Κάθε συμβάν παραδίδει το ίδιο αντικείμενο σχολίου, έτσι το `event` πρέπει μόνο να είναι έγκυρο.

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|-----------|
| tenantId | string | query | Ναι |  |
| event | string | query | Όχι |  |
| limit | number | query | Όχι |  |

## Response

Επιστρέφει: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetWebhookSamplePayloadsResponse.swift)

## Example

[inline-code-attrs-start title = 'getWebhookSamplePayloads Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let event = WebhookEventName() // WebhookEventName |  (προαιρετικό)
let limit = 987 // Double |  (προαιρετικό)

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