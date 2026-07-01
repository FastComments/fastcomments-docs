## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| userId | string | No |  |
| createTicketBody | CreateTicketBody | No |  |

## Ответ

Возвращает: [`Option[CreateTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример createTicket'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userId = "user-456"
let ticketBody = CreateTicketBody()
let (responseOpt, httpResponse) = client.createTicket(tenantId = tenantId, userId = userId, createTicketBody = ticketBody)
if responseOpt.isSome:
  let ticketResponse = responseOpt.get()
[inline-code-end]