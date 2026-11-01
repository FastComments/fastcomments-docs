## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Odpowiedź

Zwraca: [`VotePollResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VotePollResponse.swift)

## Przykład

[inline-code-attrs-start title = 'votePoll Przykład'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Poniższe przykłady kodu są nadal w wersji beta. W przypadku problemów, proszę zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String | 
let votePollBody = VotePollBody(optionId: "optionId_example") // VotePollBody | 
let urlId = "urlId_example" // String |  (opcjonalnie)
let broadcastId = "broadcastId_example" // String |  (opcjonalnie)
let sso = "sso_example" // String |  (opcjonalnie)

PublicAPI.votePoll(tenantId: tenantId, commentId: commentId, votePollBody: votePollBody, options: PublicAPI.VotePollOptions(urlId: urlId, broadcastId: broadcastId, sso: sso)) { (response, error) in
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