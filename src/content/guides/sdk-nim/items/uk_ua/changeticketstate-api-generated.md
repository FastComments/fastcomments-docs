## Parameters

| Назва | Тип | Обов’язковий | Опис |
|------|------|---------------|------|
| tenantId | string | Так |  |
| userId | string | Ні |  |
| id | string | Ні |  |
| changeTicketStateBody | ChangeTicketStateBody | Ні |  |

## Response

Повертає: [`Option[ChangeTicketStateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state_response.nim)

## Приклад

[inline-code-attrs-start title = 'changeTicketState Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.changeTicketState(
  tenantId = "my-tenant-001",
  userId = "user-42",
  id = "ticket-12345",
  changeTicketStateBody = ChangeTicketStateBody(state = "closed")
)

if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]