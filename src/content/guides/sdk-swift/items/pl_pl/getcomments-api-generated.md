## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|------|
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

## Odpowiedź

Zwraca: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Przykład

[inline-code-attrs-start title = 'getComments Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku jakichkolwiek problemów, proszę zgłaszać je poprzez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String |
let page = 987 // Int |  (opcjonalny)
let limit = 987 // Int |  (opcjonalny)
let skip = 987 // Int |  (opcjonalny)
let asTree = true // Bool |  (opcjonalny)
let skipChildren = 987 // Int |  (opcjonalny)
let limitChildren = 987 // Int |  (opcjonalny)
let maxTreeDepth = 987 // Int |  (opcjonalny)
let urlId = "urlId_example" // String |  (opcjonalny)
let userId = "userId_example" // String |  (opcjonalny)
let anonUserId = "anonUserId_example" // String |  (opcjonalny)
let contextUserId = "contextUserId_example" // String |  (opcjonalny)
let hashTag = "hashTag_example" // String |  (opcjonalny)
let parentId = "parentId_example" // String |  (opcjonalny)
let direction = SortDirections() // SortDirections |  (opcjonalny)
let fromDate = 987 // Int64 |  (opcjonalny)
let toDate = 987 // Int64 |  (opcjonalny)

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