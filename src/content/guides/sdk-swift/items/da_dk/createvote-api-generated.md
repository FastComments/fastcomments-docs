## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | query | Ja |  |
| direction | string | query | Ja |  |
| userId | string | query | Nej |  |
| anonUserId | string | query | Nej |  |

## Svar

Returnerer: [`VoteComment200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteComment200Response.swift)

## Eksempel

[inline-code-attrs-start title = 'createVote-eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever et problem, så rapportér det via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let direction = "direction_example" // String | 
let userId = "userId_example" // String |  (valgfrit)
let anonUserId = "anonUserId_example" // String |  (valgfrit)

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