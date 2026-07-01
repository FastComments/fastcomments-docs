## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
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

## Odgovor

Returns: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći uzorci koda su još u beta fazi. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (opciono)
let limit = 987 // Int |  (opciono)
let skip = 987 // Int |  (opciono)
let asTree = true // Bool |  (opciono)
let skipChildren = 987 // Int |  (opciono)
let limitChildren = 987 // Int |  (opciono)
let maxTreeDepth = 987 // Int |  (opciono)
let urlId = "urlId_example" // String |  (opciono)
let userId = "userId_example" // String |  (opciono)
let anonUserId = "anonUserId_example" // String |  (opciono)
let contextUserId = "contextUserId_example" // String |  (opciono)
let hashTag = "hashTag_example" // String |  (opciono)
let parentId = "parentId_example" // String |  (opciono)
let direction = SortDirections() // SortDirections |  (opciono)
let fromDate = 987 // Int64 |  (opciono)
let toDate = 987 // Int64 |  (opciono)

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