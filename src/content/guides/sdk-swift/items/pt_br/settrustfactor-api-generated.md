## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Não |  |
| trustFactor | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`SetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/SetUserTrustFactorResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de setTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let userId = "userId_example" // String |  (opcional)
let trustFactor = "trustFactor_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

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

---