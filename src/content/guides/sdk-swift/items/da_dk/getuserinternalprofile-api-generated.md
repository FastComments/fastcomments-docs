## Parametre

| Navn | Type | Placering | Obligatorisk | Beskrivelse |
|------|------|----------|----------|-------------|
| commentId | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`GetUserInternalProfileResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserInternalProfileResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'getUserInternalProfile Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Hvis du oplever problemer, bedes du rapportere det via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let commentId = "commentId_example" // String |  (valgfrit)
let sso = "sso_example" // String |  (valgfrit)

ModerationAPI.getUserInternalProfile(commentId: commentId, sso: sso) { (response, error) in
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