## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| page | integer | query | Nee |  |
| limit | integer | query | Nee |  |
| skip | integer | query | Nee |  |
| asTree | boolean | query | Nee |  |
| skipChildren | integer | query | Nee |  |
| limitChildren | integer | query | Nee |  |
| maxTreeDepth | integer | query | Nee |  |
| urlId | string | query | Nee |  |
| userId | string | query | Nee |  |
| anonUserId | string | query | Nee |  |
| contextUserId | string | query | Nee |  |
| hashTag | string | query | Nee |  |
| parentId | string | query | Nee |  |
| direction | string | query | Nee |  |

## Antwoord

Retourneert: [`GetComments200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetComments200Response.swift)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (optioneel)
let limit = 987 // Int |  (optioneel)
let skip = 987 // Int |  (optioneel)
let asTree = true // Bool |  (optioneel)
let skipChildren = 987 // Int |  (optioneel)
let limitChildren = 987 // Int |  (optioneel)
let maxTreeDepth = 987 // Int |  (optioneel)
let urlId = "urlId_example" // String |  (optioneel)
let userId = "userId_example" // String |  (optioneel)
let anonUserId = "anonUserId_example" // String |  (optioneel)
let contextUserId = "contextUserId_example" // String |  (optioneel)
let hashTag = "hashTag_example" // String |  (optioneel)
let parentId = "parentId_example" // String |  (optioneel)
let direction = SortDirections() // SortDirections |  (optioneel)

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