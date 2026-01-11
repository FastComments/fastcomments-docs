## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| page | integer | query | Non |  |
| limit | integer | query | Non |  |
| skip | integer | query | Non |  |
| asTree | boolean | query | Non |  |
| skipChildren | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| urlId | string | query | Non |  |
| userId | string | query | Non |  |
| anonUserId | string | query | Non |  |
| contextUserId | string | query | Non |  |
| hashTag | string | query | Non |  |
| parentId | string | query | Non |  |
| direction | string | query | Non |  |

## Réponse

Renvoie : [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple pour getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Les exemples de code suivants sont encore en bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (optionnel)
let limit = 987 // Int |  (optionnel)
let skip = 987 // Int |  (optionnel)
let asTree = true // Bool |  (optionnel)
let skipChildren = 987 // Int |  (optionnel)
let limitChildren = 987 // Int |  (optionnel)
let maxTreeDepth = 987 // Int |  (optionnel)
let urlId = "urlId_example" // String |  (optionnel)
let userId = "userId_example" // String |  (optionnel)
let anonUserId = "anonUserId_example" // String |  (optionnel)
let contextUserId = "contextUserId_example" // String |  (optionnel)
let hashTag = "hashTag_example" // String |  (optionnel)
let parentId = "parentId_example" // String |  (optionnel)
let direction = SortDirections() // SortDirections |  (optionnel)

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