## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Ne |  |
| id | string | Ne |  |
| changeTicketStateBody | ChangeTicketStateBody | Ne |  |

## Odgovor

Vrne: [`Option[ChangeTicketStateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer changeTicketState'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = ChangeTicketStateBody()
let (response, httpResponse) = client.changeTicketState(tenantId = "my-tenant-123", userId = "user-456", id = "ticket-789", changeTicketStateBody = body)
if response.isSome:
  let ticketResp = response.get()
  echo "Ticket state changed:", ticketResp
[inline-code-end]

---