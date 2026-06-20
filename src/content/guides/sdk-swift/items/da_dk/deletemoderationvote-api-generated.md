## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| voteId | string | path | Ja |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteDeleteResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'deleteModerationVote Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. For problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let voteId = "voteId_example" // String | 
let sso = "sso_example" // String |  (valgfri)

ModerationAPI.deleteModerationVote(commentId: commentId, voteId: voteId, sso: sso) { (response, error) in
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