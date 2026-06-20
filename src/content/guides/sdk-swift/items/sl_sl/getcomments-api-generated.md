## Parametri

| Ime | Type | Location | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| page | integer | query | Ne |  |
| limit | integer | query | Ne |  |
| skip | integer | query | Ne |  |
| asTree | boolean | query | Ne |  |
| skipChildren | integer | query | Ne |  |
| limitChildren | integer | query | Ne |  |
| maxTreeDepth | integer | query | Ne |  |
| urlId | string | query | Ne |  |
| userId | string | query | Ne |  |
| anonUserId | string | query | Ne |  |
| contextUserId | string | query | Ne |  |
| hashTag | string | query | Ne |  |
| parentId | string | query | Ne |  |
| direction | string | query | Ne |  |
| fromDate | integer | query | Ne |  |
| toDate | integer | query | Ne |  |

## Odgovor

Vrne: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'getComments Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta fazi. Za kakršnokoli težavo poročajte preko http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (neobvezno)
let limit = 987 // Int |  (neobvezno)
let skip = 987 // Int |  (neobvezno)
let asTree = true // Bool |  (neobvezno)
let skipChildren = 987 // Int |  (neobvezno)
let limitChildren = 987 // Int |  (neobvezno)
let maxTreeDepth = 987 // Int |  (neobvezno)
let urlId = "urlId_example" // String |  (neobvezno)
let userId = "userId_example" // String |  (neobvezno)
let anonUserId = "anonUserId_example" // String |  (neobvezno)
let contextUserId = "contextUserId_example" // String |  (neobvezno)
let hashTag = "hashTag_example" // String |  (neobvezno)
let parentId = "parentId_example" // String |  (neobvezno)
let direction = SortDirections() // SortDirections |  (neobvezno)
let fromDate = 987 // Int64 |  (neobvezno)
let toDate = 987 // Int64 |  (neobvezno)

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