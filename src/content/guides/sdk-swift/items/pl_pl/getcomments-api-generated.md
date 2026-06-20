## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| page | integer | query | Nie |  |
| limit | integer | query | Nie |  |
| skip | integer | query | Nie |  |
| asTree | boolean | query | Nie |  |
| skipChildren | integer | query | Nie |  |
| limitChildren | integer | query | Nie |  |
| maxTreeDepth | integer | query | Nie |  |
| urlId | string | query | Nie |  |
| userId | string | query | Nie |  |
| anonUserId | string | query | Nie |  |
| contextUserId | string | query | Nie |  |
| hashTag | string | query | Nie |  |
| parentId | string | query | Nie |  |
| direction | string | query | Nie |  |
| fromDate | integer | query | Nie |  |
| toDate | integer | query | Nie |  |

## Odpowiedź

Zwraca: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W razie problemu zgłoś go przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (opcjonalne)
let limit = 987 // Int |  (opcjonalne)
let skip = 987 // Int |  (opcjonalne)
let asTree = true // Bool |  (opcjonalne)
let skipChildren = 987 // Int |  (opcjonalne)
let limitChildren = 987 // Int |  (opcjonalne)
let maxTreeDepth = 987 // Int |  (opcjonalne)
let urlId = "urlId_example" // String |  (opcjonalne)
let userId = "userId_example" // String |  (opcjonalne)
let anonUserId = "anonUserId_example" // String |  (opcjonalne)
let contextUserId = "contextUserId_example" // String |  (opcjonalne)
let hashTag = "hashTag_example" // String |  (opcjonalne)
let parentId = "parentId_example" // String |  (opcjonalne)
let direction = SortDirections() // SortDirections |  (opcjonalne)
let fromDate = 987 // Int64 |  (opcjonalne)
let toDate = 987 // Int64 |  (opcjonalne)

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

---