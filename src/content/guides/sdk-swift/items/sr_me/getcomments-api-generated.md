## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
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

## Odgovor

Vraća: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Primjer

[inline-code-attrs-start title = 'getComments Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su i dalje beta. Za bilo koji problem, molimo vas da ga prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (opcionalno)
let limit = 987 // Int |  (opcionalno)
let skip = 987 // Int |  (opcionalno)
let asTree = true // Bool |  (opcionalno)
let skipChildren = 987 // Int |  (opcionalno)
let limitChildren = 987 // Int |  (opcionalno)
let maxTreeDepth = 987 // Int |  (opcionalno)
let urlId = "urlId_example" // String |  (opcionalno)
let userId = "userId_example" // String |  (opcionalno)
let anonUserId = "anonUserId_example" // String |  (opcionalno)
let contextUserId = "contextUserId_example" // String |  (opcionalno)
let hashTag = "hashTag_example" // String |  (opcionalno)
let parentId = "parentId_example" // String |  (opcionalno)
let direction = SortDirections() // SortDirections |  (opcionalno)
let fromDate = 987 // Int64 |  (opcionalno)
let toDate = 987 // Int64 |  (opcionalno)

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