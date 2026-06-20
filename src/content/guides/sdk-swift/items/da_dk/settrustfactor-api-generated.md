## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Nej |  |
| trustFactor | string | query | Nej |  |
| sso | string | query | Nej |  |

## Svar

Returnerer: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'setTrustFactor Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Følgende kodeeksempler er stadig i beta. Ved problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (valgfri)
let trustFactor = "trustFactor_example" // String |  (valgfri)
let sso = "sso_example" // String |  (valgfri)

ModerationAPI.setTrustFactor(userId: userId, trustFactor: trustFactor, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]