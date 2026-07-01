## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| options | GetTicketsOptions | Non |  |

## Réponse

Retourne : [`Option[GetTicketsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_tickets_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getTickets'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketsOpt, httpResp) = client.getTickets(tenantId = "my-tenant-123", options = GetTicketsOptions())
if ticketsOpt.isSome:
  let tickets = ticketsOpt.get()
  # utilisez les tickets au besoin
[inline-code-end]