## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| badgeId | string | query | Tak |  |
| userId | string | query | Nie |  |
| commentId | string | query | Nie |  |
| broadcastId | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`AwardUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AwardUserBadgeResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład putAwardBadge'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W razie problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (opcjonalne)
let commentId = "commentId_example" // String |  (opcjonalne)
let broadcastId = "broadcastId_example" // String |  (opcjonalne)
let sso = "sso_example" // String |  (opcjonalne)

ModerationAPI.putAwardBadge(badgeId: badgeId, userId: userId, commentId: commentId, broadcastId: broadcastId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]