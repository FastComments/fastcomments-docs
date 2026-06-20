## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Da |  |
| userId | string | query | Ne |  |
| commentId | string | query | Ne |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Response

Vraća: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RemoveUserBadgeResponse.swift)

## Primer

[inline-code-attrs-start title = 'putRemoveBadge Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sledeći primeri koda su još uvek beta. Za bilo koji problem, prijavite putem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (neobavezno)
let commentId = "commentId_example" // String |  (neobavezno)
let broadcastId = "broadcastId_example" // String |  (neobavezno)
let sso = "sso_example" // String |  (neobavezno)

ModerationAPI.putRemoveBadge(badgeId: badgeId, userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]