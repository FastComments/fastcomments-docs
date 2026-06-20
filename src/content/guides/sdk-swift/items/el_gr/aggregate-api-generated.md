Συσσωρεύει έγγραφα ομαδοποιώντας τα (εάν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διαφορετικές λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| parentTenantId | string | query | Όχι |  |
| includeStats | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AggregateResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregate'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα παρακάτω παραδείγματα κώδικα είναι ακόμα beta. Για οποιοδήποτε πρόβλημα, παρακαλώ αναφέρετέ το μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let aggregationRequest = AggregationRequest(query: [QueryPredicate(key: "key_example", value: QueryPredicate_value(), _operator: "_operator_example")], resourceName: "resourceName_example", groupBy: ["groupBy_example"], operations: [AggregationOperation(field: "field_example", op: AggregationOpType(), alias: "alias_example", expandArray: false)], sort: AggregationRequest_sort(dir: "dir_example", field: "field_example")) // AggregationRequest | 
let parentTenantId = "parentTenantId_example" // String |  (προαιρετικό)
let includeStats = true // Bool |  (προαιρετικό)

DefaultAPI.aggregate(tenantId: tenantId, aggregationRequest: aggregationRequest, parentTenantId: parentTenantId, includeStats: includeStats) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]