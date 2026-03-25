## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | query | Sim |  |
| userId | string | query | Sim |  |

## Resposta

Retorna: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de createTicket'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Os exemplos de código a seguir ainda estão em beta. Para qualquer problema, por favor reporte via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String | 
let createTicketBody = CreateTicketBody(subject: "subject_example") // CreateTicketBody | 

DefaultAPI.createTicket(tenantId: tenantId, userId: userId, createTicketBody: createTicketBody) { (response, error) in
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