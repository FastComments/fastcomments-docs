## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| badgeId | string | query | Ja |  |
| userId | string | query | Nee |  |
| commentId | string | query | Nee |  |
| broadcastId | string | query | Nee |  |
| sso | string | query | Nee |  |

## Antwoord

Geeft terug: [`RemoveUserBadgeResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/RemoveUserBadgeResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'putRemoveBadge Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Als u een probleem tegenkomt, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let badgeId = "badgeId_example" // String | 
let userId = "userId_example" // String |  (optioneel)
let commentId = "commentId_example" // String |  (optioneel)
let broadcastId = "broadcastId_example" // String |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

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

---