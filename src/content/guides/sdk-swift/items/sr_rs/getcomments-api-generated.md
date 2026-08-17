## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|------|
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

[inline-code-attrs-start title = 'Primer getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (опционално)
let limit = 987 // Int |  (опционално)
let skip = 987 // Int |  (опционално)
let asTree = true // Bool |  (опционално)
let skipChildren = 987 // Int |  (опционално)
let limitChildren = 987 // Int |  (опционално)
let maxTreeDepth = 987 // Int |  (опционално)
let urlId = "urlId_example" // String |  (опционално)
let userId = "userId_example" // String |  (опционално)
let anonUserId = "anonUserId_example" // String |  (опционално)
let contextUserId = "contextUserId_example" // String |  (опционално)
let hashTag = "hashTag_example" // String |  (опционално)
let parentId = "parentId_example" // String |  (опционално)
let direction = SortDirections() // SortDirections |  (опционално)
let fromDate = 987 // Int64 |  (опционално)
let toDate = 987 // Int64 |  (опционално)

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