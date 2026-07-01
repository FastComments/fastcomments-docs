## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
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

## Svar

Returnerer: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getComments Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
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
let fromDate = 987 // Int64 |  (valgfri)
let toDate = 987 // Int64 |  (valgfri)

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