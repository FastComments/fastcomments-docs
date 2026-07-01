## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|-------------|------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`GetUserTrustFactorResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserTrustFactorResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getTrustFactor'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// O código de exemplo a seguir ainda está em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcional)
let sso = "sso_example" // String |  (opcional)

ModerationAPI.getTrustFactor(tenantId: tenantId, options: ModerationAPI.GetTrustFactorOptions(userId: userId, sso: sso)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]