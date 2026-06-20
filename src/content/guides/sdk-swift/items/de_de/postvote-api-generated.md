## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| direction | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`VoteResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/VoteResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'postVote Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch in der Beta-Phase. Bei Problemen melden Sie diese bitte unter http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String | 
let direction = "direction_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

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