## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
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

## Отговор

Връща: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getComments Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следващите примерни кодове все още са бета. При какъвто и да е проблем, моля докладвайте чрез http://github.com/OpenAPITools/openapi-generator/issues/new
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

---