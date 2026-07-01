## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
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

## Réponse

Renvoie : [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (optional)
let limit = 987 // Int |  (optional)
let skip = 987 // Int |  (optional)
let asTree = true // Bool |  (optional)
let skipChildren = 987 // Int |  (optional)
let limitChildren = 987 // Int |  (optional)
let maxTreeDepth = 987 // Int |  (optional)
let urlId = "urlId_example" // String |  (optional)
let userId = "userId_example" // String |  (optional)
let anonUserId = "anonUserId_example" // String |  (optional)
let contextUserId = "contextUserId_example" // String |  (optional)
let hashTag = "hashTag_example" // String |  (optional)
let parentId = "parentId_example" // String |  (optional)
let direction = SortDirections() // SortDirections |  (optional)
let fromDate = 987 // Int64 |  (optional)
let toDate = 987 // Int64 |  (optional)

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