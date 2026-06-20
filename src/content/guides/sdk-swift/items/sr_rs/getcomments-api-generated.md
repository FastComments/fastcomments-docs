## Параметри

| Име | Тип | Локација | Обавезно | Опис |
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

## Одговор

Враћа: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'getComments Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примери кода су још у бети. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (опционо)
let limit = 987 // Int |  (опционо)
let skip = 987 // Int |  (опционо)
let asTree = true // Bool |  (опционо)
let skipChildren = 987 // Int |  (опционо)
let limitChildren = 987 // Int |  (опционо)
let maxTreeDepth = 987 // Int |  (опционо)
let urlId = "urlId_example" // String |  (опционо)
let userId = "userId_example" // String |  (опционо)
let anonUserId = "anonUserId_example" // String |  (опционо)
let contextUserId = "contextUserId_example" // String |  (опционо)
let hashTag = "hashTag_example" // String |  (опционо)
let parentId = "parentId_example" // String |  (опционо)
let direction = SortDirections() // SortDirections |  (опционо)
let fromDate = 987 // Int64 |  (опционо)
let toDate = 987 // Int64 |  (опционо)

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