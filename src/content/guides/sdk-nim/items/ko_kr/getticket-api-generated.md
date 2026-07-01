## Parameters

| 이름 | 유형 | Required | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string = "" | No |  |

## Response

반환: [`Option[GetTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket_response.nim)

## Example

[inline-code-attrs-start title = 'getTicket 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketOpt, httpResp) = client.getTicket(tenantId = "my-tenant-123", id = "ticket-456", userId = "")
if ticketOpt.isSome:
  let ticket = ticketOpt.get()
  discard ticket
[inline-code-end]