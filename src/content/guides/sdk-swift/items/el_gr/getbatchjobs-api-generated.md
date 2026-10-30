The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Μόνο εργασίες αυτού του τύπου, για παράδειγμα migrate-pages. |
| status | string | query | No | Μόνο εργασίες σε αυτήν την κατάσταση: requested, running, success ή failed. |
| skip | integer | query | No | Πόσες εργασίες να παραλειφθούν, για σελιδοποίηση. |

## Απόκριση

Επιστρέφει: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getBatchJobs'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω δείγματα κώδικα είναι ακόμη σε beta. Για οποιοδήποτε πρόβλημα, παρακαλούμε αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | Μόνο εργασίες αυτού του τύπου, για παράδειγμα migrate-pages. (προαιρετικό)
let status = "status_example" // String | Μόνο εργασίες σε αυτήν την κατάσταση: requested, running, success ή failed. (προαιρετικό)
let skip = 987 // Int | Πόσες εργασίες να παραλειφθούν, για σελιδοποίηση. (προαιρετικό)

DefaultAPI.getBatchJobs(tenantId: tenantId, options: DefaultAPI.GetBatchJobsOptions(type: type, status: status, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]