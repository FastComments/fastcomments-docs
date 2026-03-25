## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 是 |  |

## 回應

回傳: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## 範例

[inline-code-attrs-start title = 'createTicket 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式碼範例仍為測試版。如有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
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