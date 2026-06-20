## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | integer | query | Не |  |
| limit | integer | query | Не |  |
| skip | integer | query | Не |  |
| asTree | boolean | query | Не |  |
| skipChildren | integer | query | Не |  |
| limitChildren | integer | query | Не |  |
| maxTreeDepth | integer | query | Не |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| anonUserId | string | query | Не |  |
| contextUserId | string | query | Не |  |
| hashTag | string | query | Не |  |
| parentId | string | query | Не |  |
| direction | string | query | Не |  |
| fromDate | integer | query | Не |  |
| toDate | integer | query | Не |  |

## Отговор

Връща: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример за getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следните примерни фрагменти от код все още са в бета. За проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (по избор)
let limit = 987 // Int |  (по избор)
let skip = 987 // Int |  (по избор)
let asTree = true // Bool |  (по избор)
let skipChildren = 987 // Int |  (по избор)
let limitChildren = 987 // Int |  (по избор)
let maxTreeDepth = 987 // Int |  (по избор)
let urlId = "urlId_example" // String |  (по избор)
let userId = "userId_example" // String |  (по избор)
let anonUserId = "anonUserId_example" // String |  (по избор)
let contextUserId = "contextUserId_example" // String |  (по избор)
let hashTag = "hashTag_example" // String |  (по избор)
let parentId = "parentId_example" // String |  (по избор)
let direction = SortDirections() // SortDirections |  (по избор)
let fromDate = 987 // Int64 |  (по избор)
let toDate = 987 // Int64 |  (по избор)

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