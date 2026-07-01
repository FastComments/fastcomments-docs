## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|--------------|-------------|-------------|
| tenantId | string | query | Oui |  |
| userId | string | query | Non |  |
| trustFactor | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple setTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Le code suivant est encore en version bêta. Pour tout problème, veuillez le signaler via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (facultatif)
let trustFactor = "trustFactor_example" // String |  (facultatif)
let sso = "sso_example" // String |  (facultatif)

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