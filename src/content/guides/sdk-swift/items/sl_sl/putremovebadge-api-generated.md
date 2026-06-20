## Parametri

| Name | Type | Location | Obvezno | Opis |
|------|------|----------|----------|-------------|
| badgeId | string | query | Da |  |
| userId | string | query | Ne |  |
| commentId | string | query | Ne |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odziv

Vrne: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RemoveUserBadgeResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer putRemoveBadge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še v beta. Za morebitne težave prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (izbirno)
let commentId = "commentId_example" // String |  (izbirno)
let broadcastId = "broadcastId_example" // String |  (izbirno)
let sso = "sso_example" // String |  (izbirno)

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