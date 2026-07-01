## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| userId | string = "" | Ne |  |

## Odgovor

Vraća: [`Option[GetTicketResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_ticket_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getTicket'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (ticketOpt, httpResp) = client.getTicket(tenantId = "my-tenant-123", id = "ticket-456", userId = "")
if ticketOpt.isSome:
  let ticket = ticketOpt.get()
  discard ticket
[inline-code-end]

---