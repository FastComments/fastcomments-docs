Συνδέει ένα URL σε ένα συμβάν σχολίου (REST hook subscribe). Η επανασύνδεση του ίδιου URL στο ίδιο
συμβάν και domain επιστρέφει την υπάρχουσα συνδρομή. Οι παραδόσεις υπογράφονται με HMAC, δείτε τον οδηγό
webhooks· η παλαιά κεφαλίδα `token` δεν αποστέλλεται ποτέ σε συνδρομές μέσω API.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απάντηση

Επιστρέφει: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateWebhookResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα createWebhook'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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