## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
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

## Odgovor

Vraća: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još u beta fazi. Za bilo koji problem, prijavite ga putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (neobavezno)
let limit = 987 // Int |  (neobavezno)
let skip = 987 // Int |  (neobavezno)
let asTree = true // Bool |  (neobavezno)
let skipChildren = 987 // Int |  (neobavezno)
let limitChildren = 987 // Int |  (neobavezno)
let maxTreeDepth = 987 // Int |  (neobavezno)
let urlId = "urlId_example" // String |  (neobavezno)
let userId = "userId_example" // String |  (neobavezno)
let anonUserId = "anonUserId_example" // String |  (neobavezno)
let contextUserId = "contextUserId_example" // String |  (neobavezno)
let hashTag = "hashTag_example" // String |  (neobavezno)
let parentId = "parentId_example" // String |  (neobavezno)
let direction = SortDirections() // SortDirections |  (neobavezno)

DefaultAPI.getComments(tenantId: tenantId, page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]