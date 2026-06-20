## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
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

## Risposta

Restituisce: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/APIGetCommentsResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio di getComments'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let page = 987 // Int |  (opzionale)
let limit = 987 // Int |  (opzionale)
let skip = 987 // Int |  (opzionale)
let asTree = true // Bool |  (opzionale)
let skipChildren = 987 // Int |  (opzionale)
let limitChildren = 987 // Int |  (opzionale)
let maxTreeDepth = 987 // Int |  (opzionale)
let urlId = "urlId_example" // String |  (opzionale)
let userId = "userId_example" // String |  (opzionale)
let anonUserId = "anonUserId_example" // String |  (opzionale)
let contextUserId = "contextUserId_example" // String |  (opzionale)
let hashTag = "hashTag_example" // String |  (opzionale)
let parentId = "parentId_example" // String |  (opzionale)
let direction = SortDirections() // SortDirections |  (opzionale)
let fromDate = 987 // Int64 |  (opzionale)
let toDate = 987 // Int64 |  (opzionale)

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