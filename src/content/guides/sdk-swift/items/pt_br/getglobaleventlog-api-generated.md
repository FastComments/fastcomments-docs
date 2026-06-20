req
tenantId
urlId
userIdWS

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim |  |
| userIdWS | string | query | Sim |  |
| startTime | integer | query | Sim |  |
| endTime | integer | query | Não |  |

## Resposta

Retorna: [`GetEventLogResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetEventLogResponse.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getGlobalEventLog'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let userIdWS = "userIdWS_example" // String | 
let startTime = 987 // Int64 | 
let endTime = 987 // Int64 |  (opcional)

PublicAPI.getGlobalEventLog(tenantId: tenantId, urlId: urlId, userIdWS: userIdWS, startTime: startTime, endTime: endTime) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]