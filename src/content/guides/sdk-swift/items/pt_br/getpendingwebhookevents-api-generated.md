## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| commentId | string | query | Não |  |
| externalId | string | query | Não |  |
| eventType | string | query | Não |  |
| type | string | query | Não |  |
| domain | string | query | Não |  |
| attemptCountGT | number | query | Não |  |
| skip | number | query | Não |  |

## Resposta

Retorna: [`GetPendingWebhookEvents200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetPendingWebhookEvents200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getPendingWebhookEvents'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let commentId = "commentId_example" // String |  (opcional)
let externalId = "externalId_example" // String |  (opcional)
let eventType = "eventType_example" // String |  (opcional)
let type = "type_example" // String |  (opcional)
let domain = "domain_example" // String |  (opcional)
let attemptCountGT = 987 // Double |  (opcional)
let skip = 987 // Double |  (opcional)

DefaultAPI.getPendingWebhookEvents(tenantId: tenantId, commentId: commentId, externalId: externalId, eventType: eventType, type: type, domain: domain, attemptCountGT: attemptCountGT, skip: skip) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]