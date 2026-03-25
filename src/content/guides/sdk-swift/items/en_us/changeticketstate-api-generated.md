## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Returns: [`ChangeTicketState200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ChangeTicketState200Response.swift)

## Example

[inline-code-attrs-start title = 'changeTicketState Example'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let userId = "userId_example" // String | 
let id = "id_example" // String | 
let changeTicketStateBody = ChangeTicketStateBody(state: 123) // ChangeTicketStateBody | 

DefaultAPI.changeTicketState(tenantId: tenantId, userId: userId, id: id, changeTicketStateBody: changeTicketStateBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]
