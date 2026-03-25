## Parametri

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Да |  |

## Одговор

Враћа: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## Примјер

[inline-code-attrs-start title = 'createTicket Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sljedeći primjeri koda su i dalje u beta fazi. За било који проблем, пријавите га путем http://github.com/OpenAPITools/openapi-generator/issues/new
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