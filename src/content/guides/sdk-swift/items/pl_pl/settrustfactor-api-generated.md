## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Nie |  |
| trustFactor | string | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład setTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są wciąż w wersji beta. W przypadku jakichkolwiek problemów prosimy zgłosić je pod adresem http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcjonalny)
let trustFactor = "trustFactor_example" // String |  (opcjonalny)
let sso = "sso_example" // String |  (opcjonalny)

ModerationAPI.setTrustFactor(tenantId: tenantId, options: ModerationAPI.SetTrustFactorOptions(userId: userId, trustFactor: trustFactor, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]