## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|-------------|
| tenantId | string | Да |  |
| userId | string | Не |  |
| id | string | Не |  |
| changeTicketStateBody | ChangeTicketStateBody | Не |  |

## Отговор

Връща: [`Option[ChangeTicketStateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за changeTicketState'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = ChangeTicketStateBody()
let (response, httpResponse) = client.changeTicketState(tenantId = "my-tenant-123", userId = "user-456", id = "ticket-789", changeTicketStateBody = body)
if response.isSome:
  let ticketResp = response.get()
  echo "Ticket state changed:", ticketResp
[inline-code-end]

---