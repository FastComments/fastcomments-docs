Queues a batch job that reverses every migration from one migrate-pages job, newest first. Available for 90 days.  
Τοποθετεί σε ουρά μια εργασία batch που αντιστρέφει κάθε μετανάστευση από μία εργασία migrate-pages, ξεκινώντας από την πιο πρόσφατη. Διαθέσιμη για 90 ημέρες.

Records changed since the migration are left alone, so a later edit is never overwritten.  
Οι εγγραφές που έχουν αλλάξει μετά τη μετανάστευση παραμένουν αμετάβλητες, ώστε μια μεταγενέστερη επεξεργασία να μην αντικαθίσταται ποτέ.

Poll GET /api/v1/batch-jobs/{batchJobId} for the restored/skipped counts.  
Κάντε ερώτημα GET /api/v1/batch-jobs/{batchJobId} για τους μετρητές επαναφοράς/παράλειψης.

## Parameters

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|------------|-----------|
| tenantId | string | query | Ναι |  |
| undoId | string | path | Ναι |  |

## Response

Returns: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/QueuePageMigrationJobAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'undoPageMigrations Παράδειγμα'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω παραδείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
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

---