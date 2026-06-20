## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| id | string | Non |  |
| changeTicketStateBody | ChangeTicketStateBody | Non |  |

## Response

Retourne : [`Option[ChangeTicketStateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_change_ticket_state_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple de changeTicketState'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = ChangeTicketStateBody()
let (response, httpResponse) = client.changeTicketState(tenantId = "my-tenant-123", userId = "user-456", id = "ticket-789", changeTicketStateBody = body)
if response.isSome:
  let ticketResp = response.get()
  echo "Ticket state changed:", ticketResp
[inline-code-end]

---