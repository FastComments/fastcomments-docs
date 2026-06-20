## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| userId | string | Nie |  |
| id | string | Nie |  |
| changeTicketStateBody | ChangeTicketStateBody | Nie |  |

## Odpowiedź

Zwraca: [`Option[ChangeTicketStateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład changeTicketState'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = ChangeTicketStateBody()
let (response, httpResponse) = client.changeTicketState(tenantId = "my-tenant-123", userId = "user-456", id = "ticket-789", changeTicketStateBody = body)
if response.isSome:
  let ticketResp = response.get()
  echo "Ticket state changed:", ticketResp
[inline-code-end]

---