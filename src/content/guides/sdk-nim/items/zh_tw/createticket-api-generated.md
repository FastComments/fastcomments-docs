## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| userId | string | 否 |  |
| createTicketBody | CreateTicketBody | 否 |  |

## 回應

返回: [`Option[CreateTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket_response.nim)

## 範例

[inline-code-attrs-start title = 'createTicket 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userId = "user-456"
let ticketBody = CreateTicketBody()
let (responseOpt, httpResponse) = client.createTicket(tenantId = tenantId, userId = userId, createTicketBody = ticketBody)
if responseOpt.isSome:
  let ticketResponse = responseOpt.get()
[inline-code-end]

---