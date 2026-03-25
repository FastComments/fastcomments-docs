## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | poizvedba | Da |  |
| userId | string | poizvedba | Da |  |

## Response

Vrača: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## Primer

[inline-code-attrs-start title = 'Primer createTicket'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji primeri kode so še beta. V primeru težav, prosimo prijavite na http://github.com/OpenAPITools/openapi-generator/issues/new
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