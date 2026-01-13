## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| userId | string | query | Não |  |
| urlId | string | query | Não |  |
| fromCommentId | string | query | Não |  |
| viewed | boolean | query | Não |  |
| type | string | query | Não |  |

## Resposta

Retorna: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetNotificationCount200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getNotificationCount'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String |  (opcional)
let urlId = "urlId_example" // String |  (opcional)
let fromCommentId = "fromCommentId_example" // String |  (opcional)
let viewed = true // Bool |  (opcional)
let type = "type_example" // String |  (opcional)

DefaultAPI.getNotificationCount(tenantId: tenantId, userId: userId, urlId: urlId, fromCommentId: fromCommentId, viewed: viewed, type: type) { (response, error) in
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