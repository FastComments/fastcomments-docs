## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |

## Response

Returns: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## Example

[inline-code-attrs-start title = 'createTicket Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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
