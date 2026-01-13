## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| badgeId | string | query | Não |  |
| type | number | query | Não |  |
| displayedOnComments | boolean | query | Não |  |
| limit | number | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetUserBadges200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getUserBadges'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcional)
let badgeId = "badgeId_example" // String |  (opcional)
let type = 987 // Double |  (opcional)
let displayedOnComments = true // Bool |  (opcional)
let limit = 987 // Double |  (opcional)
let skip = 987 // Double |  (opcional)

DefaultAPI.getUserBadges(tenantId: tenantId, userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]