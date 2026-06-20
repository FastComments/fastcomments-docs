## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Ja |  |
| direction | string | query | Ja |  |
| userId | string | query | Nee |  |
| anonUserId | string | query | Nee |  |

## Antwoord

Retourneert: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'createVote Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in bèta. Als er een probleem is, meld dit via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let direction = "direction_example" // String | 
let userId = "userId_example" // String |  (optioneel)
let anonUserId = "anonUserId_example" // String |  (optioneel)

DefaultAPI.createVote(tenantId: tenantId, commentId: commentId, direction: direction, userId: userId, anonUserId: anonUserId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]