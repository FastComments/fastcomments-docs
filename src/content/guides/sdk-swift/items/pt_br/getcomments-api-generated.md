## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| page | integer | query | Não |  |
| limit | integer | query | Não |  |
| skip | integer | query | Não |  |
| asTree | boolean | query | Não |  |
| skipChildren | integer | query | Não |  |
| limitChildren | integer | query | Não |  |
| maxTreeDepth | integer | query | Não |  |
| urlId | string | query | Não |  |
| userId | string | query | Não |  |
| anonUserId | string | query | Não |  |
| contextUserId | string | query | Não |  |
| hashTag | string | query | Não |  |
| parentId | string | query | Não |  |
| direction | string | query | Não |  |

## Resposta

Retorna: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (opcional)
let limit = 987 // Int |  (opcional)
let skip = 987 // Int |  (opcional)
let asTree = true // Bool |  (opcional)
let skipChildren = 987 // Int |  (opcional)
let limitChildren = 987 // Int |  (opcional)
let maxTreeDepth = 987 // Int |  (opcional)
let urlId = "urlId_example" // String |  (opcional)
let userId = "userId_example" // String |  (opcional)
let anonUserId = "anonUserId_example" // String |  (opcional)
let contextUserId = "contextUserId_example" // String |  (opcional)
let hashTag = "hashTag_example" // String |  (opcional)
let parentId = "parentId_example" // String |  (opcional)
let direction = SortDirections() // SortDirections |  (opcional)

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