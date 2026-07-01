## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetTicketsOptions | No |  |

## Відповідь

Возвращает: [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Приклад

[inline-code-attrs-start title = 'Пример getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketsOpt, httpResp) = client.getTickets(tenantId = "my-tenant-123", options = GetTicketsOptions())
if ticketsOpt.isSome:
  let tickets = ticketsOpt.get()
  # использовать билеты по мере необходимости
[inline-code-end]