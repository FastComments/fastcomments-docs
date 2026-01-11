## Параметри

| Назва | Тип | Location | Обов'язкове | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| page | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| skip | integer | query | Ні |  |
| asTree | boolean | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| urlId | string | query | Ні |  |
| userId | string | query | Ні |  |
| anonUserId | string | query | Ні |  |
| contextUserId | string | query | Ні |  |
| hashTag | string | query | Ні |  |
| parentId | string | query | Ні |  |
| direction | string | query | Ні |  |

## Відповідь

Повертає: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні приклади коду все ще у бета-версії. Якщо виникне проблема, будь ласка, повідомте через http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (необов'язково)
let limit = 987 // Int |  (необов'язково)
let skip = 987 // Int |  (необов'язково)
let asTree = true // Bool |  (необов'язково)
let skipChildren = 987 // Int |  (необов'язково)
let limitChildren = 987 // Int |  (необов'язково)
let maxTreeDepth = 987 // Int |  (необов'язково)
let urlId = "urlId_example" // String |  (необов'язково)
let userId = "userId_example" // String |  (необов'язково)
let anonUserId = "anonUserId_example" // String |  (необов'язково)
let contextUserId = "contextUserId_example" // String |  (необов'язково)
let hashTag = "hashTag_example" // String |  (необов'язково)
let parentId = "parentId_example" // String |  (необов'язково)
let direction = SortDirections() // SortDirections |  (необов'язково)

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

---