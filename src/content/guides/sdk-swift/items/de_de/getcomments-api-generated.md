## Parameter

| Name | Typ | Location | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nein |  |
| limit | integer | query | Nein |  |
| skip | integer | query | Nein |  |
| asTree | boolean | query | Nein |  |
| skipChildren | integer | query | Nein |  |
| limitChildren | integer | query | Nein |  |
| maxTreeDepth | integer | query | Nein |  |
| urlId | string | query | Nein |  |
| userId | string | query | Nein |  |
| anonUserId | string | query | Nein |  |
| contextUserId | string | query | Nein |  |
| hashTag | string | query | Nein |  |
| parentId | string | query | Nein |  |
| direction | string | query | Nein |  |
| fromDate | integer | query | Nein |  |
| toDate | integer | query | Nein |  |

## Antwort

Gibt zurück: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'getComments Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch Beta. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (optional)
let limit = 987 // Int |  (optional)
let skip = 987 // Int |  (optional)
let asTree = true // Bool |  (optional)
let skipChildren = 987 // Int |  (optional)
let limitChildren = 987 // Int |  (optional)
let maxTreeDepth = 987 // Int |  (optional)
let urlId = "urlId_example" // String |  (optional)
let userId = "userId_example" // String |  (optional)
let anonUserId = "anonUserId_example" // String |  (optional)
let contextUserId = "contextUserId_example" // String |  (optional)
let hashTag = "hashTag_example" // String |  (optional)
let parentId = "parentId_example" // String |  (optional)
let direction = SortDirections() // SortDirections |  (optional)
let fromDate = 987 // Int64 |  (optional)
let toDate = 987 // Int64 |  (optional)

DefaultAPI.getComments(tenantId: tenantId, page: page, limit: limit, skip: skip, asTree: asTree, skipChildren: skipChildren, limitChildren: limitChildren, maxTreeDepth: maxTreeDepth, urlId: urlId, userId: userId, anonUserId: anonUserId, contextUserId: contextUserId, hashTag: hashTag, parentId: parentId, direction: direction, fromDate: fromDate, toDate: toDate) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]