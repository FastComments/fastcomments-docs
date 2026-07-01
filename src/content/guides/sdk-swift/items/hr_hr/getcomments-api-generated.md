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

[inline-code-attrs-start title = 'Primjer getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda još su beta. Za bilo koji problem, molimo prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int | (neobavezno)
let limit = 987 // Int | (neobavezno)
let skip = 987 // Int | (neobavezno)
let asTree = true // Bool | (neobavezno)
let skipChildren = 987 // Int | (neobavezno)
let limitChildren = 987 // Int | (neobavezno)
let maxTreeDepth = 987 // Int | (neobavezno)
let urlId = "urlId_example" // String | (neobavezno)
let userId = "userId_example" // String | (neobavezno)
let anonUserId = "anonUserId_example" // String | (neobavezno)
let contextUserId = "contextUserId_example" // String | (neobavezno)
let hashTag = "hashTag_example" // String | (neobavezno)
let parentId = "parentId_example" // String | (neobavezno)
let direction = SortDirections() // SortDirections | (neobavezno)
let fromDate = 987 // Int64 | (neobavezno)
let toDate = 987 // Int64 | (neobavezno)

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