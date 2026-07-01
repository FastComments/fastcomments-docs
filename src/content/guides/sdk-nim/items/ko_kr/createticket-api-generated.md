## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| createTicketBody | CreateTicketBody | No |  |

## 응답

반환: [`Option[CreateTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket_response.nim)

## 예시

[inline-code-attrs-start title = 'createTicket 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userId = "user-456"
let ticketBody = CreateTicketBody()
let (responseOpt, httpResponse) = client.createTicket(tenantId = tenantId, userId = userId, createTicketBody = ticketBody)
if responseOpt.isSome:
  let ticketResponse = responseOpt.get()
[inline-code-end]