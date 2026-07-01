## Параметри

| Назва | Тип | Розташування | Обов’язковий | Опис |
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

## Відповідь

Повертає: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду ще є бета. У разі будь-якої проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (опціонально)
let limit = 987 // Int |  (опціонально)
let skip = 987 // Int |  (опціонально)
let asTree = true // Bool |  (опціонально)
let skipChildren = 987 // Int |  (опціонально)
let limitChildren = 987 // Int |  (опціонально)
let maxTreeDepth = 987 // Int |  (опціонально)
let urlId = "urlId_example" // String |  (опціонально)
let userId = "userId_example" // String |  (опціонально)
let anonUserId = "anonUserId_example" // String |  (опціонально)
let contextUserId = "contextUserId_example" // String |  (опціонально)
let hashTag = "hashTag_example" // String |  (опціонально)
let parentId = "parentId_example" // String |  (опціонально)
let direction = SortDirections() // SortDirections |  (опціонально)
let fromDate = 987 // Int64 |  (опціонально)
let toDate = 987 // Int64 |  (опціонально)

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