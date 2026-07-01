## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| page | integer | query | No |  |
| limit | integer | query | No |  |
| skip | integer | query | No |  |
| asTree | boolean | query | No |  |
| skipChildren | integer | query | No |  |
| limitChildren | integer | query | No |  |
| maxTreeDepth | integer | query | No |  |
| urlId | string | query | No |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |
| contextUserId | string | query | No |  |
| hashTag | string | query | No |  |
| parentId | string | query | No |  |
| direction | string | query | No |  |
| fromDate | integer | query | No |  |
| toDate | integer | query | No |  |

## Odziv

Vrne: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. V primeru težav, prosimo, prijavite jih na http://github.com/OpenAPITools/openapi-generator/issues/new
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

DefaultAPI.getComments(tenantId: tenantId, options: DefaultAPI.GetCommentsOptions(page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]