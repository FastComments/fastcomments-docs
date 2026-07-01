## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|--------------|--------------|
| tenantId | string | query | Ja |  |
| userId | string | query | Nein |  |
| trustFactor | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Beispiel

[inline-code-attrs-start title = 'setTrustFactor Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele befinden sich noch im Beta‑Status. Bei Problemen bitte melden via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (optional)
let trustFactor = "trustFactor_example" // String |  (optional)
let sso = "sso_example" // String |  (optional)

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