Σειροθετεί μια εργασία δέσμης που αντιστρέφει κάθε μετανάστευση από μία εργασία migrate-pages, από τη νεότερη προς την παλαιότερη. Διαθέσιμη για 90 ημέρες.
Οι εγγραφές που άλλαξαν μετά τη μετανάστευση δεν επηρεάζονται, ώστε μια μεταγενέστερη επεξεργασία να μην αντικαθίσταται ποτέ.
Κάντε polling στο GET /api/v1/batch-jobs/{batchJobId} για τα πλήθη των επαναφερόμενων/παραλειφθέντων.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| undoId | string | path | Ναι |  |

## Απάντηση

Επιστρέφει: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα undoPageMigrations'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let undoId = "undoId_example" // String | 

DefaultAPI.undoPageMigrations(tenantId: tenantId, undoId: undoId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]