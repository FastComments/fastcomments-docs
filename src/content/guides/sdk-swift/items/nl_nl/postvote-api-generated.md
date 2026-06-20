## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| direction | string | query | Nee |  |
| sso | string | query | Nee |  |

## Respons

Retourneert: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## Voorbeeld

[inline-code-attrs-start title = 'postVote Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden bevinden zich nog in beta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let direction = "direction_example" // String |  (optioneel)
let sso = "sso_example" // String |  (optioneel)

ModerationAPI.postVote(commentId: commentId, direction: direction, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]