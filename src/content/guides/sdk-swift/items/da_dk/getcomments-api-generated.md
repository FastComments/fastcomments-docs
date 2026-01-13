## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| skip | integer | query | Nej |  |
| asTree | boolean | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| urlId | string | query | Nej |  |
| userId | string | query | Nej |  |
| anonUserId | string | query | Nej |  |
| contextUserId | string | query | Nej |  |
| hashTag | string | query | Nej |  |
| parentId | string | query | Nej |  |
| direction | string | query | Nej |  |

## Respons

Returnerer: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis der opstår problemer, bedes du rapportere via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (valgfri)
let limit = 987 // Int |  (valgfri)
let skip = 987 // Int |  (valgfri)
let asTree = true // Bool |  (valgfri)
let skipChildren = 987 // Int |  (valgfri)
let limitChildren = 987 // Int |  (valgfri)
let maxTreeDepth = 987 // Int |  (valgfri)
let urlId = "urlId_example" // String |  (valgfri)
let userId = "userId_example" // String |  (valgfri)
let anonUserId = "anonUserId_example" // String |  (valgfri)
let contextUserId = "contextUserId_example" // String |  (valgfri)
let hashTag = "hashTag_example" // String |  (valgfri)
let parentId = "parentId_example" // String |  (valgfri)
let direction = SortDirections() // SortDirections |  (valgfri)

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