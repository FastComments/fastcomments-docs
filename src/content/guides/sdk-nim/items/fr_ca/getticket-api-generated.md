## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| userId | string = "" | No |  |

## Réponse

Renvoie : [`Option[GetTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getTicket'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketOpt, httpResp) = client.getTicket(tenantId = "my-tenant-123", id = "ticket-456", userId = "")
if ticketOpt.isSome:
  let ticket = ticketOpt.get()
  discard ticket
[inline-code-end]