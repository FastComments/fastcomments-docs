## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| userId | string | Non |  |
| createTicketBody | CreateTicketBody | Non |  |

## Réponse

Renvoie : [`Option[CreateTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_ticket_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple createTicket'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let userId = "user-456"
let ticketBody = CreateTicketBody()
let (responseOpt, httpResponse) = client.createTicket(tenantId = tenantId, userId = userId, createTicketBody = ticketBody)
if responseOpt.isSome:
  let ticketResponse = responseOpt.get()
[inline-code-end]