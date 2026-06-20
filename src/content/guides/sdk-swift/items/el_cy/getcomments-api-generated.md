## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| page | integer | query | Όχι |  |
| limit | integer | query | Όχι |  |
| skip | integer | query | Όχι |  |
| asTree | boolean | query | Όχι |  |
| skipChildren | integer | query | Όχι |  |
| limitChildren | integer | query | Όχι |  |
| maxTreeDepth | integer | query | Όχι |  |
| urlId | string | query | Όχι |  |
| userId | string | query | Όχι |  |
| anonUserId | string | query | Όχι |  |
| contextUserId | string | query | Όχι |  |
| hashTag | string | query | Όχι |  |
| parentId | string | query | Όχι |  |
| direction | string | query | Όχι |  |
| fromDate | integer | query | Όχι |  |
| toDate | integer | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Τα ακόλουθα δείγματα κώδικα είναι ακόμα σε beta. Για οποιοδήποτε πρόβλημα, αναφέρετε μέσω http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (προαιρετικό)
let limit = 987 // Int |  (προαιρετικό)
let skip = 987 // Int |  (προαιρετικό)
let asTree = true // Bool |  (προαιρετικό)
let skipChildren = 987 // Int |  (προαιρετικό)
let limitChildren = 987 // Int |  (προαιρετικό)
let maxTreeDepth = 987 // Int |  (προαιρετικό)
let urlId = "urlId_example" // String |  (προαιρετικό)
let userId = "userId_example" // String |  (προαιρετικό)
let anonUserId = "anonUserId_example" // String |  (προαιρετικό)
let contextUserId = "contextUserId_example" // String |  (προαιρετικό)
let hashTag = "hashTag_example" // String |  (προαιρετικό)
let parentId = "parentId_example" // String |  (προαιρετικό)
let direction = SortDirections() // SortDirections |  (προαιρετικό)
let fromDate = 987 // Int64 |  (προαιρετικό)
let toDate = 987 // Int64 |  (προαιρετικό)

DefaultAPI.getComments(tenantId: tenantId, page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]