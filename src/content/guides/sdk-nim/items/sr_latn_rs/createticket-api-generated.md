## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| createTicketBody | CreateTicketBody | Ne |  |

## Response

Vraća: [`Option[CreateTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket_response.nim)

## Primer

[inline-code-attrs-start title = 'createTicket Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userId = "user-456"
let ticketBody = CreateTicketBody()
let (responseOpt, httpResponse) = client.createTicket(tenantId = tenantId, userId = userId, createTicketBody = ticketBody)
if responseOpt.isSome:
  let ticketResponse = responseOpt.get()
[inline-code-end]