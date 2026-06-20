## Параметри

| Име | Тип | Локација | Обавезно | Опис |
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

## Одговор

Враћа: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи примјери кода су још у бета фази. За било који проблем, пријавите га преко http://github.com/OpenAPITools/openapi-generator/issues/new
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