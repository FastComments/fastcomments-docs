## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| page | integer | query | Нет |  |
| limit | integer | query | Нет |  |
| skip | integer | query | Нет |  |
| asTree | boolean | query | Нет |  |
| skipChildren | integer | query | Нет |  |
| limitChildren | integer | query | Нет |  |
| maxTreeDepth | integer | query | Нет |  |
| urlId | string | query | Нет |  |
| userId | string | query | Нет |  |
| anonUserId | string | query | Нет |  |
| contextUserId | string | query | Нет |  |
| hashTag | string | query | Нет |  |
| parentId | string | query | Нет |  |
| direction | string | query | Нет |  |

## Ответ

Возвращает: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Пример

[inline-code-attrs-start title = 'Пример getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следующие примеры кода всё ещё находятся в бета-версии. Если возникнут проблемы, пожалуйста, сообщите через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (необязательно)
let limit = 987 // Int |  (необязательно)
let skip = 987 // Int |  (необязательно)
let asTree = true // Bool |  (необязательно)
let skipChildren = 987 // Int |  (необязательно)
let limitChildren = 987 // Int |  (необязательно)
let maxTreeDepth = 987 // Int |  (необязательно)
let urlId = "urlId_example" // String |  (необязательно)
let userId = "userId_example" // String |  (необязательно)
let anonUserId = "anonUserId_example" // String |  (необязательно)
let contextUserId = "contextUserId_example" // String |  (необязательно)
let hashTag = "hashTag_example" // String |  (необязательно)
let parentId = "parentId_example" // String |  (необязательно)
let direction = SortDirections() // SortDirections |  (необязательно)

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