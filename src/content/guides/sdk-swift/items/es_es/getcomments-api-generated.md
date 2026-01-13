## Parámetros

| Name | Type | Location | Required | Description |
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

## Respuesta

Devuelve: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Los siguientes ejemplos de código aún están en beta. Para cualquier problema, por favor informe en http://github.com/OpenAPITools/openapi-generator/issues/new
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