## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| page | integer | query | Hayır |  |
| limit | integer | query | Hayır |  |
| skip | integer | query | Hayır |  |
| asTree | boolean | query | Hayır |  |
| skipChildren | integer | query | Hayır |  |
| limitChildren | integer | query | Hayır |  |
| maxTreeDepth | integer | query | Hayır |  |
| urlId | string | query | Hayır |  |
| userId | string | query | Hayır |  |
| anonUserId | string | query | Hayır |  |
| contextUserId | string | query | Hayır |  |
| hashTag | string | query | Hayır |  |
| parentId | string | query | Hayır |  |
| direction | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Örnek

[inline-code-attrs-start title = 'getComments Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new üzerinden bildirin
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (isteğe bağlı)
let limit = 987 // Int |  (isteğe bağlı)
let skip = 987 // Int |  (isteğe bağlı)
let asTree = true // Bool |  (isteğe bağlı)
let skipChildren = 987 // Int |  (isteğe bağlı)
let limitChildren = 987 // Int |  (isteğe bağlı)
let maxTreeDepth = 987 // Int |  (isteğe bağlı)
let urlId = "urlId_example" // String |  (isteğe bağlı)
let userId = "userId_example" // String |  (isteğe bağlı)
let anonUserId = "anonUserId_example" // String |  (isteğe bağlı)
let contextUserId = "contextUserId_example" // String |  (isteğe bağlı)
let hashTag = "hashTag_example" // String |  (isteğe bağlı)
let parentId = "parentId_example" // String |  (isteğe bağlı)
let direction = SortDirections() // SortDirections |  (isteğe bağlı)

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