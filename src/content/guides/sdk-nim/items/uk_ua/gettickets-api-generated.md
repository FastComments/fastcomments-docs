## Параметри

| Ім'я | Тип | Обов'язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Yes |  |
| options | GetTicketsOptions | No |  |

## Відповідь

Повертає: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketsOpt, httpResp) = client.getTickets(tenantId = "my-tenant-123", options = GetTicketsOptions())
if ticketsOpt.isSome:
  let tickets = ticketsOpt.get()
  # використовуйте tickets за потребою
[inline-code-end]