## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| userId | string | query | Tak |  |

## Odpowiedź

Zwraca: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład createTicket'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Następujące przykłady kodu są nadal w wersji beta. W przypadku problemów zgłoś je przez http://github.com/OpenAPITools/openapi-generator/issues/new
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