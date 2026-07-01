## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
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

Vraća: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za sve probleme, molimo prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
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