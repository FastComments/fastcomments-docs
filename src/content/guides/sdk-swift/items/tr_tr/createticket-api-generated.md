## Parametreler

| Ad | Tip | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| userId | string | query | Evet |  |

## Yanıt

Dönen değer: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## Örnek

[inline-code-attrs-start title = 'createTicket Örneği'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Aşağıdaki kod örnekleri hâlâ beta aşamasındadır. Herhangi bir sorun için lütfen http://github.com/OpenAPITools/openapi-generator/issues/new adresinden bildirin
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