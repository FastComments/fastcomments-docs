---
Αναγνωρίζει το διαπιστευτήριο σε χρήση: το μισθωτή στο οποίο ανήκει και, για διακριτικά OAuth, τον χρήστη που το εξουσιοδότησε.  
Οι ενσωματώσεις το χρησιμοποιούν για να δοκιμάσουν μια σύνδεση και να την ετικετοποιήσουν.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|-----------|------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetMeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetMeResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getMe'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετε το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 

DefaultAPI.getMe(tenantId: tenantId) { (response, error) in
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