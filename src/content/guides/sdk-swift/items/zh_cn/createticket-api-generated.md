## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| userId | string | query | 是 |  |

## 响应

返回: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateTicket200Response.swift)

## 示例

[inline-code-attrs-start title = 'createTicket 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍为测试版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
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